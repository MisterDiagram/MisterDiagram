# Gaps

Status: pre-alpha, M0 and M1 met; M2 human gate accepted; independent closure verification complete. Phase 3 is in progress; Visio acceptance of writer output is pending.

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

## G7: Phase 3 text fidelity after the accepted M2 human gate

Charles accepted the human gate on 2026-10-12 at 16:45. Four one-pixel boundaries
are accepted under the common-area rule; they remain visible measurements.
Text promotion and the 97.4911% oracle1 floor stand. Render CI is mandatory again.
Independent closure passes 2 and 3 are clean. The aggregate caller-built
sheet-budget issue from pass 1 is fixed and regression-tested. Phase 3 is in progress; writer acceptance remains pending.

The last full text snapshot has 6087 cached and 1311 COM failures. Counts below are
measured buckets or confirmed witnesses, not a disjoint root-cause census. A
bucket can contain additional causes; do not treat a representative witness as
proof that every row has that cause. Phase 3 must complete that attribution.

| Phase 3 item | Count and witness | Remaining work |
|---|---|---|
| Segoe font identity and compatible metrics | Dominant User.Width delta bucket: 360 cached, 180 COM; User.MenuItemTextSize: 262 cached, 90 COM. MWFNAVCTRL_M master 17 shape 464 and MWFCONCTRL_M master 125 shape 221 | Resolve portable font identity and compatible metrics; no system font scan |
| Date formatting | #VALUE! buckets: 309 cached TxtHeight, 108 User.TextHeight, 80 Height (497 cells); CALNDR_M master 9 shape 7 is a confirmed DATE/FORMATEX witness | Convert ISO-date hook values for formatting and implement missing date-picture tokens; classify the rest of the error bucket |
| Locale2057 | 96 cached TxtHeight failures explicitly report unsupported FORMAT locale 2057 | Implement witnessed locale handling |
| Fine rounding | 87 cached TxtWidth failures in the -0.000916-inch bucket; AZURECLOUD_M master 422 shape 5 | Resolve remaining metric quantization; broader TxtWidth bucket 1124 is not all rounding |
| Master-scale context | 1 confirmed failing witness: APPL_M master 7 shape 5, within 1054 cached TxtHeight failures; master 2 shape 5 is a matching counterexample | Attribute remaining scale failures and model the master-specific context without breaking correct scaled text |

Other cached buckets include 1742 User.Width, 746 Width, 198 Height and 26 DirArrowOffset;
these overlap the categories above. Complete failure and witness lists remain in
the private corpus report. PDF ligature extraction and reading order still need
stronger fidelity coverage beyond nonempty text. Performance budgets are not yet
frozen; the gate decision froze pixel ceilings only.

## G8: Phase 3 EMF+ and compound-line fidelity

Charles explicitly defers these from the M2 gate. Compound-line fidelity has
4 non-default generated witnesses (line/compound-1 through compound-4),
plus the compound-0 control; the line group passes
its frozen ceiling. EMF+ payload interpretation has 17 comment records in 1 generated
foreign-object drawing and is deferred to Phase 3. The legacy fallback remains rendered, with unknown records skipped.
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


M2.9.1 closes the shipped blank-cell blocker:1108/1108 contact drawing areas now
have ink, compared with465before. This is a coverage claim, not pixel parity
for every shipped master. Scene warnings remain visible, including unsupported
foreign records and foreign objects without painting operations.

## Phase 3: extreme stroke inputs (M2 review N-4)

Two synthetic review cases remain explicit stress-test work: a50-inch line
weight, and a custom dash array whose entries sum to zero. Audit the first for
bounded stroke expansion and raster memory across PNG/SVG/PDF. Scene validation
already rejects non-positive dash cycles; retain end-to-end malformed-metafile
coverage as follow-on work so no earlier expansion bypasses that check. These
are2review cases, not2measured corpus failures; no generated oracle input or
ceiling was changed. They do not establish compound-line fidelity, which remains
the separately deferred Phase3 formatting gap.

## G8: Phase 3 per-file render outliers

P3/M3.0 retains the frozen group pixel ceilings and aggregation. The additional
count ratchet fails `corpus check` above ten individual pixel outliers. Regression:
`per_file_outlier_ratchet_rejects_eleven_even_when_groups_pass`.

The ten current files, relative to `corpus/generated/`, are:

- `connector/dynamic-glue.vsdx`
- `connector/move-before.vsdx`
- `connector/static-glue.vsdx`
- `line/compound-1.vsdx`
- `line/compound-2.vsdx`
- `line/compound-3.vsdx`
- `line/compound-4.vsdx`
- `image/emf.vsdx`
- `page/metric-scale.vsdx`
- `text/multiline-wrap.vsdx`

The 6,087 cached and 1,311 COM text failures, 17 EMF+ payloads, compound-line
fidelity and two extreme-stroke cases above carry into Phase 3 as counted gaps.

## G9: Phase 3 writing and from-scratch acceptance

M3.1 provides the typed skeleton, observed style/cell sets and an authored
connector master with fresh UUIDs. Tests: `skeleton_parts_styles_and_fresh_connector_identity`,
`skeleton_cell_set_matches_blank_and_probe_observations`,
`rectangle_rejects_hostile_coordinates_and_text`. M3.2 adds model-driven writing,
opaque payload preservation, scalar cache recalculation and bounded hostile-model
validation. Tests include `unknown_parts_and_extensions_survive_modelled_edits`
and `serialization_does_not_apply_setf_actions_to_later_caches`. All 1,387
packages read-write-read and 5,548 mutations produce zero panics. Stencil/template
output, full dump/render measurement and the 230-file Visio acceptance remain
pending; structural re-reading does not establish save fidelity.
M3.3 adds explicit drawing/stencil/template output. The stencil retains masters
and a PageSheet-only metadata entry while removing drawing page parts. Tests:
`authored_stencil_and_template_retain_their_distinct_contracts` and
`existing_shipped_stencil_and_template_write_and_reopen`. Visio acceptance is
still pending.

M3.4 adds complete local-cell and fixed-viewport render comparisons to the
round-trip gate. Final CI passes all eight gates, including the 1,387-package
cell test at its 99% floor and the 256-package quick render set. An earlier quick diagnostic measured zero MAD and differing fraction for
every original/rewrite pair, with no render errors (769 master images plus
223 page images). The final full render sweep
and exact full cell rates must be recorded from a current-tree corpus check;
an older diagnostic run was stopped at the requested session handoff. No
230-file Visio acceptance or thumbnail acceptance is claimed.
