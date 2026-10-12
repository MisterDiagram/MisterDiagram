# Gaps

Status: pre-alpha, M0 and M1 met; Phase 2 theme evaluation implemented; text and output work remain.

## G1: Remaining ShapeSheet fidelity

The Phase 1 reader loads all 1387 corpus files; 5548 deterministic mutations produce no panics.
Formula syntax and local/cross-scope evaluation are implemented with bounded errors.
104 function names, master/style/default inheritance and dirty recalc are implemented.
CLI dumps and both numeric corpus oracles are implemented. Full shipped cached accuracy is
97.978884%; resolved accuracy is 98.763183%, with shipped dump coverage 104/1181. Unknown functions,
complete host font IDs and implicit master-child materialization remain open.
Dormant CAD geometry algorithms resume in Phase 2.

Phase 1 acceptance and two independent frozen verification passes are complete. Further
numerical fidelity, complete theme/text hosts and geometry remain later work. Keep the
Phase 2 planning estimate at 70 to 110 hours: renderer, text and theme work has not yet
been measured. Phase 1 took approximately 2.3 wall-clock hours with parallel agents.

## Phase 0 open work

- Visio acceptance resolved in M0.9: normal-process launch and attachment accepted both test files with 9257 cells and zero errors.
- Shipped dump: 104/1181 files complete after four hours; all 1181 files copied (725 stencils, 456 templates). Resume in later background sessions. Charles explicitly excludes this remaining dump coverage from M0 blockers.

## G2: Whole-file info input buffering

Phase 1: info reads the whole compressed file before package checks, within the inherited
deckcraft caps. Review finding N-3 is recorded; no Phase 0 change.

## G3: Closed stderr handling

Resolved in Phase 1 CLI: stderr writes are fallible and their errors are ignored while preserving the failure exit status. Review finding N-4 originally measured exit 1 without panic.

## G4: M1 discovery follow-up (M2.0)

S-1 separates literal, empty and non-literal formulas in both oracle reports. S-2 excludes
No Formula and filters effective inherited formulas, with Charles authorizing a new floor
on that corrected denominator (at least 97%). S-3/S-4 fix explicit inches, master drawing
units and complete COM Connection/Actions rows. S-5 records the document evaluator in L2
vsdx with L1 host interfaces for the upcoming theme/text implementations.
Regression: `basic_rectangle_corpus_has_every_com_identity_and_unit`; all 5748 source rows
have a computed identity and matching units. N-1 compares an empty V with empty Text only.

All five M2.0 closure checks pass. Final strict cached rate 97.595111%; resolved
99.152024%, with non-literal resolved 99.836044%. Read-only shipped coverage 104/1181.

## G5: Rendering (Phase 2)

M2.1 implements all geometry row types in y-up inches, section flags and local/page
transforms. Fifteen kinds have private corpus witnesses; relative Beziers, spline/NURBS
equivalence, clipping and hostile inputs have numerical regressions. The original CAD
editing algorithms remain dormant after their two earlier retarget failures; the active
row converter is new. Fills, lines, themes, text, images and all output formats remain next.

M2.2 paint primitives now cover procedural fill patterns 0..40, gradient stops with alpha,
line patterns 0..23, cap/join/compound bands, straight-corner rounding and arrow kinds 0..45.
Mixed curve/line corner rounding and precise curved-arrow profiles still need render-oracle work;
no visual score is claimed before Step 6. Marker size extrapolation outside corpus sizes
1,2,4 remains unmeasured.

M2.3 removes all eight theme functions from hooks and adds package theme/variant lookup,
no-theme defaults, gradient stops and basic effect values. Full oracle1 is98.128577%,
oracle2 is99.192198%, and theme-only COM is99.875218%. Theme cached accuracy is91.793858%:
remaining font IDs, style/color details and effect completeness must be judged alongside
rendered output. No raster fidelity claim. Theme and text host counts are reported separately.

## G6: M2.4 text fidelity work

The P2-followup embedded Latin pack resolves the missing Calibri-substitute resource.
The preceding blocked measurements follow as history: the substitute was absent from the read-only craft-fonts checkout.
All seven tested default families resolve to BIZ UDPGothic. Full oracle 1 is 95.161026%
against 97.5951%; text-only COM is 20.248276%. Do not commit this Rust work until CI
passes. The copied text layout, resolved run adapter and function dispatch are partial.
Empty-text minimum lines, precise metrics, dynamic field formatting, inherited tab
sets and aligned tab placement remain open. Rendering, export, contact sheets, renderer
fuzzing, performance gates and independent M2 review have not been reached.

M2.4.0 adds all 18 requested pinned font files, attribution and notices, and the authorized 90% text promotion gate. Arimo variables shape directly. Main floors stay fixed and theme stays included; a deferred text bucket is not milestone acceptance.

M2.4 now supplies font-size percentage spacing, a minimum empty line, trailing paragraph
spacing, aligned tab placement and formatted nonvolatile fields. The original empty-label
height matches its corpus witness. Full text metrics and all rendered text remain measured
gaps until the later render and promotion gates; date/document fields still use their
saved display while their separate hooks remain deferred.
