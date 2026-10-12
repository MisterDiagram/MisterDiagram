# Gaps

Status: pre-alpha, M0 met; Phase 1 in progress.

## G1: ShapeSheet evaluation in progress

The Phase 1 reader loads all 1387 corpus files; 5548 deterministic mutations produce no panics.
Formula syntax and local/cross-scope evaluation are implemented with bounded errors.
69 functions, master/style/default inheritance and dirty recalc are implemented.
CLI dumps and both numeric corpus oracles remain in progress. Unknown functions,
complete host font IDs and implicit master-child materialization remain open.
Dormant CAD geometry algorithms resume in Phase 2.

Phase 1 planned effort remains 70 to 100 hours until evaluator measurements are available.

## Phase 0 open work

- Visio acceptance resolved in M0.9: normal-process launch and attachment accepted both test files with 9257 cells and zero errors.
- Shipped dump: 104/1181 files complete after four hours; all 1181 files copied (725 stencils, 456 templates). Resume in later background sessions. Charles explicitly excludes this remaining dump coverage from M0 blockers.

## G2: Whole-file info input buffering

Phase 1: info reads the whole compressed file before package checks, within the inherited
deckcraft caps. Review finding N-3 is recorded; no Phase 0 change.

## G3: Closed stderr handling

Phase 1: review finding N-4 measured eprintln on closed stderr as exit 1, with no panic.
Consider explicit error-channel handling; no Phase 0 change.
