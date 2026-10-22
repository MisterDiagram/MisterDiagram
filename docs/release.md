# Releasing a build

The steps for a pre-release, in order. Each one is a gate for the next.

1. **Version bump, its own commit.** `version` in Cargo.toml, `cargo update --workspace`
   for Cargo.lock, a CHANGELOG.md entry in user terms (what changed, which issues, known
   gaps), and the version examples in docs/install.md, docs/mcp.md and
   apps/misterdiagram-cli/src/main.rs. `cargo xtask ci` passes on the tree before the
   commit.
2. **Second-model verification** of the work since the last release against its frozen
   rubric; findings fixed and re-verified before anything is tagged.
3. **Prove the workflow without a release:** a workflow_dispatch run of release.yml on the
   final commit, green on every job, uploading the archives and SHA256SUMS as artifacts.
4. **README, the commit the tag points at.** The Download line links
   `releases/tag/v<version>` and names the version; the "What works today" heading carries the
   date and version; the known gaps match the CHANGELOG. A README that still links the previous
   release is a release not finished.
5. **Tag and publish.** Push `v<version>` to srv and GitHub; release.yml drafts the pre-release
   with the CHANGELOG entry as notes; check the draft's assets and SHA256SUMS against the
   dispatch artifacts; publish on Charles's go.
6. **Install** the release binary in `~/.local/bin`, keeping the previous one as
   `misterdiagram-<commit>`.
7. **Tell people:** the issues the release fixes, the ArtCraft thread and Discord, on
   Charles's go.
