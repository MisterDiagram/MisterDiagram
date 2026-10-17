# Gaps

Brief 22 (2026-10-16): the MisterDiagram rename changes no parity score or feature gap. Dated measurements below retain their original names and paths.

Status: pre-alpha, M0 through M3 met under the recorded gates. P4a met on 2026-10-15 under the straight-route ruling. M4 plan gates are met on 2026-10-15 under the closure-2 rulings and two clean frozen passes. The carried rendering and routing gaps remain measured below.

## G0: Contributed fixes

- PR #4 (Matheus Breguêz): a self-closing element no longer bypasses the OPC XML depth cap; `depth_boundary_is_independent_of_empty_element_syntax` and `public_xml_parser_enforces_depth_for_both_empty_element_forms` hold MAX_DEPTH for both element forms. Closes #1.
- PR #5 (Matheus Breguêz): VBA project parts are classified by OPC content type as well as filename, so a renamed payload is dropped and flagged; `typed_vba_parts_are_removed_with_default_and_override_content_types`. Closes #2.
- PR #6 (Matheus Breguêz): macro-enabled stencils and templates (.vssm, .vstm) are admitted by the package validator and the CLI; `imports_macro_enabled_stencils_and_templates_without_vba_payloads`. Closes #3.
- PR #35 (Matheus Breguêz): TRIM evaluates with the MS-VSDX whitespace set; the reference example and all 23 scalars are tested in `crates/sheet/tests/trim.rs`, imported-cell recalculation in `crates/vsdx/tests/trim.rs`. Closes #7.

## G1: Remaining ShapeSheet fidelity

The Phase 1 reader loads all 1387 corpus files; 5548 deterministic mutations produce no panics.
Formula syntax and local/cross-scope evaluation are implemented with bounded errors.
104 function names, master/style/default inheritance and dirty recalc are implemented.
CLI dumps and both numeric corpus oracles are implemented. Full shipped cached accuracy is
97.978884%; resolved accuracy is 98.763183%, with shipped dump coverage 104/1181. Unknown functions,
complete host font IDs and implicit master-child materialization remain open.
Dormant CAD geometry algorithms resume in Phase 2.

M2.8.2 extends main-part validation and CLI info to macro-enabled stencils and
templates. Six package families are admitted, using package metadata for type
validation. Synthetic imports strip conventional VBA parts and preserve opaque
content; no runtime or rendered fidelity measurement is claimed.

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
the private corpus report. PDF reading order still needs
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

M3.5 pre-VM acceptance preparation covers230packages and540page/master images.
Nineteen selected shipped images cannot yet render: ENTITY_U(2, CHAR), USTRME_U(1,
cycle), EEPATH_U(2, REF), UMLSEQUENCE_INTERACTIONFRAME_M(2, LISTORDER), PTSKST_M(6,
transform/value errors and a text cycle), SOCIAL_DISTANCING_M(6, unsupported foreign
image, cycles and text/transform value errors). These are explicit failed images,
not omitted comparisons. The full corpus sweep will provide broader counts.
The five scratch model constructions build and re-read, but Visio open/repair and
thumbnail acceptance remain unmeasured until the VM report completes.

### M3.5 acceptance remains blocked (2026-10-13)

The run has 225 completed status files: 224 opened and saved, one rejected on
open. SOCIAL_DISTANCING_M also logged OPEN OK but timed out during its dump and
has no completed status. Four subsequent templates were not reached. The reporter's
repaired=0 counters are unmeasured because it exits each failed-file comparison;
they are not evidence of zero repairs.

The harness calls ZipFile.OpenRead(saved) while Visio still holds the SaveAs file
open. That sharing violation affected all 224 completed saves, before part listing
and PNG export. A follow-up must export while open, close the document, then inspect
and hash the saved ZIP. The failure is in our harness, not evidence that the writer
passed or failed repair checks. Preserve this attempt and its hashes before rerun.
The timeout recovery also exposed an existing PowerShell quoting error in the
runner's `Write-Output ...; forcing stop` diagnostic. Its log is retained.

scratch__dynamic-glue.vsdx was rejected as corrupt. It is also the authored PNG
thumbnail case, so the cause is not isolated between thumbnail and connector/package
construction. scratch__rectangle-text.vsdx (no thumbnail) opened and saved, but its
repair and render checks remain unmeasured. No thumbnail acceptance conclusion is
claimed beyond this observed open/save behavior.

Full host round-trip cells: 15,080,738/15,080,738 (100.000000%). All successfully
compared original/rewrite images have MAD=0, fraction=0 and no size mismatch.
There are 568 master and 74 shipped-page render errors, 642 total. The full render
gate fails. Generated Visio comparisons pass all ten groups, with ten disclosed
outliers, four tolerated one-pixel differences and zero hard size failures.

Fresh coverage: 1,387 packages, 1,181 shipped, 413 completed shipped dumps,
626 dump files. Shipped cached oracle: 4,675,663/4,783,865 (97.738189%), above
97.491100%. COM: 27,416,668/27,803,388 (98.609090%), below 98.624100%.
Text remains promoted: cached 61,748/67,797 (91.077776%); COM 16,631/18,113
(91.818031%). Coverage grew; no floor, ceiling, denominator or corpus observation
was changed. Parity remains partial at 25%.

M3.7 thumbnail item CLOSED by Charles's P3-closure decision and diagnostic variants:
all five scratch documents now omit thumbnail parts and relationships. The existing
EMF round-trip remains byte-preserving. PNG rejection is isolated by v1-nothumb;
the separate dynamic-glue issue and post-save repair comparisons remain open.

### M3.9 carried renderer gaps

The previous full original-side sweep had568master and74page failures (642total).
These remain renderer work for a later pass. Identical failures on both sides of a
read-write-read pair are equal for the writer gate under Charles's P3-closure rule;
one-sided or different errors still fail. Equal errors are reported separately and
are not assigned pixel scores. Generated Visio comparisons retain their frozen gates.

| Original error class | Count | Witness |
|---|---:|---|
| transform #VALUE! | 272 | ms-shipped/1033/BPMN_ADDRESS_CHANGE_PROCESS_M.vstx#master5 |
| #VALUE! | 74 | ms-shipped/1033/BCKGRN_M.vssx#master7 |
| #CYCLE! | 70 | ms-shipped/1033/BASFLO_M.vssx#master9 |
| REF | 64 | ms-shipped/1033/BLDCOR_M.vssx#master8 |
| CONTAINERSHEETREF | 62 | ms-shipped/1033/BPMN_ADDRESS_CHANGE_PROCESS_M.vstx#master4 |
| LISTORDER | 33 | ms-shipped/1033/CROWHRMANAGEMENTSYSTEM_U.vstx#master2 |
| image format undetermined | 21 | ms-shipped/1033/BPRES_M.vssx#master7 |
| #NUM! | 16 | ms-shipped/1033/BLDCOR_M.vssx#master1 |
| CHAR | 16 | ms-shipped/1033/ENTITY_M.vssx#master1 |
| #NAME? _UCON_GEOTYP | 6 | ms-shipped/1033/COMOLE_M.vssx#master10 |
| #NAME? LISTMEMBERCOUNT | 4 | ms-shipped/1033/WFCTRL_M.vssx#master11 |
| #DIV/0! | 2 | ms-shipped/1033/PROCESS_M.vssx#master20 |
| #NAME? _UCON_ENDTYP | 2 | ms-shipped/1033/IDEF0_M.vssx#master8 |

