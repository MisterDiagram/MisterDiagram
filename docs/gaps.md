# Gaps

Status: pre-alpha, M0 met; Phase 1 in progress.

## G1: ShapeSheet evaluation in progress

The Phase 1 reader loads all 1387 corpus files; 5548 deterministic mutations produce no panics.
Formula syntax and local/cross-scope evaluation are implemented with bounded errors.
104 function names, master/style/default inheritance and dirty recalc are implemented.
CLI dumps and both numeric corpus oracles are implemented. Full shipped cached accuracy is
97.978884%; resolved accuracy is 98.763183%, with shipped dump coverage 104/1181. Unknown functions,
complete host font IDs and implicit master-child materialization remain open.
Dormant CAD geometry algorithms resume in Phase 2.

Remaining Phase 1 closure work is final acceptance and two clean independent verification
passes. Further numerical fidelity, complete theme/text hosts and geometry remain later work.

## Phase 0 open work

- Visio acceptance resolved in M0.9: normal-process launch and attachment accepted both test files with 9257 cells and zero errors.
- Shipped dump: 104/1181 files complete after four hours; all 1181 files copied (725 stencils, 456 templates). Resume in later background sessions. Charles explicitly excludes this remaining dump coverage from M0 blockers.

## G2: Whole-file info input buffering

Phase 1: info reads the whole compressed file before package checks, within the inherited
deckcraft caps. Review finding N-3 is recorded; no Phase 0 change.

## G3: Closed stderr handling

Resolved in Phase 1 CLI: stderr writes are fallible and their errors are ignored while preserving the failure exit status. Review finding N-4 originally measured exit 1 without panic.
