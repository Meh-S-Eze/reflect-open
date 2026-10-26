//! File coordination for graph writes. On macOS and iOS a write that swaps a
//! staged file into place runs inside an `NSFileCoordinator` replacing claim,
//! so file-sync daemons (iCloud Drive and File Provider clients) see one
//! ordered save of the item instead of a file replaced behind their back.
//! Elsewhere the write runs directly.

use std::path::Path;
use std::time::Duration;

use reflect_types::AppResult;

/// Longest wait for other coordinators (a sync daemon applying a download, for
/// example) before the write fails instead of blocking the save forever.
const TIMEOUT: Duration = Duration::from_secs(10);

/// Run `write` inside a coordinated replacing write of `target`. `write`
/// receives the path to write, which the coordinator may hand back changed
/// when another process moved the item.
pub(crate) fn coordinated_replace<T>(
    target: &Path,
    write: impl FnOnce(&Path) -> AppResult<T>,
) -> AppResult<T> {
    coordinated_replace_within(target, TIMEOUT, write)
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn coordinated_replace_within<T>(
    target: &Path,
    _timeout: Duration,
    write: impl FnOnce(&Path) -> AppResult<T>,
) -> AppResult<T> {
    write(target)
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn coordinated_replace_within<T>(
    target: &Path,
    timeout: Duration,
    write: impl FnOnce(&Path) -> AppResult<T>,
) -> AppResult<T> {
    use std::cell::Cell;
    use std::ptr::NonNull;
    use std::sync::mpsc;

    use block2::RcBlock;
    use objc2::rc::Retained;
    use objc2_foundation::{NSFileCoordinator, NSFileCoordinatorWritingOptions, NSURL};
    use reflect_types::AppError;

    /// The coordinator, moved to the watchdog thread only to call `cancel`.
    struct CancelHandle(Retained<NSFileCoordinator>);
    // SAFETY: Apple documents `-[NSFileCoordinator cancel]` as callable from
    // any thread, and the watchdog calls nothing else on it.
    unsafe impl Send for CancelHandle {}
    impl CancelHandle {
        fn cancel(&self) {
            self.0.cancel();
        }
    }

    let url = NSURL::from_file_path(target)
        .ok_or_else(|| AppError::io(format!("not a file path: {}", target.display())))?;
    let coordinator = NSFileCoordinator::new();

    let (done, wait) = mpsc::channel::<()>();
    let handle = CancelHandle(coordinator.clone());
    let watchdog = std::thread::spawn(move || {
        if wait.recv_timeout(timeout) == Err(mpsc::RecvTimeoutError::Timeout) {
            handle.cancel();
        }
    });

    let write = Cell::new(Some(write));
    let result = Cell::new(None);
    let accessor = RcBlock::new(|url: NonNull<NSURL>| {
        // SAFETY: the coordinator passes a valid URL for the block's duration.
        let path = unsafe { url.as_ref() }.to_file_path();
        if let Some(write) = write.take() {
            result.set(Some(write(path.as_deref().unwrap_or(target))));
        }
    });
    let mut error = None;
    coordinator.coordinateWritingItemAtURL_options_error_byAccessor(
        &url,
        NSFileCoordinatorWritingOptions::ForReplacing,
        Some(&mut error),
        &accessor,
    );
    drop(done);
    let _ = watchdog.join();

    match (result.take(), error) {
        (Some(result), _) => result,
        (None, Some(error)) => Err(AppError::io(format!(
            "file coordination failed for {}: {}",
            target.display(),
            error.localizedDescription()
        ))),
        (None, None) => Err(AppError::io(format!(
            "file coordination skipped the write for {}",
            target.display()
        ))),
    }
}

#[cfg(all(test, any(target_os = "macos", target_os = "ios")))]
mod tests {
    use super::*;
    use std::fs;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, mpsc};

    use block2::RcBlock;
    use objc2_foundation::{NSFileCoordinator, NSFileCoordinatorReadingOptions, NSURL};

    /// Hold a coordinated read of `path` on another thread until `release`
    /// fires. Returns once the read is held.
    fn hold_read(path: &Path, release: mpsc::Receiver<()>) -> std::thread::JoinHandle<()> {
        let path = path.to_path_buf();
        let (held, wait_held) = mpsc::channel();
        let reader = std::thread::spawn(move || {
            let url = NSURL::from_file_path(&path).unwrap();
            let accessor = RcBlock::new(|_: NonNull<NSURL>| {
                held.send(()).unwrap();
                let _ = release.recv();
            });
            NSFileCoordinator::new().coordinateReadingItemAtURL_options_error_byAccessor(
                &url,
                NSFileCoordinatorReadingOptions::empty(),
                None,
                &accessor,
            );
        });
        wait_held.recv().unwrap();
        reader
    }

    #[test]
    fn the_write_runs_on_the_target() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("note.md");
        let written = coordinated_replace(&target, |path| {
            fs::write(path, "# Note\n")?;
            Ok(path.to_path_buf())
        })
        .unwrap();
        assert_eq!(written, target);
        assert_eq!(fs::read_to_string(&target).unwrap(), "# Note\n");
    }

    #[test]
    fn the_write_waits_for_a_coordinated_reader() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("note.md");
        fs::write(&target, "before\n").unwrap();
        let (release, released) = mpsc::channel();
        let reader = hold_read(&target, released);

        // The write reports whether the reader had already let go when it ran.
        let reader_done = Arc::new(AtomicBool::new(false));
        let (started, wait_started) = mpsc::channel();
        let writer = std::thread::spawn({
            let target = target.clone();
            let reader_done = Arc::clone(&reader_done);
            move || {
                started.send(()).unwrap();
                coordinated_replace(&target, |path| {
                    let ran_after_reader = reader_done.load(Ordering::SeqCst);
                    fs::write(path, "after\n")?;
                    Ok(ran_after_reader)
                })
            }
        });
        wait_started.recv().unwrap();
        std::thread::sleep(Duration::from_millis(200));

        reader_done.store(true, Ordering::SeqCst);
        release.send(()).unwrap();
        assert!(writer.join().unwrap().unwrap());
        reader.join().unwrap();
        assert_eq!(fs::read_to_string(&target).unwrap(), "after\n");
    }

    #[test]
    fn the_write_fails_when_the_wait_times_out() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("note.md");
        fs::write(&target, "before\n").unwrap();
        let (release, released) = mpsc::channel();
        let reader = hold_read(&target, released);

        let result = coordinated_replace_within(&target, Duration::from_millis(100), |path| {
            Ok(fs::write(path, "after\n")?)
        });

        release.send(()).unwrap();
        reader.join().unwrap();
        assert!(result.is_err());
        assert_eq!(fs::read_to_string(&target).unwrap(), "before\n");
    }
}