### P3-closure acceptance blockers (M3.11, 2026-10-13)

- 230/230 opened, 230/230 repaired by the frozen comparison:4,107 literal/part differences
 and295,787 source COM differences. Basic probe witnesses BulletStr 0 versus empty,
 LangID 1033 versus en-US, and TextBkgnd 26 versus#ffffff.
- All 5 scratch saves add thumbnail.emf despite the explicitly required thumbnail-free
 input. Preserve both the construction rule and the strict repair result; do not exclude it.
- Scratch three-masters loses all 3 master parts and masters.xml.rels on Visio SaveAs.
 Scratch metric/background/layers has99 literal/part and323 source COM differences.
- Acceptance render failures remain in 7 groups: blank-pages, from-scratch-pages,
 from-scratch-masters, generated-masters, shipped-stencil-masters,
 shipped-template-masters and shipped-template-pages. The10named generated page groups pass.
- Promoted COM text27,360/30,261 =90.413403%, below the unchanged 90.713900% floor.
 Remaining text failures: 6,049 cached,2,901COM. Main COM coverage rebase does not waive
 the independent sticky text ratchet. Coverage is 702 completed shipped dumps.
- Frozen milestone verification cannot close while the measured acceptance and text gates fail.

Per-file JSON and full worst-file lists are retained privately in
corpus/_state/m3/accept-report.json and corpus/_state/P3-closure-report.md.

### M3.12 source-cache correction

The M3.11 source COM count above is historical and unreliable: its expected dump
resolved inherited providers rather than the local values actually written. The
claim that Visio normalized the original BulletStr, LangID and TextBkgnd values
was also wrong. Our writer replaced those values. M3.12 preserves unchanged local
caches, with raw arrow/probe/SITACC regressions and typed dirty-cache tests. The
historical acceptance results still need the M3.18 restatement; this fix alone does
not close Visio acceptance.

P3-closure-2 M3.13 supplies missing inherited instance caches and preserves existing
local `Inh` records. The basic-rectangle exact-cell-set regression passes, including
reconstruction of its Geometry and Connection caches. Visio export validation of the
resized scratch master remains pending the new acceptance run. This does not yet
close the dropped-master render gap.

M3.14: authored master content IDs now start at 5 and invalid lower IDs are rejected.
Original Visio ID 4 is retained with a source-byte witness (PHONE_CONN_U.vstx).
Host regressions pass; the three live master counts and PNG sizes await the fresh
M3.23 acceptance run. No live acceptance claim is made from host tests.

M3.15 host fixes cover the missing background relationship, stale page links, dangling
Connects and metric default-unit metadata. Millimetre and metre contexts have separate
regressions. The scratch metric COM/part count above remains historical until rerun.
The renderer's background-chain depth bound is128; the writer permits bounded acyclic
chains beyond it. No corpus failure is attributed to that synthetic limit.
M3.16 closes the host-side missing routed-state defect for the authored connector.
Straight horizontal, reverse, vertical and diagonal route regressions pass. Live gap
ink and two-Connect acceptance remains pending; the earlier COM result alone did not
prove visible routed geometry.

M3.18 restates the historical run on unchanged inputs and Visio output: 230 opened,
230 repaired, 30,237 literal/part differences and 180,294 source COM differences.
The 17,223 unobserved style-local XML cells are listed separately, with unchanged
numeric eligibility and mandatory literal XML comparison. The reporter also
separately discloses 1,296 phantom PageSheet unit slots and 14 MasterType rewrites.
These replace the unreliable M3.11 counts above. Fresh writer acceptance is pending.

### M3.21: pinned COM text cohort ruling

Charles's P3-closure-2 M3.21 authorizes the same coverage rule for sticky text COM.
Old text: 12807/14118 = 90.713982150%, meeting the 90.7139% contract.
New text: 14553/16143 = 90.150529641%. Combined: 27360/30261 = 90.413403391%.
The combined sticky text floor is 90.413403300%, rounded down at nine decimal places.
The cached text floor is unchanged. Pinned historical 702-dump main COM remains 44,046,421/44,727,163 =
98.478012120%, with floor98.478012100%. Its old cohort was20,805,945/21,055,732 =
98.813686458%, above98.6241%; new was23,240,476/23,671,431 =98.179429879%.

The preceding brief's413-dump observation was27,416,668/27,803,388 =98.609090%,
framed as302old+111new. Proven completed membership was301old+112new, because
one old artifact was partial. The702-dump measured main floor stands. The later872-dump audit measured
53,865,489/54,718,413 =98.441248652%, below that standing floor; it is not rebased. Both main
and text cohorts are now committed under apps/diagramcraft-corpus/tests/cohorts/,
with exact dump paths, counts and SHA256 pins in floors.toml. Generated/blank213
dumps stay in the old population; old total514, combined915. These fixed lists
cannot silently move to a later snapshot. The shared source directory is read-only.

### M3.22: Phase 4 shipped acceptance rendering gaps

Charles's P3-closure-2 keeps the selected ten stencils and ten templates mandatory
for open, part, attribute and COM checks. Their render rows are informational;
generated, blank and authored scratch render rows remain mandatory. This is the
explicit M3.22 scope ruling, not a change to the frozen pixel ceilings.

