# Diagramcraft: working notes for Claude

Read AGENTS.md first. Its asset, clean-room, never-crash, architecture and gate rules apply.

## Start every session

1. Read plan/STATUS.md, plan/execution-plan.md and log/devlog.md.
2. Run cargo xtask ci before editing.

## Habits

Every authoring action is an engine command with tests. Inspect rendered outputs when rendering
exists. Commit landed work as M<phase>.<n>: what and push to origin master. Keep the roadmap,
progress docs and STATUS current. Record decisions instead of asking mid-brief. Never write to
craft-apps. End sessions by copying STATUS and devlog to corpus/_state/.
