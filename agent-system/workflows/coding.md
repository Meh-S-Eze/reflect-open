# Coding Workflow

## Establish the change

Read current instructions, HANDOFF.md, live Git state, and the smallest relevant
source chain. Before code edits, name the source-of-truth owner, the path being
replaced or demoted, and observable evidence that will prove the behavior changed.
Trace the trigger through navigation/services/adapters to the final output.

For a nontrivial change, record a short plan with acceptance scenarios, affected
owners, validation, and rollback. Convert user corrections into concrete
acceptance/regression scenarios. Mark ideas as proposals until accepted. Obtain
the upstream-required sign-off for broad UX, migrations, public APIs, and release
behavior; do independent investigation while a decision is pending.

For consequential or unresolved choices, record the options, selected behavior,
reason, decision owner, and remaining uncertainty in the existing plan or handoff.
Use a concrete example or small mockup when it clarifies the choice. Update the
plan when findings contradict it. Routine implementation choices do not require
a separate decision ledger or a new approval gate.

Make acceptance observable: name the trigger, expected output, important failure
cases, and evidence that could disprove success. A proxy score or passing helper
test cannot establish usability or native behavior it does not exercise.

## Implement in the owning layer

- Reuse [architecture conventions](../../docs/plans/architecture-conventions.md),
  [service guidance](../../docs/contributing/services.md), and the
  [design system](../../design-system/readme.md).
- `packages/core` owns product rules and platform interfaces; `packages/app`
  owns application orchestration; desktop React owns presentation and Tauri
  adapters. Rust owns native capabilities. Use explicit service dependencies.
- Inspect the existing implementation before creating another abstraction.
  Replace obsolete routes deliberately rather than maintaining competing owners.
- Preserve daily-note-first behavior, Markdown portability, keyboard access,
  `private: true` enforcement, direct provider calls, and keychain secrets.
- Validate external inputs at the boundary. Respect generation checks, graph
  locks, and own-write reconciliation. Read
  [sync safety](../../docs/git-backup-safety.md) before modifying sync or graph IO.
- For editor behavior, investigate Meowdown ownership rather than hiding a
  dependency defect in Reflect. Its upstream guide's `~/repos/meowdown` path is
  a maintainer example; verify a local checkout exists before relying on it.

## Validate and review

Use [the validation workflow](validation.md). Tests should assert user-visible
behavior and important invariants, not mirror implementation details. Add
regression coverage for meaningful bugs; avoid ceremonial tests for simple prose.

Review the final diff for correctness, private-note leakage, durable data loss,
keyboard/focus behavior, stale async effects, unintended dependency changes,
and upstream compatibility. An independent reviewer is useful when authorized;
if none ran, report a self-review honestly. Do not claim human approval.

For nontrivial work, review the implementation's decisions against the accepted
scope as well as reviewing the code. Turn material new failures and repeated
corrections into the scoped [learning loop](learning.md), rather than appending
generic instructions after every task.

## Finish the authorized outcome

Update durable rationale and HANDOFF.md. Publish within the authorized destination
using [the contribution workflow](contributions.md). Report what changed, why,
what passed, what remains unverified, and where the user can inspect the result.
Reuse valid evidence until new edits or changed assumptions invalidate it.
Park an isolated blocker with its evidence while completing independent work.
