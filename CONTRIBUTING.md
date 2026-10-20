# Contributing

Thank you for looking at MisterDiagram. Pull requests are welcome, and the first ones were
merged within a day of review. A few rules keep the project honest and keep review fast.

## Test prerequisites

Use Rust 1.92 or newer and Python 3.11 or newer for the full test suite.
Without a Python 3 executable (`python3` or `python` on PATH), the Python package
comparisons in `crates/engine/tests/create_style.rs` and `crates/mcp/tests/tools.rs`
print a reason and skip the Python-dependent checks. Other Rust tests continue. `MISTERDIAGRAM_REQUIRE_PYTHON=1` turns a missing interpreter into a test
failure; `cargo xtask ci` and CI set it automatically. Private-corpus tests have
separate prerequisites described in `docs/development.md`.

## Clean room, no exceptions

- Test inputs are contributor-authored: synthetic XML strings, in-memory packages built with
  the test helpers, or drawings produced by this repo's own scripts. Never a Microsoft stencil,
  template, icon or font, and never a file saved by Visio.
- Visio is observed only as a black box, on drawings our scripts made, in a licensed copy.
- Formats and functions come from Microsoft's published references. Every element, cell and
  function cites its reference page in its doc comment; `cargo xtask sources` regenerates
  docs/sources.md and CI checks it.
- If you consult any non-Microsoft documentation about Visio or its formats, add a line to
  docs/provenance.md section 3 in the same commit. Never open the source of libvisio or any
  other Visio implementation while working here.
- A PR without the private oracle corpus can claim reference coverage. Measured parity
  against Visio is confirmed by the maintainer, who runs the corpus. Say which one you claim.

## Shape of a change

- One issue per PR, one PR at a time, small enough to review in one sitting.
- Plain commit subjects. The `M<phase>.<n>:` prefix is reserved for milestone commits by the
  maintainer.
- Gaps and follow-ups go in docs/gaps.md; decisions in plan/STATUS.md (gitignored, so
  summarise in the PR). Do not add a docs/specs tree.
- Never edit an oracle output, a threshold, a floor or a denominator to make a test pass.
- `cargo xtask ci` green, with the hostile-input tests extended for every new parser path.
- The maintainer may rebase your branch onto master before merging; authorship is kept.

## Where things are

AGENTS.md has the rules every agent and contributor works under, docs/architecture.md the
layers, docs/development.md the build gates and the oracle VM workflow, ROADMAP.md the
milestones and the numbers each one has to reach.

## Performance pull requests

Include `tools/bench` numbers before and after on the same machine with nothing
else running. Build both revisions first, then measure all five workloads and
attach the JSONL totals. With Python 3.11 or newer, from a clean checkout of your proposed change:

```sh
git fetch origin master
python3 tools/bench/run.py --baseline origin/master --candidate HEAD
```

The command creates disposable checkouts below `target/performance-pr`, builds
both harnesses using each revision's locked dependency versions, runs the same strict commands and calls
`python3 tools/bench/compare.py BEFORE.vsdx AFTER.vsdx` for each saved package.
Every uncompressed part must be byte-identical to master. Only fresh master
GUIDs and docProps save times are masked. Use a new `--output target/NAME` for
another run. The harness records wall time, command count and shape count.

See [#39](https://github.com/MisterDiagram/MisterDiagram/issues/39) and the
[measured cost breakdown](docs/gaps.md#m50--39-strict-authoring-cost-and-capacity-2026-10-17)
for where time goes today. Deferred-publication batch mode remains shelved until
saving is independent of command history. MCP batch preserves every strict
write/import boundary and only groups rollback and undo.

## Test builds

Testers can download artifacts from an Actions run page (GitHub login required); they are untested builds of that commit.
macOS may require clearing the downloaded binary's quarantine attribute with `xattr -d com.apple.quarantine misterdiagram`.
