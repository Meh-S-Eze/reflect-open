# Validation And Development

Commands run from the repository root. Read live package scripts and
[testing guidance](../../docs/contributing/testing.md) before using them; this
workflow does not replace their current definitions.

## Setup and safe previews

Use the pnpm version declared by `packageManager` in
[package.json](../../package.json). Install with `pnpm install --frozen-lockfile`.
Do not change lockfiles merely to make setup pass. Inspect unexpected install
failures before retrying with broader changes.

Use `pnpm dev` for frontend development. For native interaction, prefer
`pnpm tauri:dev`: the
[development overlay](../../apps/desktop/src-tauri/tauri.dev.conf.json) gives
Reflect Dev its own app identity and an inert updater endpoint. Use an isolated
synthetic graph even with this separate identity. Do not replace the installed
stable app or point a development build at personal notes without authorization.

## Match checks to risk

| Change | Required evidence |
| --- | --- |
| Agent guidance or prose only | Link validation, `git diff --check`, factual/source review; no app test claims |
| TypeScript application behavior | `pnpm check`, `pnpm build`, focused Vitest cases and relevant interaction checks |
| Desktop React behavior | Above plus relevant `.test.tsx` browser tests; verify keyboard, focus, navigation, sizing, and persisted state as applicable |
| Rust or native commands | Above where applicable; targeted crate tests and required sidecar staging |
| Sync, migrations, privacy, recovery | Targeted failure/recovery and preservation cases using synthetic data, plus relevant baseline checks |

Upstream test invocation is `pnpm test --run <actual-test-path>`. Node `.test.ts`
and browser `.test.tsx` projects have different environments. Install browser
runtimes with `pnpm --filter @reflect/desktop test:install` when needed. Test
WebKit for relevant desktop behavior with `REFLECT_TEST_BROWSER=webkit`.
Playwright is for software QA; use connectors/HTTP for ordinary research.

Before compiling the desktop Rust crate, stage sidecars with
`pnpm --filter @reflect/desktop sidecar`. Use targeted Rust checks such as
`cargo test -p reflect-open` or `cargo test -p reflect-cli` when relevant.
Do not repeatedly run all workspace tests without a reason.

## Evidence discipline

Record source revision or working diff, exact commands, pass/fail, meaningful
test counts, and environment limits. Distinguish browser preview from native
Tauri verification, synthetic checks from live use, and build from installation.
Missing dependencies or unavailable platforms are gaps, not passing checks.
After the appropriate checks pass, broaden testing only for an unresolved risk,
a new edit, or a failure. Never use a production graph to resolve a test gap.