Historical run, corrected M3.18 reporter (before the fixed writer's rerun):

| Render error class | Views | Witness |
|---|---:|---|
| #CYCLE! | 4 | shipped-stencil__USTRME_U#master-10 |
| #VALUE! | 8 | shipped-template__PTSKST_M#page-0 |
| CHAR | 2 | shipped-stencil__ENTITY_U#master-2 |
| LISTORDER | 2 | shipped-template__UMLSEQUENCE_INTERACTIONFRAME_M#page-0 |
| our renderer: The image format could not be determined | 1 | shipped-template__SOCIAL_DISTANCING_M#page-0 |
| our renderer: shape 5 geometry: #NAME? REF | 1 | shipped-stencil__EEPATH_U#master-27 |
| our renderer: shape 5 transform: #NAME? REF | 1 | shipped-stencil__EEPATH_U#master-23 |
| size | 38 | shipped-stencil__ENTITY_U#master-10 |

Fresh PLANT and SOCIAL_DISTANCING COM counts and ENTITY_U/UCMNME_U size witnesses will be recorded after M3.23. Their historical values are not evidence for the fixed writer.

## M3.23 fresh acceptance, 2026-10-14 UTC: partial

Required results are not met. All 230 files opened, but 226 are repaired under the
M3.20 definition and 229 fail at least one mandatory check. There are 5,856 repair
differences and 179,958 source COM differences. COM is a separate column and does
not by itself mark a file repaired. No new repair exception was added.
From-scratch results:

| File | Repairs | Source COM differences | Measured result |
|---|---:|---:|---|
| rectangle-text | 5 | 0 | Both page images 193 x 97; pixel ceiling fails |
| dynamic-glue | 8 | 0 | Both 481 x 97; dark route in Visio, two COM connects; pixel ceiling fails |
| dropped-master | 5 | 0 | Both 193 x 97; resized geometry present; pixel ceiling fails |
| three-masters | 5 | 0 | Three masters retain one shape each; both sides 97 x 97, 145 x 97, 193 x 97; pixel ceiling fails |
| metric-background-layers | 19 | 65 | Background relationship retained, no unexpected part difference; pixel and COM checks fail |

Our scratch fills are opaque black where Visio paints white. This is not an alpha
comparison artifact: rectangle-text pixel (3,3) is (0,0,0,255), while Visio is white.
The serialized default FillForegnd is #ffffff and the palette only declares index24.
Evaluator::cached_cell recognizes a literal RGB cache only through a declared
palette match, otherwise its numeric cached-value path returns #VALUE!; the scene
color fallback paints black. The page sheet and shape FillForegnd diagnostics both
show #VALUE! in log/m3-23-scratch-sheet.tsv. This is an implementation gap, not
Visio changing the intended fill or grounds for changing the pixel comparison.

The remaining scratch repairs include empty Copyright/HelpTopic cells, inherited
Comment/ShapeKeywords, master Icon additions, removed connector caches and missing
layer defaults. The metric page height also differs in final decimal digits.
Metric COM still has 65 differences, including Para.IndFirst/IndLeft/IndRight and
TextPosAfterBullet units 73 versus 70 when U is absent. The zero-COM requirement
is not met. No vendor Icon is copied into the authored skeleton.

The most frequent generated repair witness is pages.xml Page ViewCenterX/Y:
begin-00 and 0001-probe change 4.2538462786703 to 4.2461539707522 and
5.5076924693091 to 5.500000161391. These are still repairs under M3.20.
Seven generated page images fail strict size comparison: connector/obstacle,
one line image and five text images (the exact names and sizes follow in the report).
Generated master pixels also fail (2.648070% over 16). The M2 generated-corpus gate
still passes all ten frozen groups, with ten individual outliers; it is a different
comparison and retains its approved one-pixel rule. M3 acceptance is strict.

Shipped render checks are informational only. The selected shipped files still
must pass open, part, attribute, literal and COM checks. PLANT_M has 25 repairs
and 125,161 COM differences; SOCIAL_DISTANCING_M has 4,876 repairs and 27,436 COM
differences. These are fresh fixed-writer measurements. ENTITY_U master4 is
98 x 98 versus 99 x 99; UCMNME_U master5 is 97 x 49 versus 865 x 433.
These render gaps carry into Phase 4, while structural/COM failures still block M3.

- generated__connector__obstacle#page-0: ours [529, 289], Visio [529, 307]
- generated__line__pattern-00#page-0: ours [480, 384], Visio [384, 288]
- generated__text__align-0-0#page-0: ours [388, 289], Visio [387, 289]
- generated__text__align-0-1#page-0: ours [388, 289], Visio [387, 289]
- generated__text__align-0-2#page-0: ours [388, 291], Visio [387, 291]
- generated__text__multiline-wrap#page-0: ours [206, 385], Visio [207, 385]
- generated__text__sizes#page-0: ours [481, 531], Visio [481, 532]

| Shipped render error | Views | Witness |
|---|---:|---|
| #CYCLE! | 4 | shipped-stencil__USTRME_U#master-10 |
| #NAME? REF | 2 | shipped-stencil__EEPATH_U#master-23 |
| #VALUE! | 8 | shipped-template__PTSKST_M#page-0 |
| CHAR | 2 | shipped-stencil__ENTITY_U#master-2 |
| LISTORDER | 2 | shipped-template__UMLSEQUENCE_INTERACTIONFRAME_M#page-0 |
| our renderer: The image format could not be determined | 1 | shipped-template__SOCIAL_DISTANCING_M#page-0 |
| size | 150 | shipped-stencil__AZUREVMWARESOLUTION_M#master-2 |

Later full COM coverage adds 170 completed dumps beyond the pinned 702. Its rate
98.441248652% is below the standing main floor 98.478012100%; this is an open
Phase 3 gate, not another authorized rebase. New-set failing function occurrences
are non-disjoint: no-function 166,151, THEMEVAL 3,661, IF 1,449, GUARD 559,
THEMEGUARD 442. Witness DVCALL_M master50 Textcallout: LocPinY Height*.5 yields
#VALUE!, and User.ThemeColor differs (81 versus color index32). Full evidence is
log/m3-21-coverage.log. The sticky text floor is resolved under the pinned M3.21 rule.

### Phase 5: authored master icons

The seven master entries across the five scratch documents have no authored Icon.
Visio generates previews on save. MisterDiagram will author its own master icons
in Phase 5; no vendor icon is copied into the skeleton. Baseline subtraction remains
per file and path: an unmatched scratch Icon addition is disclosed as a residual
repair rather than silently borrowing another master's baseline event.

### Small master export coverage (M3.29, Phase 4)

The generated BASIC rectangle masters retain an edge-sampling gap. For
`generated/master/basic-rect.vsdx` master 2 (145 x 97), the top-centre vertical RGB
profile is ours `[191,64,255,255]`, Visio `[127,127,255,255]`; the left-centre
profile is `[191,64,255,255]` on both. All 574 pixels over 16 are within four pixels
of the image border. Master 4 has 592 such pixels: 559 in that border band and
33 on its rounded corners. Probe master 2 (97 x 97) has 189, all in the border band.
Both top profiles have approximately the same integrated darkness; their coverage
centroids differ by about a quarter pixel. This is an observed sampling/placement
gap, not evidence of a missing shape. No rendering rule or ceiling is changed on
the strength of this small-image case. Private evidence: paired VM master PNGs and
`log/m3-29-before-profiles.json`. Generated master group remains a required gate;
recording the implementation work for Phase 4 does not mark that gate passed.

## M3.30 remaining closure gaps, measured 2026-10-14

Required M3 repairs remain open, not deferred or excused:

| File or set | Residual repair differences | Witness and cause |
|---|---:|---|
| Five scratch files | 693 | rectangle 139, dynamic glue 140, dropped master 148, metric 148, three masters 118. OPC/content-type ordering and byte serialization, docProps/windows metadata, FaceNames metadata and generated Icon differences lack matching blank-baseline identities. Empty master defaults may be removed by SaveAs. |
| AZURECOMPUTE_M | 14 | Root relationship bytes/attributes change when the missing thumbnail relationship is restored: 10 value differences and 4 additions beyond that file's baseline. |
| PTSKST_M | 24 | Six newly emitted named User.ValueFormat rows carry IX; Visio removes IX. Each becomes a removed/added row and cell identity. Example page1.xml shape27/shape34. |
| SOCIAL_DISTANCING_M | 3974 | 3 relationship changes, 28 XCon removals, 705 Geometry A additions, named-row/attribute changes, Value/Menu/Prompt changes and 6 ComplexScriptFont changes. Example page1.xml shape1006 Control.Row_1.XCon; shape1050/1074/1076 Geometry row2 A. Original XCon count151, ours181, Visio save of ours153, original baseline151. |

The full SOCIAL counts by comparison category are in private
`corpus/_state/m3/residual-repair-causes.json`. These are writer defects under
the exact baseline rule. No denominator or repair exception is changed.
Its 309 residual COM differences remain required too; page0 shape337
Prop.VisWallHeight and User.3D_extrude_length are witnesses (`2.6 m` versus `2.6`).
PLANT_M's125169raw differences are also present in its baseline: residual0.

Phase 4 renderer work (these three required pixel groups still prevent M3 closure):
generated masters MAD1.202102,2.648070% over16; scratch masters MAD2.823616,
4.452799%; scratch pages MAD4.609743,3.019190%. All required dimensions pass the
one-pixel rule. Master edge profiles are documented above; scratch text faces
and antialiasing remain visibly different. No ceiling moves.

Shipped rendering remains informational under M3.22. Latest error inventory:

| Class | Images | Witness |
|---|---:|---|
| Hard size mismatch | 39 | ENTITY_U master10; UCMNME_U master11 |
| CHAR | 2 | ENTITY_U master2 |
| LISTORDER | 2 | UMLSEQUENCE_INTERACTIONFRAME_M page0/master27 |
| REF | 2 | EEPATH_U master23 transform and master27 geometry |
| CYCLE | 4 | USTRME_U master10; PTSKST_M master4 text block; SOCIAL_DISTANCING_M page4 |
| VALUE transform | 7 | PTSKST_M page0 and master2 |
| VALUE text | 1 | SOCIAL_DISTANCING_M master42 |
| Undetermined image format | 1 | SOCIAL_DISTANCING_M page0 |

These58image failures are retained in the report. Shipped open, repair and COM
checks remain mandatory. All input and output observations are preserved.

## M3.33 vertical export investigation: mapping unresolved

M3.31 prevents inherited-cache expansion on unchanged read shapes. The new
regression checks exact local shape/section/row/cell keys and literal attributes
for all 20 shipped acceptance originals. M3.32 completes the observed authored
package metadata. Neither change has a new Visio acceptance measurement yet;
the M3.30 repair and COM counts above remain historical, not updated results.

Six global Y-only mappings were compared with the existing VM images. None
passes every required group. The best observed fraction for scratch masters is
2.055466% (MAD 1.077698), and for scratch pages 2.490908% (MAD 3.735655), from
different candidates. Both still exceed 2%. These are diagnostic minima, not a
combined render rule or a replacement gate. The horizontal-plus-grid candidate
improves generated masters to 0.101071% but fails image at 2.022449%.

The 66-view measured edge table and candidate group table are recorded in
file-format-parity.md. A further witness, scratch dynamic-glue page-0 at row10,
has dark-core edge positions x=1,192..193,288,479 in ours and x=5,193,287,475 in
Visio, with identical 481x97 dimensions. The requested vertical-only change
cannot move those X coordinates. No production mapping was changed and no
ceiling was moved. This does not prove that every untested mapping must fail.

The P3-closure-4 stop was a faithful reading of that brief. Charles clarified in
P3-closure-5 that it stops mapping work, not the whole brief. The three small-image
groups remain measured, ruling pending. Comparison, rerun and verification work
have resumed. M3 is not accepted; no Phase 4 implementation has begun.

### M3.37 connector extent diagnostic (2026-10-14)

Bounded diagnostic completed in under one hour, with no renderer change. The
comparison covers 67 extent-related cells: scratch page 0 shape 3 against the
original probe page 0 shape 4 (the first, straight Dynamic connector). Sources:
`corpus/_vm/out/accept/scratch__dynamic-glue/observed.sheet.tsv` and
`corpus/_vm/out/baseline/generated__basic__0001-probe/observed.sheet.tsv`.
The pre-rerun scratch dump is preserved at
`corpus/_vm/out/accept/history-scratch__dynamic-glue-1c7ac390b42e41ac8ca5b55c874f7b44/observed.sheet.tsv`.
The source hashes, formulas, units and all equal rows are retained privately in
`corpus/_state/m3/closure5/extent-cell-diagnostic.json`.

| Differing cell | Scratch ResultIU | Probe ResultIU | Interpretation |
|---|---:|---:|---|
| BeginY | 1.5 | 8.75 | Different connector position or length |
| EndX | 4 | 5 | Different connector position or length |
| EndY | 1.5 | 8.75 | Different connector position or length |
| Geometry1.X2 | 1 | 2 | Different connector position or length |
| LocPinX | 0.5 | 1 | Different connector position or length |
| PinX | 3.5 | 4 | Different connector position or length |
| PinY | 1.5 | 8.75 | Different connector position or length |
| ShdwForegnd | 25 | 0 | Palette index; shadow pattern is zero |
| TxtPinX | 0.5 | 1 | Different connector position or length |
| Width | 1 | 2 | Different connector position or length |
| ThePage!ShdwForegnd | 25 | 0 | Palette index; shadow pattern is zero |

All other 56 selected rows agree, including units and formulas. Height is 0.25 in,
LocPinY/TxtPinY 0.125 in, TxtWidth 0.55555555555555558 in and
TxtHeight 0.24444444444444441 in. Both arrows are 0, both arrow sizes 2,
LineWeight 0.01041666666666667 in, shape shadow offsets 0 and page shadow
offsets +0.125/-0.125 in. Geometry has MoveTo (0, 0.125) and a horizontal LineTo
whose X equals the different connector Width (1 versus 2 in).

No differing length or delta equals the measured approximately 0.04 in padding.
The four-pixel edge displacement is approximately 0.041667 in at 96 dpi, numerically
four times the common LineWeight, but that weight is identical in both drawings.
That coincidence does not identify a differing authored cell or establish a cause.
No skeleton term is selected for a fix from this table. The extent cause remains
open for the next brief; M3.33 sampling and the three small-image groups remain
measured, ruling pending at the M3 human gate. No ceiling has moved.

## M3.35 changed-input rerun: partial (2026-10-14)

M3.34 now compares OPC entries by their keys, with byte-only differences kept
informational. M3.31/M3.32 changed seven acceptance inputs: scratch rectangle-text,
dynamic-glue, dropped-master, metric-background-layers, three-masters, and shipped
PTSKST_M and SOCIAL_DISTANCING_M. The other 223 hashes are unchanged. The baseline
was not rerun. The 3,600-second changed-input run completed six files, then timed
out while dumping SOCIAL_DISTANCING_M. A single-file retry uses the same budget.
The original baseline took 62 minutes 1 second for SOCIAL_DISTANCING_M, so the
budget is a material limitation; no larger budget is inferred or substituted.

The first new comparison has 229 complete verified observations, five repaired
files and 15 residual records. Every completed file has zero residual COM rows.
PTSKST_M has zero residual repairs and COM. SOCIAL remains unverified, rather
than inheriting its historical 309-row result or being counted as zero.

| Witness | Residual records | Values or cause |
|---|---:|---|
| scratch rectangle-text, dynamic-glue, dropped-master | 3 each | Page 0 ViewCenterX 0 to 4.2461539707522; ViewCenterY 0 to 5.500000161391; ViewScale 1 to -1 |
| scratch metric-background-layers | 3 | Page 0 ViewCenterX 0 to 4.1338583298009; ViewCenterY 0 to 5.8480972006905; ViewScale 1 to -1 |
| shipped-stencil AZURECOMPUTE_M | 3 | Writer removes the original thumbnail relationship; Visio restores its Present, Type and Target records. Original and baseline save both retain the relationship, so baseline subtraction cannot remove our addition. |

The scratch page attributes are outside the authorized document-level blank
baseline and the three first-save classes. They remain repairs. AZURECOMPUTE_M's
original root relationship targets docProps/thumbnail.emf; its removal is visible
under keyed comparison and is not a byte-only difference. No comparison exception
is introduced for either result. The three small-image groups remain measured,
ruling pending under P3-closure-5. Final retry results are recorded in M3.38.

### M3.38 closure-5 repairs (historical; closed by M3.41)

230/230 opened; all residual COM counts are zero, including SOCIAL_DISTANCING_M
and PTSKST_M after M3.31. SOCIAL's 15712 raw COM differences also occur in the
baseline. The zero-repair gate still fails on 15 metadata records in five files:

| Witness | Records | Cause |
|---|---:|---|
| scratch__rectangle-text | 3 | Page 0 ViewCenterX, ViewCenterY, ViewScale differ |
| scratch__dynamic-glue | 3 | Same three Page attributes |
| scratch__dropped-master | 3 | Same three Page attributes |
| scratch__metric-background-layers | 3 | Same three Page attributes |
| shipped-stencil__AZURECOMPUTE_M | 3 | Writer removes original root-thumbnail relationship; Visio restores it |

The page attributes are outside the scratch document-level baseline. The missing
thumbnail relationship is not a baseline addition. Neither class is waived.
The three small-image groups remain measured, ruling pending under closure-5;
M3.33 and M3.37 diagnostic findings stand with no renderer change. The first
SOCIAL attempt timed out, but the single-file retry completed within 3600s, so
no acceptance observation remains incomplete.

## Phase 4: export mapping for small images

Charles's M3 gate ruling, 2026-10-14: generated-masters, from-scratch-masters
and from-scratch-pages are informational until the mapping is derived, when
this item reopens as a required gate. Ceilings stay unchanged. Keep the M3.33
66-view table and all six candidate results in file-format-parity.md, and the
M3.37 67-cell connector diagnostic above: no differing cell explains the
approximately 0.04 in padding. No renderer change is part of M3.39/M3.40.
Saved measurements remain visible in corpus check and measured in reports.

### M3.40 relationship preservation

The writer no longer drops AZURECOMPUTE_M's original dangling thumbnail link.
The twenty shipped acceptance originals pass the keyed relationship regression;
synthetic source-byte cases cover unreferenced page-type links and explicit
background retargeting. Fresh VM residual counts are recorded in M3.41.

### M3.41 metadata closure

The historical fifteen residual records above are closed: all 230 files have
zero residual repairs and zero residual COM under the explicit M3.39 Page-view
class and preserved M3.40 source relationships. SOCIAL_DISTANCING_M and PTSKST_M
remain zero. The Phase 4 export-mapping item and its measured small-image
failures remain open, as do the informational shipped renderer errors above.

M3.42 closure status (2026-10-14): no open closure-6 blocker. Two consecutive
independent frozen passes are clean, 12 YES each. Small-image export mapping
remains a Phase 4 item with its existing 66-view table, six candidates and
connector diagnostic. The ten per-file outliers, text/evaluator limitations,
17 EMF+ payloads, compound lines and extreme-stroke cases remain documented
work; M3 does not claim those rendering gaps are solved.

## M4.0: Phase 4a carry-forward (2026-10-14)

The three small-image groups remain informational and are not worked in 4a.
The retained M3.41 measurements are: generated-masters 9 images, MAD 1.202102,
over16 2.648070%; from-scratch-masters 8 images, MAD 2.823616, over16 4.452799%;
from-scratch-pages 5 images, MAD 4.609743, over16 3.019190%. Ceilings and the
mapping gap are unchanged; these are saved measurements, not a new VM run.

Compared vectorcraft 63402451cce0d76926119cc9c0e961e7a4076b18
("Check embedded PNG/JPEG bitmap size from the header before decoding (#1716)")
with our retargeted crates/render/src/metafile/dib.rs. Both BI_JPEG and BI_PNG
already call raster::decode, which reads header dimensions with into_dimensions
before decoding, caps each axis at 16384 and pixels at 33554432, then configures
decoder allocation limits. This is at least as strict as the DIB checks
(32768 per axis, 33554432 pixels). No upstream code is ported. The compressed
DIB regression now proves both dispatch tags reject the oversized header
before attempting to decode its deliberately inconsistent pixel payload.

Other deckcraft and cadcraft drift on copied paths is not ported in M4.0:
render, text and fonts were retargeted and have diverged; the nurbs.rs
arc-length changes concern algorithms we do not call. The separately completed
PDF-1 ToUnicode port remains intact.

The shipped-dump worker remains retired at 1181/1181. No new dump or VM run.
Strict literal roundtrip is now an exact integer equality gate at 100%,
ratcheted from the M3 result 15080738/15080738. A one-cell loss fails even
when floating-point percentage rounding could print 100%. Other floors,
populations, denominators, hook lists and volatile lists are unchanged.

### P4a M4.1: document session and file commands

The engine now owns a Document, current page and selection, bounded snapshot
undo/redo history and journal. Failed edits restore the complete session. Dirty
model edits use the existing dependency-aware writer and reader before publication,
so unchanged imported V/F/U stay intact and dependent edited caches are refreshed.
The six registered commands are file.new, file.open, file.save, file.export,
edit.undo and edit.redo. Save covers vsdx/vssx/vstx; export covers PNG/SVG/PDF.

Focused tests preserve all 998 strict literal cells of the private probe after
open/save/reopen and preserve identical saved bytes after a failed edit. Parameter
bounds, history eviction, dependent-cache undo/redo, hostile command inputs and
output permissions have regressions. Independent draft review found aggregate
export-memory retention: scene construction and EMF record emission now charge
image, geometry and text storage, with small-budget regression fixtures.

CLI authoring, master drop, glue and generator acceptance remain open in the later
P4a steps. Breadth parity remains 27%; existing file and export rows retain their
current D/P scores. M4.1 does not establish P4a or M4 acceptance.

### P4a M4.2: authoring command surface

The registry now has 43 commands. Page/background relationships, layer membership,
rectangle/ellipse/line geometry, transforms, grouping, ordering, text runs, shape
data, hyperlinks, arbitrary parsed cell formulas and formatting/arrangement edits
have transactional tests. Group child scale formulas match the private rotated
group witness. DrawLine endpoint transforms stay consistent after move/resize/rotate.
The full probe diff remains open until the later glue, route and CLI steps.
Authoring rows retain their existing score until that acceptance evidence is available.

### P4a M4.3: stencil and master authoring

Stencil open/search/list, document-master creation and inherited master drops are
implemented. The BASIC_U Rectangle regression matches every local cell, formula,
unit and Inh marker of the private basic-rect witness within1e-6. Ten drops from
BASFLO_U and NETSME_U are generated for the later four-file VM acceptance.
Nested MasterShape IDs, deduplication, styles, fonts and related resources are
mapped with bounded traversal. VM repair acceptance remains pending.

### P4a M4.4

Connection rows and static glue use the observed PAR/PNT formulas and Connects parts. Target movement and undo are tested; the exact static rebuild is exercised in M4.8.

### P4a M4.5

Dynamic glue and transform triggers are route-owned evaluator hooks. The move-before session command matches all ten required move-after cells within1e-6. The full CLI diff is retained in M4.8.

### P4a M4.6

Straight route Geometry and XForm match four private witnesses. The explicit authoring_routes acceptance command remains nonzero for individual connector pixel comparisons; the frozen aggregate CI render gate is unchanged. This unmet requirement keeps P4a partial.

### P4a M4.7

commands, describe, run and convert share the engine Session. JSON schemas are listed, argv and command counts are bounded, hostile arguments have regressions, and failed sequences do not publish the final save. Corpus check is unchanged.

### P4a M4.8

Probe, static and move diffs are empty:6381,5349 and10 eligible cells respectively, using existing exclusions. Both generator scripts run in CI, including connector ink detection. Seven independent discovery findings have regression fixes. VM serialization refinements follow in M4.9; P4a pixel acceptance remains partial.

## P4a initial partial acceptance and 4b inputs (historical)

P4a is partial. Engine authoring, stencil drop, live static/dynamic glue and straight
routing are implemented. Probe/static/move cell diffs are empty. The individual
connector image checks remain failures: dynamic-glue and move-before over16
3.190144%, static-glue 3.175803%, against the unchanged 2% ceiling. Move-after is
0.736069%. The newly authored two-boxes pair also fails at 2.976338%, although
both exports contain ink between the boxes. The existing aggregate connector
gate still passes. No threshold, oracle output, exclusion or denominator is changed.
The export-mapping gap remains outside this brief under Charles's dated ruling.

The four-file acceptance profile pins each filename, source and baseline identity.
It does not replace the default M3 230-file profile or its 225-file baseline.
Generated vendor masters and all VM artifacts remain in private, ignored corpus
storage. The three exact diffs also remain private under the standing no-corpus-
commit rule, despite the brief's use of the word committed.

4b estimate: seven work items, roughly 24 to 40 engineering hours before review:
orthogonal obstacle paths; reroute after edits; walking preferences beyond the
four straight witnesses; line jumps and page routing preferences; theme.apply;
the SAN topology generator; and Visio acceptance plus closure verification.
This is an estimate, not a measured completion promise. Small-image export mapping
remains a separately scoped prerequisite for the outstanding pixel checks.

Public CI continues to run without the private corpus. The new private witness
tests skip only when the whole corpus root is absent; an incomplete present
corpus still fails. The two-boxes smoke test and corrupt-ZIP parser regression
remain self-contained. Private acceptance on this Mac runs all witness checks.

Closure record: all eight host CI gates pass, both privately and without the private corpus/fonts. The full corpus floor check passes. Two independent frozen verification passes each retain 17 YES and 3 NO; the required pixel failures prevent two clean passes. No P4a or M4 milestone closure is claimed.

## Renderer: connector stroke rendering

Charles's 2026-10-15 ruling accepts straight-route authoring on cell/geometry
equality within1e-6 and the unchanged corpus connector group ceiling. These
per-pair pixel measurements remain printed as informational until this renderer
item closes, when they reopen as required checks. No numerical ceiling, floor
or denominator changes. G8's maximum remains ten outliers; two-boxes is not
added to that ratchet because it is not a corpus-check input.

| Pair | Over16 | Status |
|---|---:|---|
| dynamic-glue | 3.190144% | informational |
| move-before | 3.190144% | informational |
| static-glue | 3.175803% | informational |
| two-boxes | 2.976338% | informational |
| move-after | 0.736069% | informational, within ceiling |

Carry forward M3.23's dark-route observation: the scratch dynamic-glue witness
had a dark route in Visio and two COM connects while its pixel comparison failed.
The historical M3 table remains intact above. P4a now has matching route cells
and geometry; this item concerns rendering the same content.

4b estimate remains 24 to 40 engineering hours before review: obstacle paths,
rerouting and walking choices, line jumps/page preferences, theme.apply, SAN
generation and VM closure. This light closure brief adds no 4b implementation
and does not reduce that estimate. The separate small-image export-mapping gap
from M3 remains open.

P4a closure completed 2026-10-15: three-platform CI green, two consecutive frozen passes with 21 YES/0 NO, seven empty diffs and unchanged four-file VM evidence. The renderer items above remain open and their per-pair checks reopen as required when those items close. No parity row moves; the 4b estimate remains 24 to 40 hours.

### P4b M4.12: route sweep and renderer prerequisites

The first P4b VM run produced 23 route witnesses and one theme witness, with
143212 cells read and zero cell errors. The variant attempt failed with Visio's
"Requested operation is presently disabled". `theme.apply` is deferred: no
variant witness. The successful theme drawing remains in corpus measurement.

The new portrait routes exposed independent X/Y canvas fitting. Export fitting
now preserves aspect ratio, using the larger fitted scale about the canvas
centre. The existing double-line witness exposed a solid stroke where the
central third should be transparent. Legacy theme slot 100 now uses the base
style and accent when variation tables are absent; an outer shadow mask uses
only input document geometry and colors. No built-in theme asset is embedded.
These fixes also exposed and corrected premultiplied raster samples being
returned as straight-alpha PNG samples.

The preview over all 228 drawings retains ten per-file outliers with the frozen
ratchet unchanged. `line/compound-1` is resolved; `route/offset-left` is an
outlier. The other nine remain `page/metric-scale`, `image/emf`,
`line/compound-2`, `line/compound-3`, `line/compound-4`,
`connector/dynamic-glue`, `connector/move-before`, `connector/static-glue`, and
`text/multiline-wrap`. Connector stroke rendering remains open. The canvas
change is not a claim that all small-image export mapping is solved.

M4.12 full corpus check passes: route23 MAD0.495426, over16 0.613902%;
theme1 MAD0.422183, over16 0.949668%; strict15104014/15104014 cells.
Ordinary SVG strokes now use native stroke attributes, retaining the shared
outline fallback for asymmetric caps. This corrects raster/SVG disagreement on
the tiny metric-scale witness without changing its comparison ceiling. Shadow
mask allocation and filter work both have explicit bounds and hostile tests.

M4.13: bounded orthogonal routing now matches the17 non-jump route sweep cases,
the original obstacle witness and all four P4a straight witnesses within1e-6.
Grid, plow, page-break and dense-connector behavior remain partial beyond the
observed cases. Line jumps follow in M4.14.

M4.14 named gap: `jump-three-crossings endpoint packing`. Visio places vertical
connectors10/12 atx2.015625/5.984375; ours at2/6. BeginX,EndX,PinX and horizontal
connector9 Geometry1.X2/X3/X6/X7 have ten differences of0.015625in. The nonempty
diff is retained. The other28 route drawings match within1e-6, including the
original line-jumps and five new jump witnesses. No tolerance was widened.

M4.15: obstacle, target and page-layout edits invalidate routes transactionally.
Obstacle-out/back and style/permeability regressions pass. WalkPreference0 is
the sweep's observed value. Values1/2 have existing P4a synthetic tests, not new
Visio route witnesses; combined3 and all other nonzero values remain unwitnessed
by this sweep. No new walking preference choices are claimed.

M4.16: `theme.apply deferred, no witness` for variants. The first sweep's
Page.SetTheme(1) succeeds; Page.SetThemeVariant(1,1,2) returns "Requested operation
is presently disabled". Part A explicitly permits deferral on either refusal.
No command is implemented from the single successful theme drawing. Witness:
private `_state/p4b/witness-probe-log.txt`, theme/builtin-1-variant failure.

### P4b routing and SAN acceptance, 2026-10-15

The route sweep matches 28 of 29 drawings within 1e-6. Named gap
`jump-three-crossings endpoint packing`: Visio places vertical connectors10/12
at x2.015625/5.984375, ours at2/6. Their BeginX, EndX and PinX differ by0.015625in;
horizontal connector9 Geometry1.X2/X3/X6/X7 have the same displacement at the
first and third crossings. Ten differences are retained privately. The named-gap
check admits only those coordinates and values, so other defects still fail.

The observed route uses 1.5 times LineToLine spacing around alignment-box
obstacles and a bounded visibility grid. BlockSize, LineToNode, PlowCode and
AvoidPageBreaks values are preserved and observed, but these witnesses did not
isolate an additional grid-snapping, plowing or page-break avoidance rule. Those
semantics remain partial, as do curved styles, one-axis permeability and dense
connector packing. Limits are60 obstacles,16384 grid nodes and2000 crossing
segments; requests beyond those bounds fail transactionally.

WalkPreference0 is the route sweep's observed value. Values1 and2 have the
existing P4a synthetic direction regressions, not new Visio route witnesses;
combined3 and all other nonzero values remain unwitnessed by this sweep. No new
walking choices are claimed. `theme.apply` remains deferred because SetThemeVariant
was refused; one successful SetTheme drawing does not establish variant authoring.

The SAN script meets the structural and ink checks:24 Switch instances,40 ISLs,
80 Connects, two layers/labels and four data properties per switch. Visio opens it
and preserves every route across1344 Geometry/XForm cells. Required closure fails:
1336 residual serialization differences,3 font COM differences, and PNG dimensions
1612x747 versus Visio1612x757. Witness rebuilds have zero residual repairs/COM.

Named SAN gaps: `san-property-row-serialization` (Visio adds Calendar, DataLinked,
LangID, Prompt and Verify, and removes literal string formulas/units),
`san-master-text-serialization` (master type541 becomes2, paragraph and text-run
normalization), `san-font-identity` (Liberation Sans face1 is coalesced into face0,
three local Char.Font values change), and `san-export-height` (10pixels).
These are failures, not newly allowed rewrite classes. The exact input and output
are retained in the private P4b acceptance record. The two VM runs specified by this brief are complete. A changed input would
require a fresh observation; this record preserves the accepted route evidence
and the failed SAN evidence unchanged.

Phase5 remains blocked on the M4 SAN gate. Planning estimate:12 to24 engineering
hours for the observed SAN serialization/render gaps and closure, then32 to48
hours for Phase5 export/MCP work and its acceptance. The connector-stroke and
small-image renderer items remain open under the standing rulings.

The full SAN residual inventory is retained privately as
`_state/m4b/san-residual-diffs.json`:1128 Property changes,6 Character,9 Paragraph,
3 Text,2 FaceName,1 MasterType and187 other records. The latter include authored
floating-point decimal spellings, inherited-zero Geometry cells, label ObjType
elision and the page LineJumpCode rewrite. Named follow-up
`san-authored-cell-canonicalization` covers those187 records. Numerical route
equality does not waive these serialization differences in the repair gate.

### P4b-closure candidate, 2026-10-15

The two original-based rebuilds have empty sheet diffs: Property3970 cells and
master4948 cells. Fresh Visio acceptance removes the Property and font COM
gaps: Property repair residual0 and all three files COM residual0. Authored text
leaves the document FaceNames unchanged unless a face is explicitly requested.
The SAN generator no longer writes its renderer substitute as a document face.
These changes remain uncommitted candidates until full CI passes.

`san-export-height` has a witnessed candidate fix: text-only label bounds reserve
a full line advance around the baseline. The retained discrepancy is at the top,
not ten rows of arbitrary padding. Fresh SAN1612x757 equals Visio1612x757;
MAD0.897574663 and over16 1.282242% pass the frozen ceilings. All40 routes and
1344 Geometry/XForm cells survive the fresh save unchanged, with80 Connects.

`san-master-text-serialization` remains open: Visio removes three redundant pp
markers from SAN and one from the master rebuild. The original Masters.Drop
witness retains pp with inherited Paragraph formatting, so its original form is
preserved under Charles's original-first ruling. A separate master-rebuild
opaque app.xml residual expands empty tags. No repair allowance is added.
Property full-row and font identity candidate fixes are not milestone closure.

G8 small-image mapping now blocks the new `data` corpus group: Property Pair
529x145, MAD5.592104/over16 3.929340%, and an additional individual outlier. The new master original adds a second
outlier at2.060690%; its group still passes. The count is12 against maximum10.
Height fit improves that Pair to0.404033/0.928229% but fails SAN at3.900% and
master at3.047%. Existing maximum fit passes SAN and the rebuilt master page.
No general mapping was derived by implementation or independent review, and no
size/aspect/per-file conditional or changed ratchet is introduced. The new
originals remain measured. The three standing informational groups retain their
status; this new data group is not added to them.

Phase5 remains blocked on M4. Estimate:8 to20 hours for the remaining marker/
metadata canonicalization and a general small-image mapping with corpus evidence,
then32 to48 hours for Phase5 and acceptance. `jump-three-crossings endpoint
packing`, unwitnessed WalkPreference behavior and `theme.apply` remain carried
as previously named. Connector stroke rendering also remains open.

Landing status: M4.19's public commit contains the recipe and observation record;
the Rust pin and M4.20..23 implementation remain uncommitted under the full-CI
rule. The private corpus preserves the candidate patch and source hashes. A
passing GitHub run of the record commit cannot validate the dirty candidate.

Candidate full CI exits1 at quick corpus: strict609674/609674 remains100%,
pinned COM98.431411% remains above98.426041%, and text gate passes. The failure
is the new data render group and12outliers against maximum10. Separate asset,
layer, source-inventory and wasm checks pass. No numerical contract is relaxed.


### P4b-closure-2, 2026-10-15

Charles's two rulings supersede the preceding closure blockers without changing
any numerical ceiling, floor, denominator or rewrite class. The204-file M3.0
pixel population is pinned by path and hash beside the COM cohorts. Every
Phase4 witness is still measured at the frozen ceilings and separately counted;
data pixels are informational until small-image mapping closes, route required.
Retained measurements classify9 historical outliers and3 Phase4 outliers:
data/property-rows3.929340%, master/from-shape2.060691%, route/offset-left3.822345%.
This differs from the brief's anticipated10/2 because line/compound-1 now passes
at1.186120%. The population is not altered to reproduce a predicted count.

Authored inherited-only pp markers are omitted, as Visio removes them from
SAN and the from-shape rebuild. Authored app.xml uses explicit closing tags
for empty elements. Imported trees and bytes keep M3.12. Full Property rows,
MasterType2/stroke-inclusive master bounds, implicit font identity and the
full-line-advance label extent remain the preserved candidate changes.
Fresh acceptance, full local CI and all three implementation GitHub jobs pass.
Two independent frozen verification passes each record 23 YES and 0 NO.

Carried gaps remain jump-three-crossings endpoint packing, unwitnessed
WalkPreference values, theme.apply, connector stroke rendering and small-image
export mapping. Phase5 estimate remains32 to48 hours after successful closure.


Closure-2 measured acceptance: the new two-file VM run and unchanged Property
observation all have residual repairs0 and COM0. SAN retains40 routes across1344
cells and80Connects;1612x757 on both sides passes0.897574663MAD/1.282242%over16.
The full corpus command exits0 with15105590/15105590 strict cells, all12 required
render groups passing, pinned outliers9/max10 and3 informational Phase4 witnesses.
`san-property-row-serialization`, `san-master-text-serialization`,
`san-font-identity`, `san-export-height` and `san-authored-cell-canonicalization`
are resolved by the canonical row/text/metadata, font and extent changes. Final
implementation GitHub checks pass, and all seven implementation commits are
pushed. Passes 5 and 6 are independently clean (23 YES, 0 NO each). M4 plan
gates are met; M4.28 records closure and receives its own final GitHub check.


## ISS-36: parallel row connector creation

Resolved: reroute preserves local endpoint literals instead of replacing them
with `F="Inh"`. The replacement reset a newly dropped connector to the master's
free endpoint and placed it inside another row's box during first-end glue.
`parallel_row_connectors_keep_horizontal_routes_in_either_order` covers all
requested row pairs and reversed failures through the engine, with two-point
horizontal geometry checked within 1e-9 in. Routing remains P for the existing
packing and unwitnessed-layout gaps; no score or numerical contract changes.
Reference: [Cell element](https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml).


## ISS-37: inherited Foreign master images

Resolved: the renderer finds ForeignData on the instance or its corresponding
master shape, resolves the relationship against that provider's part, and sends
the image through the existing decoder using the instance's transform and image
cells. Local ForeignData wins. Tests build original 8x4 BMP and PNG stencils,
drop native and resized instances through the engine, verify PNG bounds/colors,
SVG image elements and drawn PDF image XObjects, then save and reopen identically.
The master-drop row remains D; vendor-master rendering remains P for existing
fidelity gaps. Implicit inherited master children (#21) are unchanged.
References: [ForeignData](https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml)
and [Rel](https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-foreigndata_type-complextypevisio-xml).

`copy_parts` intentionally prefixes the original resource name with
`visio/imported/master<ID>/` to isolate imported resources and rewrites their
relationship targets. This explains `visio/imported/master3/visio/media/image8.bmp`.
OPC locates parts through source-local relationships rather than fixed directory
names; the VSDX Master XML part permits Image relationships, and neither cited
rule requires `visio/media/`. Existing Visio-generated PNG/JPEG/EMF witnesses
use `visio/media/image1.*`. The imported path remains unchanged.
Sources: [OPC Parts Overview](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/opc/parts-overview),
[Master XML Part](https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/10b28d2f-d32d-4b8c-96f5-dad13d32dd11).

## ISS-38: retained stencil source capacity

Resolved: the session retains up to 128 MiB of uncompressed stencil source-part
bytes and reports the requested and already-open sizes on refusal. Generated,
highly compressed zero-media fixtures verify admission above 32 MiB, refusal
above 128 MiB, aggregate accounting across separate sources, canonical-path
reuse and session preservation after refusal. Loading remains eager and bounded.
The existing stencil-open parity classification is unchanged.
Reference: [Documents.OpenEx](https://learn.microsoft.com/en-us/office/vba/api/visio.documents.openex).

## Brief A1: authored string spelling (#41)

Resolved: the writer preserves authored and edited literal spelling unless the
cell needs color or language identity encoding. This covers string cells without
U=STR and avoids a partial list of Shape Data and layer names. The reviewer
regression, broader edited-string cases, and CLI save/readback cover the fix;
existing font, palette and language cache tests remain green. String-related
parity classifications are unchanged.
Reference: [Cell](https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml)
and [Label](https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section).

## Brief A2: route ownership (#42)

Resolved: the jump pass only rewrites connectors actually routed, and plain
ObjType0 lines are neither jump recipients nor crossing candidates (W3).
Route and jump updates retain Geometry section cells. Glue preserves explicit
ObjType flags and routing policy; an explicit default policy remains a local
override. ConFixedCode2 routes retain their path while glued terminal coordinates
follow targets, including relative row coordinates (W4 and reference examples).
References: [ConFixedCode](https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section),
[ObjType](https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section),
[ConLineJumpCode](https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpcode-cell-shape-layout-section).

## Brief A3: glue bookkeeping (#43)

Copied members and connectors materialize references outside their subtree in
its original evaluation context (W2). Internal named references become numeric
references before IDs are remapped. Captured Boolean, font, color and language
values retain their serialized types. Deleted rows keep their deletion markers
and cached values when an external reference is removed. Ungroup captures values
before removing the group. Moving or arranging an end detaches its Connect rows
and resets its trigger (W1); unchanged ends retain their glue.
The orthogonal alignment case also needs the endpoint obstacle correction in A4.
References: [Cell references](https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references),
[Masters.Drop](https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop),
[Ungroup](https://learn.microsoft.com/en-us/office/vba/api/visio.shape.ungroup).

## Brief A4: endpoints inside obstacles (#44)

Resolved: routing exempts an obstacle containing an endpoint so the route can
leave it. At an obstacle boundary, only clearance normal to the attachment edge
is removed; the interior remains forbidden. Dynamic endpoint selection uses the
same clearance calculation. Other obstacles retain their clearance, verified by
a mixed-obstacle route test. Master drop, duplicate, move over a glued end,
background glue and A3's orthogonal alignment case succeed without swallowing a
routing error.
Reference: [LineToLineX](https://learn.microsoft.com/en-us/office/client-developer/visio/linetolinex-cell-page-layout-section).

## Brief A5: readable text in flipped shapes (#45)

Resolved: the shared scene text transform counter-flips about the text block's
center for FlipX, FlipY and both, matching W5. PNG, SVG and PDF consume the same
corrected text transform. Regression tests compare all three flip combinations
with unflipped text, including rasterized SVG and emitted PDF scene transforms.
References: [FlipX](https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section)
and [FlipY](https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section).

## Brief A X-1: missing-shape sheet errors

Resolved: sheet lookup checks shape existence before looking up a cell. A
sheet.set request for a missing shape now reports `shape not found`, consistent
with shape.resize. A regression asserts the exact operation error.

## Brief B0: evaluated string spelling (#41)

Resolved: section-aware cached-value interpretation preserves text in declared
string columns even without U=STR. Shape Data Label, Prompt, Format and SortKey,
layer names, and the other text columns retain numeric-looking spelling through
`sheet.get`. Numeric columns and Property.Value keep their existing interpretation.
The five-value CLI repro asserts both value and cached. String column, error,
length, inheritance, alias and EVALCELL regressions cover evaluation. No cap or
parity score changes.
References: [Label](https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section),
[Layer cells](https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-layer-sectionvisio-xml),
[Type](https://learn.microsoft.com/en-us/office/client-developer/visio/type-cell-shape-data-section).

## Brief B1: reader memory (#46)

Reader XML now streams into the retained tree, uses compact attribute storage,
and transfers completed trees and part buffers without full copies. The node
budget includes attributes and text. Dense XML, media-copy and six-stencil
regressions cover the memory contracts; the stencil source budget is unchanged.
Reference: [Visio file format](https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx).

Brief B2 (#47): quadratic attribute duplicate checking removed; distinct names use one hash lookup each. Duplicate attributes still fail, with a portable growth-ratio regression. No per-element cap.

Brief B3 (#48): imports index each relationship part once and read repeated page/master targets once. Portable tests assert three relationship parses for 64 distinct parts, and unchanged model counts under duplicate collection/content links. No count cap added; no duplicates found in the strict corpus or local stencils.

Brief B4 (#49): EMF text decoding/shaping and dash outline expansion are checked against the existing operation budget first. Scaled portable regressions assert refusal before retained outlines; ordinary text/dashes still draw. No numerical cap or render population changes.

Brief B5 (#50): shared cell caching remains enabled in EVALCELL for argument-independent results. Runtime ARG reads prevent dependent cells from becoming cross-frame cache entries. Cold and warm15-step chains return32768; frame-specific arguments and outside defaults are retained. Context-dependent entries preserve their existing outside-frame cache, and cache-hit metadata propagates to cold callers. No cap changes.

Brief B6 (#51): master contact sheets prepare one theme host and evaluator for the complete stencil. A portable structural regression counts one evaluator for eight masters, preserving the existing blank/error report semantics. No limit changes.
