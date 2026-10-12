# Roadmap

Status: pre-alpha, M0 and M1 met; Phase 2 is in progress. Target: Visio Standard 2021.

Phase 0 establishes the workspace, conventions, oracle scripts and private corpus. Phases 1
through 5 add reading and ShapeSheet evaluation, rendering, writing, authoring, and exports.
M0 closed at 1473b0d. M1 code closed at e7dfbd4 after all acceptance commands and two
clean independent frozen verification passes.

## Progress

- 2026-10-11: M0.1 scaffold, bounded OPC and CLI info. CI: 44 tests, 14 wasm libraries, all six gates passed.
- 2026-10-11: M0.2 conventions and Core-aware checklist. All feature rows remain missing.

- 2026-10-11: M0.3 runner smoke test.
- 2026-10-12: M0.4 oracle scripts. 204 generated drawings, 827857 sheet rows; blank dump 3385 cells.
- 2026-10-12: M0.5 corpus pin and 1239-file checksum manifest. All 1181 shipped files local; 104/1181 dumps complete, remainder deferred to later nights.
- 2026-10-12: M0.9 normal-process launch and COM attachment pass two-file Visio acceptance; M0 met.
- 2026-10-12: M1.1 to M1.4 reader, grammar,70 functions, inheritance and dirty recalc. All1387 packages parse;5548 deterministic mutations produce no panics.
- 2026-10-12: M1.5 CLI sheet/corpus tooling. Probe5344 eligible cells match; first full measurement shipped cached91.411080%, COM97.553995%. M1 remains incomplete while the97%cached floor is unmet. Shipped dumps104/1181.

- 2026-10-12: M1.6 numeric requirements pass: shipped cached 97.978884% over 5355260 cells; resolved 98.763183% over 7271082 cells. CI quick corpus gate and 104 functions implemented. All acceptance commands and both frozen verification passes are clean; M1 met. Shipped dumps 104/1181.

- 2026-10-12: M2.0 discovery fixes: strict cached 4302905/4408935 (97.595111%), resolved 7209425/7271082 (99.152024%), non-literal resolved 99.836044%. Basic-rectangle dump zero differences; CI seven gates pass; parity remains 10%.
