"""Check repository-local file links in the fork's agent guidance."""

import re
from pathlib import Path
from urllib.parse import unquote, urlsplit


def main() -> None:
    root = Path(__file__).resolve().parents[2]
    documents = [root / "AGENTS.md", *sorted((root / "agent-system").rglob("*.md"))]
    errors = []
    count = 0
    for document in documents:
        content = document.read_text(encoding="utf-8")
        for match in re.finditer(r"(?<!!)\[[^\]\n]+\]\(([^)\n]+)\)", content):
            target = match.group(1).strip().strip("<>")
            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc or not parsed.path:
                continue
            path = (document.parent / unquote(parsed.path)).resolve()
            count += 1
            if not path.is_relative_to(root) or not path.exists():
                line = content[: match.start()].count("\n") + 1
                errors.append(f"{document.relative_to(root)}:{line}: {target}")
    if errors:
        raise SystemExit("Broken or out-of-repository links:\n" + "\n".join(errors))
    print(f"Checked {count} local file links in {len(documents)} guidance documents.")
    print("File existence only; heading anchors and remote URLs are not checked.")


if __name__ == "__main__":
    main()
