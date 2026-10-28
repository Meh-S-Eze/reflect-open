# Reflect Fork Development Handoff

## Current request and ownership

The user requested a personal Reflect Open fork, a visible coding-focused
`agent-system/` folder, and this dedicated development chat. The setup chat has
finished and yielded ownership. This development chat is the integration owner
and sole writer for the checkout below.

On October 27, the user asked to check the Zettelkasten vault for further
best practices, then clarified that this folder should guide the project, merge
useful Reflect agent guidance, and be reached from the agent entrypoints.
This task changes guidance only. No product feature is authorized.

## Source state and write surface

- Checkout: `/Users/whatamehs/000-Development/reflect-open`.
- Fork: <https://github.com/Meh-S-Eze/reflect-open> (`origin`, default push target).
- Original: <https://github.com/team-reflect/reflect-open> (`upstream`).
- Initial base: `3fee90fe3105f5d7f3b7d641a1dc59bf436c4b60`, upstream `master`.
- Branch: `codex/agent-system-bootstrap`.
- This task began clean at `1ec574e7f9128971c8b13741e7c327de1ec781d3`, verified
  against the live fork branch.
- Write surface: root `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`, and
  `agent-system/`. Product code, provider setup/hooks, and vault files are outside
  this change.

## Changes and decisions

- `agent-system/` is the shared guidance owner. `AGENTS.md` and `CLAUDE.md` are
  thin routing adapters; the contributor guide points to the relocated guide.
- [PROJECT.md](PROJECT.md) preserves the full upstream agent-guide body from
  the task's starting revision, with relative links adjusted for its new location.
  Moving the guide avoids a competing root rules copy while retaining Reflect's
  product principles, architecture, commands, tests, and conventions.
- Existing workflows now expose consequential choices and reasons, observable
  acceptance, protected handoff context, and evidence invalidation on input change.
- [Learning from failures](workflows/learning.md) connects concrete failures to
  the owning layer, regression evidence, and scoped workflow improvements.
- [Source basis](SOURCES.md) and the
  [Zettelkasten reference](references/zettelkasten-practices.md) retain provenance,
  source-quality limits, and exact local source identifiers. Vault task semantics,
  private source bodies, external skills, tools, and runtime changes were not
  imported. Candidate advice was adapted narrowly, not treated as proven policy.

## Product context and constraints

Reflect Open v0.16.0 for Apple silicon was installed separately at
`/Applications/Reflect.app` during setup. The source checkout is newer development
code, not that installed app. No custom build has been installed or run by this
chat.

A Roam-style sidebar containing independently editable pages remains a candidate
idea, not an accepted specification. Inspect current source and resolve broad UX
choices before implementation. Do not open an upstream PR, connect a personal
graph, enable paid AI services, replace the installed app, or publish a release
without the applicable user request.

## Validation and limits

The original bootstrap passed 42 local file-link checks, validator syntax parsing,
upstream-guide preservation, and diff whitespace checks. Those are historical
documentation results, not application evidence.

For this consolidation, the following passed against the final guidance change
based on `1ec574e7f9128971c8b13741e7c327de1ec781d3`:

- `python3 agent-system/scripts/check-links.py`: 79 local file links across
  12 guidance documents, including both root adapters and the contributor guide.
- Python AST parsing of the link validator.
- Exact comparison of the full original guide body with `PROJECT.md`, allowing
  only the deterministic relative-link relocation.
- Routing checks: both root agent entrypoints link to README, PROJECT, and
  HANDOFF under `agent-system/` and remain under 50 lines.
- Byte comparison confirming provider setup and hooks remain unchanged.
- `git diff --check` and a whitespace check including new guidance files.

These were local checks and self-review; no independent agent or human acceptance
was claimed. Link validation checks local file existence, not heading anchors,
remote URLs, or prose correctness.

App dependencies, typecheck, unit/browser/native tests, build, and live interactions
remain unverified for this checkout. Existing provider setup includes automatic
dependency installation behavior; read it before starting a new harness session.

## Next session

Read the root entry point and this folder's required guidance. Confirm live Git
state and ownership. Await the first concrete product request; the sidebar idea
is not authorization. For a meaningful implementation, preserve accepted scope,
failed attempts, important rejected options, the revision checked, and the next
useful step in the existing handoff or linked plan.
