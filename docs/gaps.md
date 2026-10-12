# Gaps

Status: pre-alpha, M0 and M1 met; Phase 2 remains partial after pixel and text-metric measurement.

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

M2.5 provides PNG/JPEG and EMF foreign objects, page/background/layer rendering and PNG, SVG and vector PDF backends. The generated 204-file output sweep passes. This does not measure pixel agreement. EMF+ and unknown metafile records are skipped with diagnostics; unsupported raster operations and clip subtraction remain visible warnings. Compound line offsets, explicit gradient-stop painting, shadow/effect rendering and inherited child materialization still require renderer work.

## G7: P2-followup-4 human gate

Nine of ten groups pass; four one-pixel text export residuals remain:
align-0-0 and align-0-1 are388x289 versus387x289, align-0-2 is388x291 versus387x291,
and sizes is481x531 versus481x532. Multiline-wrap, metric-scale and right/bottom
alignment now have matching dimensions. Logical overhang and ascent rounding
remain an incomplete approximation. Tests: `text_blocks_and_unkerned_lines_match_visio_svg`
and `png_composition_and_logical_bounds_have_private_witnesses`.

Text promotion passes91.021727% cached and90.713982% COM. Charles authorized the
oracle1 floor rebase on the promoted denominator to97.4911%; oracle2 remains98.6241%.
Remaining6087cached/1311COM text failures include font-ID/Segoe metrics, date
picture and locale formatting, fine advance rounding and master scale/context.
The measured bucket table and witnesses are in file-format-parity.md and the
private session report. No oracle values or render ceilings changed.

All 1108 contact masters render. CI render comparison alone remains advisory;
strict corpus check still fails, and M2 remains unaccepted. Performance budgets
and independent closure review remain pending the human gate. PDF ligature
extraction and full reading order need stronger tests beyond nonempty text.

## G8: Deferred rendering fidelity (Phase 3 or later)

Charles explicitly defers these from the M2 gate. Compound-line fidelity remains
open; the line group passes its existing ceiling. EMF+ payload interpretation is
also deferred. The legacy fallback remains rendered, with unknown records skipped.
The seventeen EMR_COMMENT records in generated/image/emf.vsdx, media/image1.emf,
contain the following EMF+ record IDs. Offsets and sizes identify the private
witness without copying its artwork or payloads.

| Comment | EMF byte offset | Payload bytes | Inner record IDs (record bytes) |
|---|---:|---:|---|
| 1 | 108 | 32 | 0x4001(28B) |
| 2 | 152 | 144 | 0x4030(16B), 0x4022(12B), 0x402a(36B), 0x4032(28B), 0x402a(36B), 0x4004(12B) |
| 3 | 392 | 128 | 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x400a(36B) |
| 4 | 744 | 328 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(60B), 0x4008(72B), 0x4015(16B) |
| 5 | 1332 | 192 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4004(12B) |
| 6 | 2260 | 288 | 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x400a(36B) |
| 7 | 2784 | 328 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(60B), 0x4008(72B), 0x4015(16B) |
| 8 | 3372 | 192 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4004(12B) |
| 9 | 4300 | 412 | 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(144B), 0x4014(16B) |
| 10 | 5020 | 256 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(60B), 0x4015(16B) |
| 11 | 5568 | 192 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4004(12B) |
| 12 | 6492 | 372 | 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(60B), 0x4008(44B), 0x4015(16B) |
| 13 | 7120 | 308 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4008(60B), 0x4008(52B), 0x4015(16B) |
| 14 | 7668 | 216 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x400a(36B) |
| 15 | 8076 | 192 | 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4025(16B), 0x401f(12B), 0x401e(12B), 0x4021(12B), 0x402a(36B), 0x4004(12B) |
| 16 | 8976 | 200 | 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x402a(36B), 0x4026(16B), 0x4008(24B), 0x4034(12B) |
| 17 | 9320 | 16 | 0x4002(12B) |

M2.4.5 text regressions fix final-run advances, indented empty fields, numbered decimal pictures, local inherited display units and integral fractions. Segoe-related font IDs and matching metrics remain unresolved. `text_metric_terminal_fields_and_unit_override_witnesses` covers the concrete fixes.
