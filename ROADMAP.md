# Roadmap

Status: pre-alpha, M0 and M1 met; M2 human gate accepted, independent review pending. Target: Visio Standard 2021.

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

- 2026-10-12: M2.1 all fifteen geometry row kinds to kurbo paths, bounded NURBS/splines, section flags and y-up group transforms. Private corpus witness and numerical tests added; renderer work remains next.

- 2026-10-12: M2.2 procedural paints, strokes and arrow primitives. Corpus-derived fill densities and dash lengths tested; rendered fidelity is not yet measured.

- 2026-10-12: M2.3 all eight theme functions evaluated and included in the cell oracles. Full cached98.128577%, resolved99.192198%, theme-only resolved99.875218%. Parity11%; image parity remains unmeasured.

- 2026-10-12: M2.4 partial worktree stopped for the missing Latin font pack. Full cached95.161026% fails the frozen floor; resolved99.100234%, text-only20.248276%. CI fails its mandatory corpus gate. Text rows remainP, parity14%; no M2.4 commit or M2 closure.

- 2026-10-12: M2.4.0 embeds the requested 18 OFL Latin faces and adds Charles's 90% text promotion gate. CI and public no-craft-fonts tests pass (230 tests); quick main oracles98.879415%/99.037858%, text39.341608%/41.824862%, still deferred. Text completion and rendering remain open.

- 2026-10-12: M2.4 and M2.5 committed with passing CI: text layout/fields/tabs and PNG/SVG/vector PDF backends. Shared UTF-16 dump coverage is read without mutation. M2.6/M2.7 remain uncommitted at the partial gate: 204 renders, 35 size mismatches, nine failing pixel groups; SVG/PDF checks pass. Twenty-four paired sheets cover 1108 masters, with 591 failed local renders. Parity 23%; no M2 acceptance.

- 2026-10-12: P2-followup-2 worktree: arrow/cap dimensions fixed (35 size failures reduced to10), two edge profiles match exactly, BASIC theme extension indexing corrected, contacts1107/1108, foreground/background selection corrected. Text height slack, percentage baselines, drawing-unit conversion and empty-height handling have corpus regressions. Gates remain partial; no new Rust commit while CI is red.

- 2026-10-12: P2-followup-2 partial gate: main cached98.118866%, COM99.226552%; text80.083190%/80.165717%, still deferred. Ten wrong sizes, eight failing render groups; fill/line pass. Contacts1107/1108, one BASFLO direct cycle. CI fails render, so no commits or M2 closure.

- 2026-10-12: P2-followup-3 commits M2.6/M2.7 under Charles's advisory-render CI rule, then SVG text advances, marker profiles, transformed strokes, transparent image sampling, the final master cycle, FIND and fixed-point text metrics. CI 8 gates pass; strict corpus remains partial: 8/10 render groups pass, 10 size mismatches; text 84.481614%/83.415329% stays deferred. Contacts 1108/1108, 24 paired sheets; 204 render commands and SVG/PDF checks pass. No ceilings, floors or oracle files changed.

- 2026-10-12: P2-followup-4 fixes composed width, field formatting, inherited display units and six text export dimensions. Text passes promotion at 91.021727% cached and 90.713982% COM; oracle1 is 97.491192% on its promoted denominator, rebased floor 97.4911% by Charles. CI passes all eight gates. Strict corpus remains partial: nine groups pass, four one-pixel text dimensions fail. All 204 renders and SVG/PDF checks pass; 24 paired sheets cover 1108 masters. Human gate pending, no M2 closure.

- 2026-10-12 16:45: Charles accepts the M2 human gate with one-pixel export tolerance over the common area. Original pixel ceilings are frozen, render CI is mandatory again, and promoted cell floors stand. All ten render groups pass the rule test. Remaining text, 17 EMF+ comments and four non-default compound-line witnesses are Phase 3 gaps. Independent M2 review runs before Phase 3.

- 2026-10-12 M2.9.1: independent review invalidated prior1108/1108 rendering claims:465cells had ink and643were blank. EMF header detection, compound fills and group ordering restore1108ink/0blank/0errors across24stencils. Generated oracle numbers are unchanged; closure verification remains pending.
