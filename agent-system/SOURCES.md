# Source Basis

## Project authority

- [AGENTS.md](../AGENTS.md): thin entry point into this folder.
- [PROJECT.md](PROJECT.md): Reflect's original agent guide, including product
  principles, architecture, commands, native sidecars, and code conventions.
- [CONTRIBUTING.md](../CONTRIBUTING.md): setup and contribution expectations.
- [Architecture](../docs/plans/architecture-conventions.md),
  [services](../docs/contributing/services.md),
  [testing](../docs/contributing/testing.md), and
  [sync safety](../docs/git-backup-safety.md): owning technical guidance.
- [Package scripts](../package.json),
  [development overlay](../apps/desktop/src-tauri/tauri.dev.conf.json), and
  [CI](../.github/workflows/ci.yml): executable definitions, not prose promises.

## User decisions and established practices

On October 27, 2026, the user requested a personal fork of Reflect Open, then a
visible `agent-system/` folder grounded in established coding practices before
opening a dedicated development chat. Fork and local clone were created from
upstream commit `3fee90fe3105f5d7f3b7d641a1dc59bf436c4b60`.

This setup carries forward the user's established practices: current UX decisions
over historical plans; corrections as acceptance checks; one writer and integration
owner; source-backed design; bounded authorized workers; scoped commits; meaningful
validation; factual handoffs; and separate claims for tests, human acceptance,
deployment, and release. Local examples inspected during bootstrap were the
CR Journal and Context Sidebar project guides. Their product-specific deployment,
database, and vault rules were not transferred here.

The project-kit bootstrap classifier identified this checkout as a coding project,
not an Obsidian vault or integration owner. The bootstrap and rule-authoring
guidance informed the structure; the user explicitly chose a visible folder.
The initial bootstrap preserved upstream instructions and provider setup files.

## October 27 guidance consolidation

The user then requested a Zettelkasten best-practice check and clarified that
this folder should guide the project, merge useful existing agent guidance, and
be reached from the root agent entrypoints.

The full upstream guide was relocated from `AGENTS.md` at fork commit
`1ec574e7f9128971c8b13741e7c327de1ec781d3` to [PROJECT.md](PROJECT.md), preserving
its content with relative-link adjustments. Root `AGENTS.md` and `CLAUDE.md` now
route through this folder. Executable provider setup and hooks were left intact.

The [Zettelkasten source basis](references/zettelkasten-practices.md) records
the exact local research notes, evidence grades, adaptations, and transfer
limits. Shared vault framework and contract files were consulted to respect
read/privacy boundaries, not imported as repository authority. Useful practices
were integrated into existing workflows rather than copied into a second rules
tree. The rule-authoring guidance informed concise routing; its provider-specific
hidden-rule layout was not used because the user chose a visible shared folder.

These breadcrumbs explain the setup. They do not authorize unrelated work or
turn historical plans into current product requirements.
