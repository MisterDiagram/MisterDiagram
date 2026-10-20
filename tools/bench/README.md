# Authoring cost benchmark

These contributor-original workloads measure issue #39. They are not CI timing
oracles. They contain no customer names, site names or vendor artwork.

Brief 28 adds `python3 tools/bench/large.py OUTPUT`:

- `san-large`: two invented fabrics, 312 top-level shapes and 68 group children
  (380 total), 196 connectors, 2,500 commands. Two director group instances each
  have 96 named ports. Sixty-four host groups contain an original 8 by 8 bitmap.
  Fabric outlines are plain rectangles. Static and dynamic glue both occur.
- `shapes-377`: 377 rectangles with fill, line, text and two populated Shape Data
  fields, no connectors, exactly 2,500 commands. `measure.py` runs ten distinct
  CLI invocations of 250 commands, reopening and saving the same file. Creation
  uses the existing composed fill/line parameters; their strict sub-edits remain.
- `large-edit`: open the resulting `shapes-377.vsdx`, then execute 20 commands
  moving and restyling shapes and adding, gluing and deleting two connectors.
  Open, final save and process shutdown are included in elapsed time.

`invented.vssx` is generated entirely from original geometry and procedural
bitmap pixels by `large.py`; it contains no vendor masters or artwork.
`PYTHONDONTWRITEBYTECODE=1 python3 tools/bench/test_large.py` verifies population,
group/bitmap structure, connection ports and command mix.
`PYTHONDONTWRITEBYTECODE=1 python3 tools/bench/test_measure.py` checks malformed,
overlong and over-count workload inputs against the harness's admission bounds.

`instrument-current.py CHECKOUT` adds the M5.0 stage clocks to a disposable
checkout of the current engine. It requires each insertion point exactly once
and refuses the working source tree. Build its CLI and `authoring-bench`
example in release mode, then run:

```sh
python3 tools/bench/measure.py --cli PATH_TO_CLI --harness PATH_TO_AUTHORING_BENCH \
  --workloads OUTPUT --output RESULTS
```

The original five workloads must also have been generated with `generate.py`.
Each receipt records the exact command, exit status and elapsed time; stderr
retains stage clocks and `/usr/bin/time -l` peak RSS. Every shapes-377 chunk's
saved package is retained for byte comparison. Profiles are separate runs,
using `--profile ~/.cargo/bin/samply`; their timing is not used for speed targets.
Run no competing builds or tests during either measurement. Keep baseline
packages outside this repository and compare with the unchanged `compare.py`.

`generate.py OUTPUT` writes five command lists: 25, 50 and 100 rectangles,
40 rectangles with one Shape Data row each, and 1,900 commands producing an
invented 192-port director, 58 connected ports, 46 endpoint symbols and
58 statically glued connectors (163 shapes total). The large workload uses
ShapePermeableX/Y as the documented workaround for the baseline obstacle cap.
That workaround does not test the separate obstacle-cap acceptance pages.
Each workload runs in one engine session. Counts in the timing output exclude
its final file.save command, which is included in total wall time.

To reproduce the strict baseline at 13897b0, from the repository root:

```sh
git worktree add --detach target/authoring-baseline 13897b0
python3 tools/bench/instrument.py target/authoring-baseline
python3 tools/bench/generate.py target/authoring-workloads
export CRAFT_FONTS_DIR=/Users/charles/Projects/craft-apps/craft-fonts/src
export CARGO_TARGET_DIR="$PWD/target/authoring-bench-build"
cargo build --release --manifest-path target/authoring-baseline/Cargo.toml \
  -p misterdiagram-engine --example authoring-bench --example roundtrip-probe
mkdir -p target/authoring-results
for case in rectangles-25 rectangles-50 rectangles-100 data-40 ports-1900; do
  target/authoring-bench-build/release/examples/authoring-bench \
    "target/authoring-workloads/$case.json" "$PWD/target/authoring-results/$case.vsdx" \
    > "target/authoring-results/before-$case.jsonl" \
    2> "target/authoring-results/before-$case-stages.jsonl"
done
python3 tools/bench/summarize.py target/authoring-results
```

The instrumentation changes only the disposable checkout. Its command limits,
routing, writer and import policy stay intact. JSONL stdout records each command,
each 250-command slice (plus the final partial slice) and total time including
save. JSONL stderr reports Session clone, GlueHost prepare, reroute, writer plus
theme setup, import, and separate edit_document copy-on-write time. Command
execution, validation, palette updates and measurement/output overhead are the
remaining wall time. These measurements are elapsed wall clocks, not CPU clocks;
record competing workloads when comparing runs.

`roundtrip-probe` prints literal caches and independently evaluated values before
and after the strict write/import boundary. It demonstrates dependent cache
refresh and RGB formula normalization with palette allocation, without modifying
an oracle or relaxing a test.

`python3 tools/bench/compare.py FIRST.vsdx SECOND.vsdx` compares every uncompressed
part byte for byte. It masks only master UniqueID/BaseID, core.xml created/modified
and the generated custom.xml TimeEdited property,
reports the first differing part/cell position, and exits 1 on any other change.
`PYTHONDONTWRITEBYTECODE=1 python3 tools/bench/test_compare.py` checks hostile ZIP
inputs and verifies that both changed cell values and changed serialization fail.

The shipped CLI remains strict. Deferred-publication batch mode is shelved pending
history-independent saving. MCP batch keeps each strict write/import boundary. The comparison helper preserves the exact-byte diagnostic used to find
the fixed-connector intermediate-state dependency; it does not normalize XML.

The uncontended 2026-10-17 strict baseline table and stage/slice totals are in
[docs/gaps.md](../../docs/gaps.md#m50--39-strict-authoring-cost-and-capacity-2026-10-17).
Run `cargo test --release --manifest-path target/authoring-baseline/Cargo.toml
-p misterdiagram-engine --example authoring-bench` with the same environment to
check the benchmark JSON parser against malformed, deep and oversized input.

For a performance pull request against current master, use the uninstrumented
clean-checkout comparison in [CONTRIBUTING.md](../../CONTRIBUTING.md#performance-pull-requests).
`run.py` builds both revisions before measuring and compares all five saved packages.
