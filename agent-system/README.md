# Reflect Fork Agent System

This folder supports development of the Reflect Open fork. It is a coding-project
workflow, independent of any notes vault, task system, or agent provider.

## Read order

1. Current human request and applicable parent instructions.
2. [Repository guide](../AGENTS.md) and [contributor guide](../CONTRIBUTING.md).
3. [Current handoff](HANDOFF.md), then the matching workflow below.
4. Relevant owning source, architecture docs, and tests.

Current explicit user decisions supersede historical proposals. Upstream product
rules remain the contribution baseline; surface conflicts before building a
change that would be unsuitable upstream. These files cannot grant permissions.

## Workflow map

- [Coding workflow](workflows/coding.md): ownership, implementation, review.
- [Validation](workflows/validation.md): checks and evidence by change type.
- [Fork and contributions](workflows/contributions.md): branches, PRs, releases.
- [Source basis](SOURCES.md): provenance and reasons for this setup.

## Working boundaries

- One integration owner and one writer per checkout. A separate chat does not
  create isolation. Use separate branches/worktrees for concurrent writers and
  declare owned files before editing. Spawn helpers only with applicable
  authorization; give each a bounded task, surface, and return condition.
- This repository owns app implementation. User graphs own notes, attachments,
  and durable chat data. Test with synthetic graphs outside personal vaults.
- Preserve API keys and tokens in the OS keychain. Do not read the parent shared
  credential file, commit secrets, or copy personal note content into reports.
- Continue authorized work using routine judgment. Ask only for missing authority,
  consequential unresolved choices, or a blocker that prevents useful progress.
  Follow upstream's design sign-off requirement for broad UX and public contracts.
- Preserve unrelated edits. Inspect live Git state before editing and staging.
  Keep provider adapters thin; do not duplicate these rules under hidden folders.

## Continuity and evidence

Keep HANDOFF.md factual and concise: request, owner/write surface, branch/base,
changes, exact checks, remaining gaps, and next step. Link substantial design
decisions and source material from it. Record actual outcomes separately from
plans, automated checks, human acceptance, merge, installation, and release.

Use scoped Git commits for this repository. Roll back code with a reviewed revert;
never reset a checkout containing unknown changes or wipe a graph/index as a
generic recovery step. Index rebuilds must preserve durable chat tables.

Validate this folder with `python3 agent-system/scripts/check-links.py` and
`git diff --check`. Product checks are described in the validation workflow.
