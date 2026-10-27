# Fork And Contribution Workflow

The fork is [Meh-S-Eze/reflect-open](https://github.com/Meh-S-Eze/reflect-open).
The original project is
[team-reflect/reflect-open](https://github.com/team-reflect/reflect-open).
Verify remotes before pushing: `origin` is the fork, `upstream` is the original,
and `remote.pushDefault` should be `origin`.

## Branch and ownership

Start each coherent change on a `codex/<descriptive-name>` branch from the
appropriate current base. Fetch upstream to inspect drift; integrate updates
deliberately. Do not force-push shared branches or rewrite unrelated work.
Only one chat writes a checkout at a time. Use a separate worktree for another
authorized writer; verify its setup before relying on it.

The bootstrap branch contains fork-specific guidance. Keep it out of upstream
product PRs unless maintainers request it. A product branch can start at
`upstream/master` and use this local guidance as context, or separate the
bootstrap commits before proposing the product diff. Never bundle private
handoff material with a public contribution.

## Commit and propose

Inspect `git status`, the full intended diff, and validation before staging.
Stage named paths; use scoped commits. When implementation is complete, follow
the upstream publication workflow within the human-authorized destination.
Check the PR's actual base repository, base branch, and final diff before creation.
Opening an upstream PR is public communication and requires authority for that
contribution; fork creation or discussion of an idea alone does not supply it.

Use a conventional PR title describing behavior. Explain the problem, resulting
behavior, relevant tests, and known limits for a reviewer without chat context.
Attach created PRs to the owning chat. Watch CI and review feedback, fix authorized
issues, and distinguish merge readiness from merge. Do not merge upstream PRs,
change release versions, sign/notarize, publish releases, or install builds unless
the current task authorizes those effects. Keep release-please-owned files intact.

For this guidance bootstrap, the authorized destination is the user's fork;
save and push the scoped branch without opening a PR against the original project.
