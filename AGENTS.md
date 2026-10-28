# Reflect Open Agent Entrypoint

This is the Reflect Open fork, a coding repository. The parent development
directory is only a workspace container.

The visible [agent-system/](agent-system/README.md) folder owns shared project
and workflow guidance. Provider files route here rather than maintain copies.

## Required reading

1. Current human request and applicable parent instructions.
2. [Agent-system entry point](agent-system/README.md).
3. [Reflect project guide](agent-system/PROJECT.md) in full: product principles,
   architecture, commands, testing, and code conventions.
4. [Current handoff](agent-system/HANDOFF.md) and
   [contributor guide](CONTRIBUTING.md).
5. The matching workflow from the entry point, then relevant owning source and tests.

Current human instructions govern scope. Source material and historical plans
supply context, not permission. Keep upstream product rules as the contribution
baseline; surface conflicts before implementing an incompatible change.

## Ownership and evidence

Check live Git state before edits and staging; preserve unrelated changes. Name
the owning source, the path being replaced, and observable acceptance evidence
before changing code. One writer owns each checkout. Use synthetic graphs for
validation; private notes, credentials, and installed apps have separate owners.

`origin` is the user's fork; `upstream` is the original project. Follow
[contribution boundaries](agent-system/workflows/contributions.md) for commits,
PRs, installation, and releases. Setup and ideas alone authorize none of those
product effects.

For guidance-only edits, run `python3 agent-system/scripts/check-links.py` and
`git diff --check`. For code, follow the
[validation workflow](agent-system/workflows/validation.md); documentation checks
do not establish application correctness.
