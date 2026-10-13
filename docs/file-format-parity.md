# File format parity

Status: pre-alpha, M1 met. M2 human gate accepted on 2026-10-12 at 16:45; independent closure verification has two consecutive clean passes.

## Measured

M3.5 (2026-10-13) is partial: all 1,387 packages preserve all 15,080,738
compared local cells (100%). Successful original/rewrite raster pairs are identical,
but 568 master and 74 shipped-page scenes fail rendering. The 230-file VM run has
225 completed statuses (224 opened/saved, one corrupt-open rejection) and timed out
on SOCIAL_DISTANCING_M. A harness sharing violation stopped every completed save
before ZIP inventory and PNG export; repair and re-export parity are unmeasured.
No zero-repair claim is supported. Fresh COM coverage (413 completed shipped dumps)
yields 98.609090%, below the unchanged 98.624100% floor. Generated render groups
still pass their frozen ceilings. These failures keep M3 open.


M2.9 closure verification: pass 1 found the aggregate caller-built sheet budget
bypass, fixed with a failing-before/passing-after regression in M2.9.5. Passes 2
and 3 each have ten YES and zero NO at 3dc3b21. Final P2 acceptance passes all
six commands, including all eight mandatory CI gates. The renderer mutation
harness passes 1,387 original packages and 5,548 mutations without a panic.
All 24 paired sheets were regenerated and a separate drawing-crop scan confirms
1,108 ink-bearing cells, zero blank. Generated measurements below are unchanged
to six decimals. M2 is closed; Phase 3 is in progress; Visio acceptance of writer output is pending.

M2.9.4 aggregation disclosure: group MAD is the unweighted arithmetic mean
of scored per-file RGB MAD. The differing-pixel fraction is weighted by the
number of compared pixels in each file's top-left common area. Every hard size
mismatch or render error still fails its group. `corpus check` prints all204
per-file rows and the number individually above either of the group's pixel
ceilings. P3/M3.0 adds a count ratchet: more than ten individual exceedances
fails the report. The frozen group gate and pixel ceilings are unchanged.
10of204files individually exceed a pixel ceiling: connector
`dynamic-glue`, `move-before`, `static-glue`; image `emf`; line `compound-1`
through `compound-4`; page `metric-scale`; text `multiline-wrap`. Their group
counts are3,1,4,1,1respectively, zero in the other five groups. Regression: `per_file_disclosure_does_not_change_group_aggregation`.


M2.9.3 memory guards: image header dimensions are checked before pixel decoding;
32Mi pixels and16384pixels per axis remain the existing bounds. Streaming XML
preflight caps each section at16384cells and16384rows, each sheet at65536own
cells and16MiB including nested XML. Caller-built sheet trees receive the same
cell/row bounds and a16MiB decoded-content budget before cloning. The largest
observed shipped section has4243cells; largest XML part is5281894bytes.
`geometry_section_peak_memory_is_bounded` rejects100000rows at11075584bytes
peak RSS; `oversized_image_peak_memory_is_bounded` rejects a10000x10000PNG
header at3162112bytes. Measurements use macOS `/usr/bin/time -l` child processes;
these are hostile-fixture measurements, not maximum memory for every document.
Independent closure verification found a caller-built aggregate gap: five valid
16384-cell sections were accepted even though the XML path rejected the total.
M2.9.5 accumulates direct, section-level and row-level cells across the sheet
before cloning. `caller_built_sheet_cannot_split_cell_budget_across_sections`
fails before the fix and passes after it; the65536-cell limit is unchanged.


M2.8: Charles accepted the human gate on 2026-10-12 at 16:45. The original
pixel ceilings are frozen: non-text group MAD at most 4/255 and at most 2% of
pixels over 16/255; text MAD at most 8/255 and at most 5%. All ten groups pass.
Four raw dimension differences remain within the authorized one-pixel allowance;
zero hard size failures. All 204 files receive a pixel score over their common
area. MAD below uses RGB units 0..255. Oracle images are unchanged.

| Group | Files | Raw size differences | Tolerated | Hard failures | Mean RGB MAD | Pixels over 16 | Gate |
|---|---:|---:|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 0 | 0 | 0.543633 | 1.071225% | PASS |
| basic | 1 | 0 | 0 | 0 | 0.443949 | 0.745332% | PASS |
| connector | 6 | 0 | 0 | 0 | 2.142340 | 1.225363% | PASS |
| fill | 44 | 0 | 0 | 0 | 1.112028 | 1.373298% | PASS |
| group | 1 | 0 | 0 | 0 | 0.690956 | 1.251292% | PASS |
| image | 3 | 0 | 0 | 0 | 1.602828 | 1.731289% | PASS |
| line | 37 | 0 | 0 | 0 | 1.076040 | 1.030076% | PASS |
| master | 1 | 0 | 0 | 0 | 1.608085 | 1.791412% | PASS |
| page | 3 | 0 | 0 | 0 | 3.649380 | 1.344992% | PASS |
| text | 16 | 4 | 4 | 0 | 3.547545 | 3.193997% | PASS |

All 204 SVG/resvg and PDF checks pass, including extractable text in all 16 text
PDFs. Ligature extraction and complete reading order remain separate gaps.
M2.9.1 corrects the old successful-call count: only465/1108 contact cells
had drawing ink before the fix (643 blank). After the fix,1108/1108 have
drawing ink, zero blank and zero errors across24stencils. Names and failure
markers are excluded from this count. Scene warnings reach stderr. Paired
Cisco and Kubernetes sheets were regenerated and visually checked. Private
observations are not redistributable assets. Previous performance observations:25891ms for the largest
43-master stencil,479ms for500shapes. Budgets remain unfrozen.

Render comparison is mandatory in CI again. The one-pixel rule is tested in
`one_pixel_common_area_preserves_rows_and_rejects_two_pixels`; frozen limits are
checked by `ceilings_match_charles_frozen_record`. Independent closure passes 2
and 3 are clean at revision 3dc3b21. Pass 1 found the caller-built aggregate
sheet budget gap; its regression and correction remain recorded. Phase 3 is in progress; writer acceptance remains pending.

Measured completed coverage:1387packages,1181shipped,301completed shared dumps,
514TSVs. The shared directory has302dump artifacts including one partial.
COM text population increased from 7724 to 14118 as coverage grew.
Oracle1 pre-promotion:4627375/4716091 (98.118866%). Promoted:
4663847/4783865 (97.491192%), new floor 97.4911%. Charles authorized rebasing the
floor to its new denominator on 2026-10-12, preserving the97% M1 contract; it may
only rise from here. Oracle2:20806040/21055732 (98.814138%), unchanged floor98.6241%;
non-literal COM3165694/3199229 (98.951779%). Theme remains included.
Text cached:61710/67797 (91.021727%); COM:12807/14118 (90.713982%). Both exceed90%,
so text-metric cells now join both main denominators permanently. M2.9.2
records `promoted = true`: either text line falling below90% or its own ratchet
fails the gate, without removing text from the main denominators. Cached text
floor90.9875% is the lesser achieved quick/full rate; COM text floor90.7139%
is the lesser achieved quick/full rate. Main floors are unchanged.

Remaining text failures:6087 cached and1311COM. Cached buckets:1742User.Width,
1124TxtWidth,1054TxtHeight,746Width,262User.MenuItemTextSize,198Height,
26User.DirArrowOffset; other cached cells935. COM User.Width658, Width231,
other cells422. These are cell buckets, not uniform root causes. Overlapping
error buckets include309cached TxtHeight #VALUE!,207TxtWidth #VALUE!,108
User.TextHeight #VALUE!, and96TxtHeight unsupported FORMAT locale 2057.
Font identity, compatible Segoe metrics, other date pictures and master-specific
scale handling remain open. Cell comparison tolerance is unchanged.

M2.5 writes PNG, SVG and vector PDF for all 204 generated drawings. This is output coverage, not visual acceptance. PDF embeds the OFL faces and uses native variable coordinates for Arimo. Page/layer/background tests pass; pixel dimensions and appearance remain the M2.6 gate.

M2.4.0 full main counts match M2.3: oracle 1 4627833/4716091 (98.128577%),
oracle 2 7633655/7695822 (99.192198%), with non-literal 578894/579660 (99.867854%).
Text-only cached is 25010/67797 (36.889538%); text-only COM is 683/3625
(18.841379%), deferred. The M2.4 empty-line regression now matches the connector
master's 0.2444444444444444-inch height; full text fidelity remains unaccepted.

P2-followup embeds 18 Latin face files from google/fonts commit `bd8f81ddb5`.
System scanning stays disabled. The public checkout no longer needs craft-fonts for
Latin shaping. Arimo variable Regular, Bold, Italic and Bold Italic are shaped and
outlined directly by the existing HarfRust/Skrifa path; no static conversion is needed.

| Visio family | Embedded face |
|---|---|
| Calibri, Calibri Light | Carlito |
| Cambria | Caladea |
| Arial, Helvetica | Arimo |
| Times New Roman, Times | Tinos |
| Courier New, Consolas | Cousine |
| Unavailable Latin family, including Segoe UI in this checkout | Carlito |

Before promotion, the P2-followup rule required 90% in each text oracle and
deferred text-family formulas from the main denominators. Promotion is now
permanent: a later text regression fails its gate and cannot shrink the main
denominators. Theme remains included. The M2.9.2 ratchets above are mandatory.

The earlier M2.4 measurements and missing-font report below are historical.

M2.4 uncommitted partial run: oracle 1 is 4552375/4783865 (95.161026%), below
97.5951%; oracle 2 is 7630170/7699447 (99.100234%), with its non-literal subset
575409/583285 (98.649717%). Text-only COM is 734/3625 (20.248276%); text-only
cached is 28225/67797 (41.631636%). The three text functions are included.
Those results failed the earlier attempt; the followup commits above supersede that state.

### Font substitutions observed in M2.4

The configured craft-fonts checkout lacks its Latin manifest. The loader's first-choice
substitutes are listed separately from the actual face selected on this machine.
No vendor fonts were loaded, copied or embedded.

| Visio family | Intended open substitute | Actual local face |
|---|---|---|
| Calibri, Calibri Light | Carlito | BIZ UDPGothic |
| Arial | Liberation Sans | BIZ UDPGothic |
| Times New Roman | Liberation Serif | BIZ UDPGothic |
| Segoe UI | Inter | BIZ UDPGothic |
| Symbol, Wingdings | Per-glyph open fallback | BIZ UDPGothic |

`diagramcraft-text::tests::report_default_font_substitutes` records the selected faces.
The missing Latin pack is a resource blocker under the read-only sibling rule. Empty
text, exact line metrics, dynamic field formatting and aligned tabs also need evaluator
work. Font availability alone is not asserted to fix the measured discrepancies.

### Historical M2.3 measurements

M2.3 full run with all eight theme functions included: oracle1 shipped4627833/4716091
(98.128577%); oracle2 7633655/7695822 (99.192198%), with non-literal578894/579660
(99.867854%). Theme-only COM424210/424740 (99.875218%); theme-only cached282245/307477
(91.793858%). The first quick run after inclusion was85.928890% resolved, recovering
to99.038099% before the full sweep. Floors remain97.5951% and98.6241%.
Coverage remains1387packages,317TSVs,104/1181shipped dumps. Text metrics remain hooks
until M2.4. These measurements establish evaluation coverage, not rendered fidelity.


M2.0 strict-denominator final run: oracle 1 shipped 4302905/4408935
(97.595111%); oracle 2 7209425/7271082 (99.152024%), including its non-literal
subset 154666/154920 (99.836044%). This run includes 1387 packages and 317 TSVs,
with 104/1181 shipped dumps. Theme/text hooks are still excluded at M2.0.
Oracle 1 now excludes No Formula and applies hook/volatile filters to the inherited
effective formula. Charles authorized recording the corrected floor, at least 97%,
instead of retaining the rate from the old denominator. The empty V/empty Text
comparison is narrow and leaves nonempty strings and errors strict.

M2.0 quick run: shipped oracle 1 108652/109327 (99.382586%); oracle 2
2467451/2494966 (98.897179%), giving 6811 additional matches over M1's quick run
without reducing the 98.6241% resolved floor. The basic-rectangle page-1 dump has
5020 eligible rows, 728 exclusions and zero differences, including all units and row
identities. Explicit U=IN is code 65; absent units use the cell/context default, with
master drawing units from its PageSheet. Connection rows expose all fourteen COM
columns; Actions include omitted reserved columns.

The following Phase 1 measurements are retained as historical results on the old denominator.

Phase 1 full comparison after accuracy fixes: 1387 packages and 317 TSVs,
including 104/1181 shipped dumps. Oracle 1 shipped: 5247024/5355260 (97.978884%).
Oracle 2: 7181152/7271082 (98.763183%). The original requirements were 97% and 90%.
Quick CI uses all 204 generated drawings, both blank packages and 50 shipped packages:
Oracle 1 shipped 141932/142654 (99.493880%); Oracle 2 2460640/2494966 (98.624190%).
Shared floors now ratchet to 97.9788% and 98.6241%, the lower quick/full measurements
rounded down. Exact achieved counts are recorded in crates/sheet/tests/floors.toml.
The first full measurements were 91.411080% and 97.553995%; no oracle values,
exclusion lists or denominator rules were changed to reach the current rates.
The probe CLI dump matches all5344 eligible values and units (1049 frozen exclusions),
with6393 total rows and no missing/extra identities. Formula spelling is reported as source
expression or inherited expression; value comparison does not require identical whitespace.

Phase 1 grammar syntax sweep: 355806 distinct formulas across 1387 files, zero parse failures
(excluding only the `Inh` and `No Formula` storage sentinels). This is syntax coverage, not
numeric oracle accuracy. Local and cross-scope cell evaluation, explicit units, document
palette/font lookup and bounded error/cycle values have unit tests. Master/style inheritance,
104 implemented function names and numeric oracle rates are measured below.

The 104 shipped dumps were produced by the non-resumable staged corpus/_vm/dump-shipped.ps1,
not the current committed script; the interrupted TSV was renamed to .partial manually.
The committed resume path is verified only by tools/tests/shipped-resume.ps1 until its first VM run.

Visio 16.0 build 20430 (16.0.20430.20146). Generated corpus: 204 drawings,
827857 ShapeSheet rows, all six requested exports; PNG densities 96 dpi. Blank: 3385 cells.
Shipped source files: 1181, with 104 completed master dumps; remaining dumps deferred.
Visio acceptance verified on 2026-10-12 for basic/0001-probe.vsdx and blank/blank.vsdx:
both OPEN OK and ACCEPT OK, 9257 cells, zero errors, with PNG, sheet TSV and part list each.
Normal VISIO.EXE launch and GetActiveObject attachment resolve the earlier automation-launch
open hang; its underlying cause remains unknown. The shared-route health probe completed
in 7.822 seconds, including shipped-template OpenEx, blank Add, both closes and Quit.
ENTITY_M/U.vssx and OBJREL_M/U.vssx use the published 2011/1/core XML namespace;
the other measured files use 2012/main. Container inspection accepts both exact URIs.

## Observed, undocumented

M3.7 thumbnail decision (Charles, P3-closure): authored skeletons omit both the
thumbnail part and its relationship. The read-only private variants
corpus/_state/m3/diag-variants/v0-original.vsdx, v2-noconnector.vsdx and
v3-nopagerels.vsdx retain the authored 2x2 PNG and fail open with0x86db04fb.
v1-nothumb.vsdx opens (1page,1master,3shapes), isolating the PNG as the cause.
The private scratch rectangle without a thumbnail also opens. Existing thumbnail.emf
parts remain opaque on round-trip. Test: five_documents_build_and_reread asserts
no thumbnail part or relationship in all five outputs. Visio may add a thumbnail on
its subsequent save; that remains a disclosed part-list difference under the repair
contract, not an exclusion.


M3.2 writer witnesses: `ASSOCIATIONS_UML_CLASS_M.vstx`, page 1, group 242,
contains two deleted inherited members with the sentinel ID 4294967295 and
MasterShape 7 and 9. Preserve both tombstones; ordinary duplicate shape IDs
remain invalid. `AWSANALYTICS_M.vssx` has a root thumbnail relationship to
`docProps/thumbnail.emf` without that part. The writer omits this dangling
optional relationship, reports it, and fabricates no preview. Host tests cover
both cases. `BASIC_U.vssx` also carries a PageSheet-only page entry without
a Rel or PageContents part; the writer preserves that absence until the page
contains drawing content. Historical M3.5 observation, superseded by the M3.7 diagnostic above: missing
and authored PNG thumbnails had structural round-trip tests. In the M3.5 run, scratch__rectangle-text.vsdx without a thumbnail
opened and saved. The authored PNG case, scratch__dynamic-glue.vsdx, was rejected
as corrupt. Because that file also exercises a new connector, the cause is not
isolated. Harness failures prevented repair and render checks for either case;
missing/PNG thumbnail acceptance was then unresolved.


M2.9.1 shipped contact witnesses: CISCONETWORKSHAPES_M/U each had289blank
cells out of292, KUBERNETESVISIOSTENCIL_M/U each31out of40,
AWSANALYTICS_U one of28, and AWSNETCONTENTDELIVERY_M/U each one of36.
The other17stencils had zero blanks; all24 now have zero blanks.
Cisco master0 image1.emf has a116-byte EMR_HEADER with a description at88.
Treating its UTF-16 description as extension2 micrometre fields incorrectly
moved MM_ANISOTROPIC content outside the frame. Follow the Microsoft
[header-type algorithm](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/de081cd7-351f-4cc2-830b-d03fb55e89ab)
and normalize the imported frame origin before placement. This is an
implementation correction, not a contradiction of the reference.

Kubernetes master45 shape16's NoShow switch correctly evaluates true; the
regression preserves that hidden variant. Visible white Geometry sections
must form a compound fill with holes, rather than separately painting each
contour over the icon. The same cause explains the three AWS blanks:
master33 Analytics and master59 Networking and Content Delivery in both
metric and US stencils. Group DisplayMode now follows the documented
hidden/behind/in-front order. Tests:
`shipped_cisco_kubernetes_and_aws_have_drawing_ink`,
`base_header_description_is_not_optional_micrometre_dimensions`,
`anisotropic_window_and_viewport_origins_map_once`,
`group_display_mode_hides_or_orders_group_paint`, and
`labels_do_not_turn_blank_masters_into_rendered_masters`.


### Followup master failures

Contact scenes are fitted before allocating a raster: all 578 Cisco masters now fit. Empty text blocks need no layout, resolving the three Azure Compute empty zero-width blocks and NETLOC Cloud in both unit variants. NURBS precision and empty-text fixes recover another seven masters. BASFLO_U Custom1 (master 11, shape 4) remains: Height depends on User.ResizeTxtHeight, which adds Scratch.Y1 = MIN(Height/2, Scratch.X1). This direct cycle needs a principled cyclic-evaluation rule. No cached geometry replaces the error.

### Foreground export selection and drawing scale

`generated/page/background.vsdx` stores background page4 before foreground page0. The default PNG/SVG export now selects the first foreground page and composites its BackPage chain, as the generator does. The earlier export showed only page4. `generated/page/metric-scale.vsdx` has PageScale/DrawingScale 0.01; its uncropped physical page is 0.085 by 0.11in, matching its Visio SVG. Layer Visibility already excludes the Hidden rectangle in `generated/page/layers.vsdx`; the locked visible rectangle remains. The metric-scale cropped text height remains two pixels short.

### Repeated NURBS knots

NETSME_U Lock, master 6 shape 6, stores Geometry1.A3 as
0.7138452034029296 and the matching repeated knot in Geometry1.E3 as
0.71384520340293. Both represent the same knot at different serialized precisions.
The spline parser now coalesces descending representation noise within 16 machine
epsilons, while a genuinely descending knot still fails. Lock and Lock and Key in
both NETSME unit variants render. This changes geometry arithmetic, not an oracle
comparison tolerance.

### Composed text metrics

An empty TEXTHEIGHT block has one formatted line independent of wrapping width.
ADS_M Site link 3D has zero text width but a valid empty-line height. BASFLO_U
Database, External Data and Custom3 have width expressions that depend on height;
avoiding an unnecessary width evaluation resolves those three cycles. Custom1 has
a direct geometry cycle outside the function and remains an error.


Nonempty composed text has an additional 0.00005 inch of height. Empty blocks do not.
Witness: AWSANALYTICS_U master 2 shape 5 TxtHeight is 0.15005 inch in Visio,
previously 0.15 here. The basic probe page 0 shape 5 is 0.2444939358181424 in
Visio and now 0.24449444444444446 here, within the unchanged 1e-6 comparison
tolerance. Its empty master remains 0.2444444444444444. This slack is an observed
addition not mentioned by the TEXTHEIGHT reference, which includes line spacing and
margins. Width and wrapping discrepancies remain. SHAPETEXT now removes one complete terminal
CRLF marker rather than leaving a stray CR, witnessed by ASSOCIATIONS_UML_CLASS_M
page 7 shape 229 User.MemberName. Text metrics convert physical points back to drawing
inches: APPL_M master 2 shape 6 TxtWidth is 11.11111111111111 at 1:100, and APPL_U
master 25 shape 5 is 10.66666666666667 at 1:96. The previous implementation returned
0.1111111111111111 for both. Master rendering also applies its page scale before
contact-cell fitting.

For percentage line spacing, baseline placement uses OS/2 typographic ascender and
descender proportions. In text/align-0-0.svg, Visio puts the first 12pt baseline at
14.8pt after the block origin, including the 4pt top margin. The previous clipping
metrics placed it at 15.232pt. The regression now matches 14.8pt.


P2-followup-2 stroke profiles, row30 x0..2: basic/0001-probe is [255,0,255]
in Visio, previously [127,127,255] here. The master/basic-rect0.24pt edge is
[191,64,255] in Visio, previously [41,214,255]. A one-device-pixel origin and
an eighth-pixel edge lattice reproduce both exactly. A0.24pt line is0.32pixels
nominally at96dpi but paints with a one-pixel minimum. This is not a global blur:
the0.75pt probe edge is one full black pixel. Group results retain regressions.

BASIC_U.vssx Drop master56 shape7 LinePattern evaluated23 instead of solid1.
The theme contains six DrawingML line styles but seven Visio extension entries:
slot zero is the no-style entry. Indexing both tables at matrix-1 selected the
wrong lineEx. Use matrix for the extension and matrix-1 for DrawingML. The
paired Drop/Semi Circle/Gear exports witness solid outlines; no corpus data changes.


P2-followup-2 export bounds: use the union of logical geometry, foreign-object frames,
marker outlines and text extents, then ceil(width * dpi) and ceil(height * dpi), with
only a 1e-7 floating-point cancellation allowance. Page size is used for an empty drawing
or uncropped export. Logical stroke bounds use the resolved physical LineWeight;
minimum device stroke width is a painting concern. A filled marker suppresses the
line cap at that endpoint. The observed arrow scale is weight * (1.5 + 0.375 * size).
The filled endpoint circle (42) lies inside its tip. Square open-line caps have logical
radius weight / sqrt(2); closed contours retain weight / 2. Witnesses: all 92 arrow
files and all 37 line files now match dimensions, including begin-02 (482x397),
begin-42 (482x394), and cap-2 (492x394). These corrections resolve 25 of 35 size
failures. Nine text files and metric-scale still require logical text-bound work;
no constant padding or per-file size override is used.


Historical M2.4 text witnesses (superseded by M2.4.3 below): generated/basic/0001-probe.vsdx master 2 shape 5
TxtHeight now matches Visio 0.2444444444444444 inches. Page 0 shape 5 is now
0.24449444444444446 versus Visio 0.2444939358181424. ADO_M.vssx master 2
shape 45 TxtWidth remains 0.3479410807291667 versus Visio 0.3668267852610678.
These are implementation discrepancies, not reference exceptions or tolerance changes.

Generated fill/pattern-25.vsdx shows a legacy gradient with gamma-like midpoint
near 186 for a black/white ramp, rather than linear 128. The procedural implementation
uses a Gaussian blend and gamma 2.2 based on the observed samples; exact parity remains
measured by the group gate. Pattern-02 diagonal coverage has a dark center and two
partially covered neighboring pixels. fill/gradient-radial.vsdx uses a bottom-right
focus with physical aspect ratio. These black-box observations do not establish a
contradiction of a documented reference example.

Remaining render bounds: page/metric-scale.vsdx is 77x32 versus
77x30; text/align-0-2.vsdx is 387x291 versus 385x289. Text logical export extents remain unresolved; marker, cap and physical stroke
corrections below recover the other generated dimensions.


Historical P2-followup font-only check (implementation discrepancies, not established reference
exceptions): in `corpus/generated/basic/0001-probe.vsdx`, master 2 shape 5 TxtHeight
remains 0.1111111111111111 versus Visio 0.2444444444444444 inches; page 0 shape 5
TxtHeight becomes 0.2738715277777778 versus Visio 0.2444939358181424. The former
lacks an empty-text line; the latter applies natural font height to Visio's percentage
spacing. In `corpus/ms-shipped/1033/ADO_M.vssx`, master 2 shape 45 TxtWidth improves
from 0.427490234375 to 0.3479410807291667, versus Visio 0.3668267852610678.
These remain M2.4 metric work, not reasons to change the oracle or its tolerance.


`ACCESSBILITY_NEW_M.vssx`, master3 shape6, LineColor and FillBkgnd use
`IF(ISTHEMED(),THEMEVAL(),RGB(0,0,0))`. With the inherited theme selector 65534
and no page theme, Visio takes the false branch and returns custom black (palette25).
The [ISTHEMED reference](https://learn.microsoft.com/en-us/office/client-developer/visio/isthemed-function)
describes a simple positive-value test, but the page-inheritance sentinel must be
resolved first. The implementation follows the corpus observation.

In generated `arrow/begin-00.vsdx`, TextBkgnd stores V=`#ffffff`, mapped to palette25,
while its COM ResultIU is26. TextBkgnd's RGB cache conversion includes the cell's
one-based palette offset; ordinary fill and line color caches do not.


The [MSOTINT reference](https://learn.microsoft.com/en-us/office/client-developer/visio/msotint-function)
parameter text reverses the observed sign. In HIERARCHICALORGCHART_U.vstx page1
shape1001, Char.Color evaluates `MSOTINT(RGB(0,0,0),35)` to #595959;
LineColor evaluates `MSOTINT(RGB(255,255,255),-25)` to #bfbfbf.
Positive percentages therefore move toward white, negative toward black, matching
the page's description. MSOSHADE reverses that direction. The implementation
follows these witnesses and never substitutes the cached result for the calculation.

Integral HSL arguments use the observed 240-step integer interpolation.
AZUREDRAWING_M.vssx master87 shape5 ShdwForegnd stores `HSL(144,116,125)` as
palette28, #4979C0. Continuous interpolation previously produced #4A79C0.
The integer conversion also matches DTLNET_M.vssx master15's `HSL(21,165,128)`
value #DA8C36. An independent serialization survey matches all 190 literal HSL
cached colours; no fractional inputs occur in that set. Fractional HSL remains
on the continuous path, with no claim that its quantization has been observed.

Cached error projections and a stale language result are observed, not treated as
computed answers. In `ANNOT_M.vssx`, master 12 shape 6 (part `master10.xml`),
Scratch.X2 and Scratch.Y2 contain string formulas with E=`#VALUE!`, while V and COM
ResultIU are zero. In generated `image/emf.vsdx`, ClippingPath has formula `""`,
empty V and E=`#N/A`; COM reports ResultIU zero with units 224. These zeros do not
justify replacing computed error values with cached numeric values. The Scratch
reference classifies X/Y as drawing-unit cells and A-D as variants; ClippingPath
is a geometry reference, with an empty or invalid value selecting a rectangular clip.

In `APPL_M.vssx`, master 2 shape 6 LangID (part `master1.xml`) stores F=`Inh`,
V=`en-IN`. Its completed COM dump reports FormulaU=`1033` but ResultIU=`16393`.
The assigned style chain resolves the formula to en-US (1033). The implementation
keeps formula evaluation at 1033; it does not replace that result with the stale
cached language value. The mismatch remains counted by the oracle.

Host-hook fallback uses the cached final cell value, not that value as an inner function
result. ALARM_M.vssx master2 User.HasText is `LEN(SHAPETEXT(TheText))>0`, cached 0;
using 0 as the SHAPETEXT string incorrectly makes LEN nonzero and changes dependent
Controls.Row_1.YCon from Visio's 7 to 2. Frozen hook-containing formulas now retain the
whole cached result and register syntactic reference dependencies. Parsing still runs;
quoted strings/reference names and ordinary unsupported functions cannot select fallback.
This is declared cached host behavior, not proof that theme/text functions are implemented.

Internal ellipse helpers use an independently derived affine covariance transform and a
deterministic axis representative. Their exact historical branch behavior remains partial.
Identical stored inputs can have different cached outputs: EXCOBJ_M.vssx master12 shape8,
Geometry rows 3 and 6 have identical theta inputs but cache approximately -pi and 0.
BPMN_ADDRESS_CHANGE_PROCESS_M.vstx master2 shape19 and BPMN_M.vstx master6 shape19
have identical ECC inputs but cache reciprocal ratios 1.004656577415021 and
0.9953650057943116. The implementation does not read these caches or special-case files;
remaining numeric mismatches count as failures. This is an unexplained stored-input
ambiguity, not a claimed contradiction of a published internal-function specification.

Cached values can remain numeric after an error: ARROWS_M.vssx, master2.xml shape5,
Scratch.A1 has `(Controls.Row_4.Y-Height/2)/(Controls.Row_4-Width)`, `E="#DIV/0!"`
and stale `V=1.3336`. Controls.Row_4 equals Width. The evaluator returns division by zero.
This remains an Oracle1 failure on the required denominator; neither E cells nor inherited
volatile cells are silently excluded.

The internal `_MOD` spelling returns a nonnegative magnitude for negative dividends:
ADS_M.vssx master7.xml shape6 Scratch.A1 evaluates `_MOD(Angle,360DEG)` with Angle=-pi/4
and caches +pi/4. ADS_U and CONNEC_M/U provide six matching negative-angle witnesses.
The implementation uses absolute truncating remainder. Negative divisors lack a local
witness. Legacy MOD is accepted as a compatibility alias of the observed internal spelling.
Only `_MOD` is corpus-witnessed; the alias relationship remains an explicit compatibility
assumption. MODULUS retains its separately documented divisor-sign semantics.

GRAVITY implements the reference's order-independent normalized angular interval.
Exact boundary angles use an open interval pending an explicit boundary witness.

COM style enumeration differs from the XML: `corpus/generated/basic/0001-probe.vsdx`
contains style gradient sections and No Style's LayerMember, but its completed
`.sheet.tsv` has no corresponding style SRC rows. The blank style dumps agree.
The TSV adapter follows the observed Style.Section/CellsSRC surface while preserving
these XML cells in the model and evaluator. Shape gradient sections remain exposed.

Theme selector inheritance witness: the probe's Theme style 6 explicitly contains
65534 for all eight selector cells while No Style 0 contains 0. Intermediate styles
and shapes resolve to 65534. These cells must traverse their formatting style chains,
rather than fall directly back to No Style. Routing among conflicting style chains
is not established by this witness: font selectors use TextStyle, effect selectors
use FillStyle, common/color/connector selectors use LineStyle pending such evidence.

SETF's reference specifies assignment without a return value. The local inventory has
24575 SETF-leading formulas with cached V=0 and no E, including expressions outside
Actions and Events. A single-call witness is `corpus/ms-shipped/1033/MWFCONCTRL_M.vssx`,
`visio/masters/master2.xml`, shape 23, IsDropTarget:
`SETF(GetRef(User.Font),IF(ISERRVALUE(FONT("Segoe UI")),FONT("Calibri"),FONT("Segoe UI")))`.
The evaluator performs the in-memory assignment and returns numeric zero. Source package
bytes remain immutable. ARG with neither a binding nor a default returns an explicit
value error pending evidence; SETATREF edit redirection remains the future editing path.

Arithmetic reference caveats: Microsoft's ATAN2 example prints 29.0456 degrees for
ATAN2(1.25,2.25); the defined vector angle is 29.054604099077146 degrees.
The implementation follows the defined angle. A local independent witness,
`corpus/ms-shipped/1033/ADS_M.vssx`, `visio/masters/master2.xml`, shape 6 Angle,
uses `ATAN2(EndY-BeginY,EndX-BeginX)` with deltas x=0.30654847917641903 and
y=-0.28738919922789286. Visio's cached value is -0.7531512809621943 radians;
standard atan2 gives -0.7531512809621944. This witnesses the normal angle rule,
not the exact input pair in the reference example.

The TANH reference specifies radians but prints a degree-based result for TANH(-45).
The implementation follows the stated radians rule; no TANH corpus witness exists.
Legacy MOD is mentioned on the MODULUS page but has no separate current definition or local
occurrence. Its compatibility alias to observed `_MOD` is described above; MODULUS is separate.

`corpus/ms-shipped/1033/PBASLN_U.vstx`, `visio/pages/page1.xml`, contains
`USE({GUID})` with a bare GUID literal. The Microsoft USE function page describes
a quoted string argument; the reader also accepts this observed GUID spelling as text.


Visio output contradicts the Cell_Type U table
(https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/6f23bcc4-af93-4023-a380-3e78a228e166):
it writes additional U codes absent from that table. Local, unmodified witnesses:

- `NURBS`: `corpus/ms-shipped/1033/ARROWS_M.vssx#visio/masters/master10.xml`.
- `POLYLINE`: `corpus/ms-shipped/1033/APPL_M.vssx#visio/masters/master17.xml`.
- `GUID`: `corpus/ms-shipped/1033/BASIC_GANTTCHART_U.vstx#visio/pages/page1.xml`.
- `MULTIDIM`: `corpus/ms-shipped/1033/ADS_M.vssx#visio/masters/master3.xml`.
- `NUM`: `corpus/ms-shipped/1033/EVIDENCE_U.vssx#visio/document.xml`.

## Phase 1 element checklist

D means reader coverage with tests; it does not claim a writer or renderer. P means retained but partly interpreted.
Evidence: `reads_shape_cells_sections_text_and_unknown_content`, `known_metadata_trees_and_relationship_targets_are_available`,
`macro_payload_removed_and_flagged`, and `all_corpus_parses_and_seeded_mutations_never_panic`.
All 1387 packages parse, including 5548 seeded mutations with zero panics.

| Element or attribute group | Status | Microsoft reference |
|---|---|---|
| [Content_Types] | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |
| Relationships, media and thumbnail targets | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |
| DocumentSettings and attributes | D | https://learn.microsoft.com/en-us/office/client-developer/visio/documentsettings-element-visiodocument_type-complextypevisio-xml |
| Colors / ColorEntry | D | https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml |
| FaceNames / FaceName | D | https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml |
| StyleSheets / StyleSheet | D | https://learn.microsoft.com/en-us/office/client-developer/visio/stylesheets-element-visiodocument_type-complextypevisio-xml |
| DocumentSheet | D | https://learn.microsoft.com/en-us/office/client-developer/visio/documentsheet-element-visiodocument_type-complextypevisio-xml |
| EventList | D | https://learn.microsoft.com/en-us/office/client-developer/visio/eventlist-element-visiodocument_type-complextypevisio-xml |
| Pages / Page / PageSheet | D | https://learn.microsoft.com/en-us/office/client-developer/visio/pages-elementvisio-xml |
| PageContents / Shapes / Connects | D | https://learn.microsoft.com/en-us/office/client-developer/visio/connects-element-pagecontents_type-complextypevisio-xml |
| Masters / Master: UniqueID, BaseID, Icon | D | https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml |
| MasterContents / MasterShortcut | D | https://learn.microsoft.com/en-us/office/client-developer/visio/mastershortcut-element-masters_type-complextypevisio-xml |
| Windows | D | https://learn.microsoft.com/en-us/office/client-developer/visio/windows-elementvisio-xml |
| Theme parts | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |
| docProps/core, app, custom | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |
| Media parts | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |
| Shape: ID, Type, Master, MasterShape, LineStyle, FillStyle, TextStyle, Name, NameU, IsCustomName | D | https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml |
| Cell: N, V, F, U, E | D | https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml |
| Section: N, IX, Del | D | https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml |
| Row: N, IX, T, Del | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml |
| Trigger | D | https://learn.microsoft.com/en-us/office/client-developer/visio/trigger-elementvisio-xml |
| RefBy | D | https://learn.microsoft.com/en-us/office/client-developer/visio/refby-element-cell_type-complextypevisio-xml |
| Text and cp/pp/tp/fld markers | D | https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml |
| ForeignData | D | https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml |
| Hyperlink section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-hyperlink-sectionvisio-xml |
| Property: Label, Type, Format, Invisible, SortKey | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-shape-data-sectionvisio-xml |
| User section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-user-defined-cells-sectionvisio-xml |
| Actions section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-actions-sectionvisio-xml |
| Controls section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-controls-sectionvisio-xml |
| Connection section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-connection-sectionvisio-xml |
| Geometry section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml |
| Character section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-character-sectionvisio-xml |
| Paragraph section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-paragraph-sectionvisio-xml |
| Tabs section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-tabs-sectionvisio-xml |
| Scratch section | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-scratch-sectionvisio-xml |
| Layer / LayerMem | D | https://learn.microsoft.com/en-us/office/client-developer/visio/layer-membership-section |
| Fill gradient stops | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-fill-gradient-sectionvisio-xml |
| Line gradient stops | D | https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-line-gradient-sectionvisio-xml |
| Opaque unknown parts/elements, macro removal | D | https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx |

The preliminary POINTALONGPATH sweep evaluates 0/387 end to end before inheritance:
68 fail on BOUND, 273 on NEARESTPOINTONPATH, 12 on RECTSECT and 34 on missing references.
The isolated path tests pass; these are not corpus passes or an oracle denominator change.

Inheritance construction now succeeds on all 1387 local packages, with zero omitted
non-deleted master children in those files. Seven focused tests exercise master references,
style categories, tombstones and inherited geometry. Wholly omitted master-child instances
remain unsupported outside this measured corpus. Scoped baseline constants come from the
brief-authorized blank dumps through `tools/generate-defaults.py`; authored rectangle
size, position and geometry are excluded. Baselines are distinct from oracle expected values.

The largest local stencil by uncompressed package bytes is WFDEP_M.vssx (7889047 bytes).
Initial release timing: read184ms, index378ms, evaluate360581 effective cells665ms,
115457 evaluation errors. A500-shape synthetic page indexes/evaluates in2ms and recalculates
1001 dirty cells in1ms. These are measurements, not performance or accuracy gates.

Font IDs are host-specific: the blank profile and Microsoft FONT example agree on Calibri4.
Other FaceNames currently use collision-free document-local IDs, not a complete Windows font
profile. BOUND endpoint/tie behavior remains an explicit assumption awaiting edge witnesses.
Undocumented _ELLIPSE_THETA/_ELLIPSE_ECC are partial: independently derived covariance
math does not explain every observed cache. Unmatched branches remain measured failures.

After inheritance and documented dependency functions, the original-source path diagnostic
matches275/387;54 fail on FORMULAEXISTS,56 on PATHLENGTH and2 are numeric mismatches.
An intermediate527-row effective-cell diagnostic compared instance results to master caches
and is invalid as an oracle. The formal cached oracle must use original source cells only.

## Implemented function groups

ABS, ACOS, AND, ANG360, ANGLEALONGPATH, ANGLETOLOC, ANGLETOPAR, ARG, ASIN, ATAN, ATAN2, BITAND, BITNOT, BITOR, BITXOR, BLUE, BOUND, CEILING, COS, COSH, DEG, DEPENDSON, EVALCELL, EVALTEXT, FIELDPICTURE, FLOOR, FONT, FONTTOID, FORMAT, FORMATEX, FORMULAEXISTS, GETREF, GRAVITY, GREEN, GUARD, HSL, HUE, IF, IFERROR, INDEX, INT, ISERR, ISERRNA, ISERROR, ISERRVALUE, LANGUAGE, LEFT, LEN, LISTSEP, LISTSHEETREF, LOC, LOCTOLOC, LOCTOPAR, LOOKUP, LUM, LUMDIFF, MAGNITUDE, MAX, MID, MIN, MOD, MODULUS, MSOSHADE, MSOTINT, NA, NEARESTPOINTONPATH, NOT, NURBS, OR, PAR, PATHLENGTH, PI, PNT, PNTX, PNTY, POINTALONGPATH, POLYLINE, POW, RAD, RECTSECT, RED, RGB, RIGHT, ROUND, SAT, SETATREF, SETATREFEVAL, SETATREFEXPR, SETF, SHADE, SIGN, SIN, SINH, SQRT, STRSAME, SUM, TAN, TANH, TINT, TRUNC, USE, _ELLIPSE_ECC, _ELLIPSE_THETA, _MOD.

Microsoft example and hostile-input tests are in `crates/sheet/src/functions/`.
Legacy MOD is an explicit compatibility alias to the witnessed internal _MOD operation, as recorded above. Implementation coverage is not an oracle rate.

Text formatting covers common numeric, fractional, scientific, unit and English Gregorian
pictures. Explicit non-English locales, non-Gregorian calendars, predefined Office picture IDs
and specialized era formats return Unsupported. Unit provenance for complex expressions is
conservative. Unicode scalar indexing and negative MID follow the current documented
interpretation but lack COM edge-case witnesses.

Point transforms support enclosing groups, rotations and flips. Path queries support lines,
relative lines, ellipses with perpendicular axes and elliptical arcs, with bounded arc-length
integration. Other row types and ambiguous skew ellipses return Unsupported. Stored point
values do not yet preserve a transformed coordinate-frame tag across cell boundaries.

## Functions not implemented

Inventory over all 1387 local packages: 15080738 Cell elements and 6271536 formula-bearing
cells in Visio XML parts. Counts are lexical function occurrences outside quoted string
literals. Unknown solution-data payloads remain opaque. Frozen host hooks are shown; no
unsupported function is reclassified to improve an oracle score.

| Function | Occurrences | Status |
|---|---:|---|
| `THEMEVAL` | 516850 | Host hook |
| `THEMEGUARD` | 130975 | Host hook |
| `TEXTHEIGHT` | 37991 | Host hook |
| `TEXTWIDTH` | 37189 | Host hook |
| `OPENTEXTWIN` | 33330 | Missing |
| `ISTHEMED` | 18060 | Host hook |
| `SHAPETEXT` | 10721 | Host hook |
| `RUNADDONWARGS` | 8464 | Missing |
| `SHEETREF` | 7281 | Missing |
| `MASTERNAME` | 7071 | Host hook |
| `DOCMD` | 6675 | Missing |
| `THEMEPROP` | 5021 | Host hook |
| `_XFTRIGGER` | 4348 | Host hook |
| `THEMECBV` | 4180 | Host hook |
| `_WALKGLUE` | 3068 | Host hook |
| `THEME` | 2680 | Host hook |
| `CONTAINERSHEETREF` | 1667 | Missing |
| `CHAR` | 1490 | Missing |
| `DEFAULTEVENT` | 1329 | Missing |
| `FIND` | 1168 | Missing |
| `ID` | 707 | Missing |
| `RUNADDON` | 667 | Missing |
| `REF` | 535 | Missing |
| `NOW` | 508 | Host hook |
| `TONE` | 440 | Missing |
| `LOCALFORMULAEXISTS` | 344 | Missing |
| `DAY` | 336 | Host hook |
| `MONTH` | 133 | Host hook |
| `LISTMEMBERCOUNT` | 124 | Missing |
| `DATEVALUE` | 110 | Host hook |
| `DATETIME` | 100 | Host hook |
| `LISTORDER` | 89 | Missing |
| `HOUR` | 88 | Host hook |
| `WEEKDAY` | 70 | Host hook |
| `SATDIFF` | 70 | Missing |
| `DATE` | 66 | Host hook |
| `OPENGROUPWIN` | 58 | Missing |
| `BLEND` | 58 | Missing |
| `USERUI` | 48 | Missing |
| `PAGENUMBER` | 45 | Host hook |
| `_MARKER` | 43 | Missing |
| `DOCLASTEDIT` | 42 | Host hook |
| `CELLISTHEMED` | 42 | Host hook |
| `MINUTE` | 40 | Host hook |
| `SECOND` | 40 | Host hook |
| `DOCLASTSAVE` | 30 | Host hook |
| `YEAR` | 30 | Host hook |
| `MASTERMAP` | 20 | Missing |
| `PAGECOUNT` | 18 | Host hook |
| `QUEUEMARKEREVENT` | 18 | Missing |
| `DOCCREATION` | 16 | Host hook |
| `SHAPEBINDING` | 10 | Missing |
| `CONNECTORBINDING` | 10 | Missing |
| `THEMERESTORE` | 8 | Host hook |
| `_UCON_X1` | 8 | Missing |
| `_UCON_Y1` | 8 | Missing |
| `_UCON_C1` | 8 | Missing |
| `_UCON_D1` | 8 | Missing |
| `_UCON_X2` | 8 | Missing |
| `_UCON_Y2` | 8 | Missing |
| `_UCON_C2` | 8 | Missing |
| `_UCON_D2` | 8 | Missing |
| `_UCON_X3` | 8 | Missing |
| `_UCON_Y3` | 8 | Missing |
| `_UCON_C3` | 8 | Missing |
| `_UCON_D3` | 8 | Missing |
| `_UCON_BEGTYP` | 8 | Missing |
| `_UCON_ENDTYP` | 8 | Missing |
| `_UCON_GEOTYP` | 8 | Missing |
| `_UCON_SIMPLE` | 8 | Missing |
| `RAND` | 8 | Missing |
| `LAYOUT` | 8 | Missing |
| `CREATOR` | 6 | Host hook |
| `FILENAME` | 4 | Host hook |
| `GOTOPAGE` | 4 | Missing |
| `MEMBEROFGRID` | 2 | Missing |
| `CFFBINDING` | 2 | Missing |
| `TIME` | 2 | Host hook |
| `PAGENAME` | 2 | Host hook |
| `TITLE` | 2 | Host hook |
| `DIRECTORY` | 2 | Host hook |
| `DESCRIPTION` | 2 | Host hook |

### P2-followup-3: text placement versus SVG

The 16 generated text SVGs expose text rectangles and paragraph baselines.
`text_blocks_and_unkerned_lines_match_visio_svg` checks all 16 block rectangles and
the eight line starts and baselines of `text/multiline-wrap.vsdx`. Carlito with kerning
disabled reproduces the SVG advances: centered Alignment starts at118.857422pt
(SVG118.86), and the wrapped paragraph starts at9.316406pt (SVG9.32).
The latter now uses eight lines, with baselines73.8+21.6*n points, instead of seven.
The existing 4pt margins and percentage baseline (0.9em for 120% spacing) agree with
all nine alignment drawings. TxtPin/TxtAngle transform the block after layout.

Export extents remain unresolved. SVG block rectangles and glyph ink do not explain
the extra pixels in the nine text PNGs or page/metric-scale.png. A half-em padding
experiment failed across these witnesses and was discarded, not calibrated. The
renderer still uses glyph ink and the minimum text-block height. Bullet placement
also differs (first text start132.960938pt versus the SVG bullet origin111.97pt);
these origins include different bullet handling. No oracle was changed.

### P2-followup-3: arrow marker profiles

The generated arrow SVGs give scales1.90,2.26,2.98 stroke units for sizes1,2,4.
`svg_marker_scales_aliases_and_shaft_setbacks` checks these values. Type8 aliases
type5, with a concave cubic rear edge; type39 comprises adjacent filled triangles.
The shaft stops at1.75 marker units for type5/8 and4 units for type39: end-39
size1 ends at344.8pt,15.2pt before the original endpoint. Shaft trimming uses arc
length on a single open contour, preserving closed contours.

SVG marker scales imply a397.013333px logical height for end-39, while its PNG is
397px. Rounding export extents to the same eighth-pixel device lattice used for
coverage, before taking the enclosing integer size, preserves every generated
arrow and line dimension. This is an observed raster-export quantization rule;
SVG geometry remains floating point. Pixel ceilings and oracle images are unchanged.

Vertical marker-edge samples (grey,0black/255white), y0..11:

- `end-08`, x=470: Visio `[255, 255, 255, 255, 127, 0, 0, 255, 255, 255, 255, 255]`; before `[255, 255, 255, 255, 191, 0, 0, 159, 255, 255, 255, 255]`; after `[255, 255, 255, 255, 191, 0, 0, 127, 255, 255, 255, 255]`.
- `end-39`, x=470: Visio `[255, 255, 255, 255, 255, 143, 239, 255, 255, 255, 255, 255]`; before `[255, 255, 255, 255, 184, 0, 0, 153, 255, 255, 255, 255]`; after `[255, 255, 255, 255, 255, 61, 15, 237, 255, 255, 255, 255]`.
- `begin-08`, x=10: Visio `[255, 120, 120, 183, 107, 0, 0, 199, 159, 88, 207, 255]`; before `[255, 120, 169, 251, 191, 0, 0, 159, 252, 173, 121, 255]`; after `[255, 96, 91, 173, 179, 0, 0, 151, 189, 105, 85, 255]`.

### P2-followup-3: transformed strokes and export viewport

Geometry is fitted into the integer PNG canvas with a one-pixel border: x scale
`(width-2)/logical_width`, y scale `(height-2)/logical_height`. Stroke centerlines
are transformed first; the physical pen is then expanded at96dpi, retaining the
one-device-pixel minimum. Expanding a pen before the transform incorrectly scales
its coverage with the fitted viewport or group transform.

`text/align-1-0.png`, row30, is a distinguishing witness: the left edge is
`[255,0,255,255]` at x0..3, and the right edge is
`[255,255,255,32,223]` at x380..384. Both now match exactly.
`transformed_strokes_preserve_physical_pen_and_fit_export_viewport` checks these
profiles and both rotated group pens. `group/two-rects-rotated.vsdx` changes from
MAD2.874225/differing2.541990% to0.690956/1.251292%, with430x360 unchanged.
SVG and PDF use the same transformed centerlines and physical pen expansion.

### P2-followup-3: image sampling and EMF records

PNG/JPEG sources use premultiplied bicubic reconstruction with two transparent
source pixels beyond each edge, clipped to the original image frame. Clamping
the edge texel suppresses Visio's soft fringe. Bilinear with the same border
measured image-group differing2.163508%; bicubic measured1.731289%, down from
2.830826%. PNG MAD1.088698->0.507656, differing2.284071%->0.632957%;
JPEG1.588222->1.008614,2.287688%->0.640191%. EMF remains3.292215/3.920718%.
`image_filter_uses_premultiplied_alpha_and_transparent_border` prevents transparent
blue from contaminating an opaque red edge. The SVG writer keeps the same border
and clips it with a nested viewport; all 204SVG/PDF checks pass.

Only `image/emf.vsdx` has an embedded rendering EMF. Its record ID:count inventory is
`1:1,10:4,12:4,14:1,18:4,19:2,22:4,24:4,28:4,33:10,34:10,36:22,37:42,39:3,40:11,59:1,60:1,62:1,70:17,75:5,76:1,82:4,84:4,85:2,86:5,87:2,95:5,98:5`.
Classic geometry, object, transform, clipping, font and text records are recognized.
The13state records18(background mode),28(meta-region reset),98(color management)
are ignored. All17type70comments carry EMF+ payloads; these remain unrendered and
now produce an explicit warning even when classic fallback geometry exists. The
one type76BITBLT uses0x00AA0029 (destination unchanged); it is now recognized as
a no-op instead of being counted as unsupported.

`image/png.vsdx` and `image/jpg.vsdx` each contain a thumbnail EMF with
`1:1,14:1,76:1`; the actual image media are96x64PNG/JPEG. The EMF drawing's
thumbnail uses`1,13,14,18,19,20,21,22,24,25,27,28,30,33,34,35,36,37,39,40,58,70,75,76,82,84,86,87,95,98,115`.
Every thumbnail is retained as opaque package data and is never rendered.

### Clamped circular height in BASFLO_U Custom 1

A fresh scheduled-task `dump-shipped.ps1` observation on 2026-10-12 produced
`corpus/_vm/out/dump-shipped/1033/BASFLO_U.VSSX.tsv` (15 masters, 11143 cells,
zero cell errors). The shared dump directory had no completed BASFLO_U TSV and
was left untouched. Master 11, shape 4 reports:

| Cell | Formula | COM inches |
|---|---|---:|
| Height | User.ResizeTxtHeight | 0.75 |
| User.ResizeTxtHeight | MAX(User.DefaultHeight,CEILING(TEXTHEIGHT(TheText,TxtWidth)+Scratch.Y1,0.25)) | 0.75 |
| Scratch.Y1 | MIN(Height/2,Scratch.X1) | 0.25 |
| Scratch.X1 | 0.25 in*User.AntiScale | 0.25 |

Visio resolves this apparent cycle. A sufficient evaluation order is to establish
Scratch.X1=0.25 and User.DefaultHeight=0.75, bound Scratch.Y1 above by 0.25,
and bound the empty text height plus offset below 0.5. CEILING therefore cannot
raise the resize height above its 0.75 minimum. Height is uniquely 0.75; evaluating
Scratch.Y1 then gives 0.25. This is a proven evaluation order, not a claim about
Visio's internal algorithm. The evaluator now uses bounded interval propagation
on a detected cycle and accepts only a finite singleton. No formula's cached V
is used as a seed. Ambiguous, divergent and missing-reference cycles still error.
Tests: `clamped_cycles_are_proven_without_cached_formula_values` and
`empty_flowchart_text_does_not_recurse_through_wrapping_width`.

### Text metric fixed-point size and empty fields

The COM observations in ADO_M master2 shape45 (6pt), ANNOT_U master14 shape5
(8pt), ASSOCIATIONS_UML_CLASS_M master9 shape5 (10pt) and master8 shape5 (12pt)
fit one rule: round font size in inches to 18 fractional bits before computing
composed text metrics. At 120% line spacing the height corrections are respectively
-0.000001525879, -0.000000508626, +0.000000508626 and +0.000001525879 inches.
ANNOT_U master16 shape11 at 140% gives +0.000001780192 inches, independently
confirming size rounding before multiplication. SVG painting retains the original
point size. `metric_font_size_grid_matches_com_height_witnesses` tests the numeric
rule; `text_metric_grid_and_empty_fields_match_private_com_witnesses` checks actual
stencil cells to 1e-12 inches.

An empty displayed field is still composed text. BASICCIRCUIT_DIAGRAM_M master8
shape5 and HVACCE_M master12 shape11 contain an empty fld followed by the terminal
paragraph marker. Keeping that field run activates the composed-text size rounding
and 0.00005-inch slack. Their TxtHeight values now match 0.1944949530707465 and
0.2333851135253906 inches. A truly empty block retains its separate empty-line path.

FIND now implements the public four-argument contract, including literal search,
optional one-based start and case-insensitive matching. Microsoft's January 1, 2003
example returns 12. Oversized strings and invalid start positions return errors.

### Composed width: terminal advance and fixed-point metrics (M2.4.4)

The TEXTWIDTH reference describes composed width but does not specify its terminal
advance or numeric grid. Twenty cached TxtWidth witnesses across ADO_M, ADO_U,
ADS_M, ADS_U and ANNOT_M resolve this. ADO master2 shape45 measures 0.366826785261
inches for six-point text with three-point left and right margins; the previous
result was 0.347937043135. ADS master 2 shape 5 measures 0.194218091820 inches at
eight points with zero margins, previously 0.169053174555.

Candidates were tested in the requested order. Margins already occur once: ADO's
combined inset is 0.083333333333 inch and ADS has none. Neither duplicating those
margins nor adding a four-point inset explains the delta. Kerning varies with the
string and disagrees with both the cached widths and SVG's unkerned advances.
One terminal space explains the large term. Rounding the font size, then each
advance (including that space), to 18 fractional bits in inches explains the
remaining small errors. Add the same 0.00005-inch composed-text allowance observed
for height. All twenty residuals are 0.000000001 inch, below the unchanged oracle
tolerance. The allowance is not applied to a truly empty block. Painting retains
SVG point coordinates, without this metric-only quantization.

Regression: `composed_width_terminal_space_and_advance_grid`. The read-only
`width_audit` example prints source and effective formulas, text, font, point size,
margins, unkerned character advances, kerned total and space advance. Session
records retain all twenty rows and each candidate's numeric residual.

Full M2.4.4 measurement: TxtWidth cached failures fall from 4078 to 1985;
text-only cached accuracy is 60525/67797 (89.273862%), COM is 6578/7724
(85.163128%). Both remain outside the main denominators under the unchanged
90% promotion rule. Main rates remain 98.118866% cached and 99.226552% COM.
The renderer remains at ten size mismatches, with eight passing groups.

### Terminal runs, fields and display units (M2.4.5)

BA_DEC_BAN_M master25 shape6 has a 28pt time followed by an empty 14pt field.
The final run supplies the terminal advance even though it paints no glyph.
User.Width now measures 1.668892739529 inches, versus Visio 1.668892740529;
the pre-M2.4.4 value was 1.624893142945. The difference was the 14pt space.

ACTUAL_FINISHDATES_GANTTCHART_M master43 shape11 uses
`GUARD(MAX(1*Char.Size,TEXTWIDTH(TheText)))`. Its XML contains indentation before
and after an empty field, despite inherited `xml:space='preserve'`. Visio excludes
those whitespace-only indentation nodes from text composition. Ignore only nodes
containing both a newline and spaces and no non-whitespace character; preserve
single paragraph breaks, plain spaces and all non-whitespace content. Width now
matches 0.111111111111 inch and Height matches 0.133382824707 inch. The old width
was 0.602862283587 and height 0.533381298828. Lossless source XML stays unchanged.
This is an observed whitespace exception, not a reason to strip arbitrary text.

ANNOT_U master26 shape8's TxtHeight `#VALUE!` came from `FORMAT` receiving the
`esc(8)` picture returned by FIELDPICTURE(8), via Fields.Format[6]. Microsoft defines
that format as three decimal places without units. Interpret numbered decimal
pictures 2 through 9; unsupported pictures retain errors. Its width is now 2 inches
and height 0.13338282470703125, matching Visio. No cached field display fallback
was added for unsupported formats.

CRIME_SCENE_INVESTIGATION_M page0 shape8's User.DirArrowOffset is
`TEXTWIDTH(TheText)*0.55`. Local Fields.Value has `U="MM"` with `F="Inh"`.
Inheriting the expression must preserve the local display unit. Its field now
prints 22500.00 instead of 885.83 and the offset is 27.682649414063 inches,
versus Visio 27.682649414613. Master2 shape12 also exposed an integral reduced
fraction printing `0/1` instead of `0`; after correction its offset is
17.651808281250 versus Visio 17.651808281800.

Regression: `text_metric_terminal_fields_and_unit_override_witnesses`, with
host-independent cases in `numbered_field_pictures_and_integral_fractions` and
`indented_empty_field_keeps_composition_without_layout_whitespace`.

The WWFCONCTRL_M User.Width and MWFCONCTRL_M User.MenuItemTextSize residuals
include formula-produced font ID 201, not mapped by the current portable font
profile. Their documents contain Segoe UI variants; current layout falls back to
Carlito. There is no embedded metric-compatible Segoe face, and no system scan or
additional font download was introduced. MWFCONCTRL_M master 125 shape 221 remains
1.034667529975 versus 1.112060956811 inches. Font identity and compatible metrics
remain explicit limitations; numerical width multipliers would conceal them.


### PNG composition and logical text extents (M2.6.9)

The sixteen generated SVGs remain a placement oracle for TxtPin, margins and
paragraph baselines (`text_blocks_and_unkerned_lines_match_visio_svg`). They are
not identical to Visio's bitmap composition: multiline-wrap.svg has eight lines,
including "narrow text block." on one line; multiline-wrap.png has nine, with
"block." on the next line. Its nine painted row bands begin at pixels 69,98,126,
155,186,212,241,270,298. A nonbreaking terminal advance during line breaking
reproduces this distinction. Exclude that synthetic advance from painted alignment.
The PNG composition is used consistently by our PNG, SVG and PDF backends.

Export bounds now include logical ascent on both sides of each baseline, and a
half-em horizontal allowance around glyph ink. Wrapped paragraphs reserve their
available line box. This is an observed partial approximation, not a complete
published Visio rule: six previously failing dimensions now agree, but four
one-pixel residuals remain. No per-file offsets or rounding thresholds were added.
Multiline-wrap changes 193x385 to 206x385 (Visio 206x385), metric-scale 77x30 to 77x32
(Visio 77x32), align-1-2 to 385x291, and all three right-aligned cases now match.
The three align-0-* cases are one pixel too wide (388 instead of387); sizes is
one pixel short (531 instead of532). Font ink and logical-overhang rounding still
need work. These files remain hard failures, with no pixel score.

Regression: `png_composition_and_logical_bounds_have_private_witnesses` checks
six independently exported PNG dimensions and the nine-line composition. The SVG
placement test continues to check the distinct eight-line SVG witness.

Remaining empty-block TxtHeight errors include APPL_M master 7 shape 5: Visio
0.166666666667 inches versus our16.666666666667. The same formula on master2
shape5 has COM16.666666666667 and already agrees. Thus removing drawing-scale
conversion globally would break a measured witness. The unresolved cause is
master-specific scale/context handling, not evidence that the oracle is wrong.

CALNDR_M master 9 shape 7 supplies a remaining concrete #VALUE! chain:
TxtHeight calls TEXTHEIGHT(TheText,TxtWidth); two text fields call
`FORMATEX(Sheet.5!Prop.visStartTime,User.visTimePMFormat,,,User.LCID)` (and
visFinishTime). The inherited picture is `{{h:mm am/pm}}`, locale1033; source
values are DATETIME(0.33333333333) and DATETIME(0.375). The DATETIME hook retains an ISO-date cached string, which is not converted to
a numeric date serial before FORMATEX. It reaches text-picture formatting and
returns #VALUE!; the calendar formatter also lacks the am/pm token in this
picture. Visio displays8:00AM and9:00AM; our field evaluation errors, so height
cannot be measured. This witness explains a
remaining formatting error, not all1054TxtHeight failures:309are#VALUE!,96are
unsupported locale 2057, and the rest are numerical/other failures.

M2.4.5 changed cached failure buckets as follows: User.Width1879 to1742,
Width1620 to746, TxtHeight1057 to1054, User.DirArrowOffset314 to26,
User.MenuItemTextSize265 to262, Height219 to198. Combined with M2.4.4,
TxtWidth falls4078 to1124. The half-inch Width and0.4-inch Height witnesses
were Gantt serialization indentation; the remaining buckets include different
causes and are not claimed solved wholesale.


### Human gate: frozen ceilings and one-pixel export tolerance (M2.8)

Charles accepted M2's human gate on 2026-10-12 at 16:45 America/New_York. The
non-text ceilings remain MAD 4/255 and 2% of pixels over 16/255; text remains
MAD 8/255 and 5%. These are recorded in floors.toml and mandatory in CI.

The size rule permits an absolute difference of at most one pixel on each axis.
Compare pixels at the same coordinates in the top-left common rectangle:
min(widths) by min(heights). Do not rescale, search for an alignment or edit the
input. Group differing fractions weight that actual common pixel area. Empty
images and a difference of two or more pixels on either axis fail. Reports retain
all raw dimension differences and distinguish tolerated from hard failures.

Reason supplied by Charles: Visio's own exports of generated/text/multiline-wrap
have eight lines in SVG and nine in PNG, so one-pixel boundary rounding cannot
be defined as right or wrong from Visio's output. This is a gate decision based
on an observed inconsistency, not a claim that Microsoft's reference promises
this tolerance. The four previously failing boundaries are align-0-0, align-0-1,
align-0-2 and sizes. All now receive common-area pixel measurements.

Regression tests cover both size directions, both axes, unequal row strides,
alpha compositing, common-area weighting, empty images and two-pixel rejection.
The render advisory bypass has been removed from both CI and corpus tests.

### P3 round-trip discoveries

`ANNOT_U.vssx`, master26 (Property line), shape5 has an Action toggle
`NOT(User.TextSide)` with cached result0. `User.TextSide` and `Scratch.B1`
reference that Action. COM reports0 for all three. Passive recalculation must
read the saved Action result; the Action formula runs only on invocation, as
[the Action reference](https://learn.microsoft.com/en-us/office/client-developer/visio/action-cell-actions-section) specifies.

`ARROWS_M.vssx`, master2 (Simple Arrow), shape5 `Scratch.A1` stores
`E="#DIV/0!"` and last valid `V="1.3336"`. COM ResultIU reports1.3336, and
`Geometry1.X4` reports0.15748031496062989. Numeric references use the last
valid value while error predicates retain the error state. This follows the
[Cell E/V contract](https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml);
our previous evaluator propagated the error into geometry and failed rendering.
These observations do not alter any oracle exclusion or floor.

### P3 round-trip comparison contract

The round-trip cell denominator is the union of all local source cells, including
master PageSheets, style gradient/LayerMember cells, deleted rows and cells with
no COM SRC mapping. No hook or volatile exclusions and no synthetic COM defaults
pad this count. Numeric sheet IDs and section/row identities must match.
Effective formulas and unit attributes compare exactly; numeric values have the
existing 1e-6 tolerance, and text/color values compare by their typed content.
Deleted cells, error values and ambiguous duplicate local names compare literally.
Inherited cells compare their resolved values, so legitimate recalculation of a
stale local Inh cache is not confused with duplicate-name masking. Missing and
extra cells fail. The required initial floor is 99%.

Each content page and master is rendered before and after writing. The image pair
uses the original world-space viewport and pixel denominator, and masters use one
scale fitted from the original. Native size differences over one pixel and origin
changes over one device pixel fail before scoring. This catches translations
without diluting them with additional white space. Frozen M2 pixel ceilings and
group aggregation apply. The generated Visio comparison remains a separate gate,
including its maximum-ten individual-outlier ratchet.

M3.8 dynamic-glue formulas are observed functional behavior from private
corpus/generated/connector/dynamic-glue.vsdx, not copied graphics. The model helper
writes WALKGLUE endpoints, XFTRIGGER target events, target ObjType1 and both Connects;
cached endpoints come from the caller's page geometry. The scratch reread and hostile
request tests cover this contract. Acceptance requires both live COM Connects rows.

M3.9 round-trip rule (Charles, P3-closure): equal error strings at the same scene,
canvas or raster stage count as equal-error pairs. Both sides run independently;
one-sided or different errors fail. Equal errors have no invented pixel measurements
and are printed per group. The generated Visio render oracle and ceilings are unchanged.

## M3.10 COM coverage ruling, 2026-10-13

The historical population at 9eb358d has 301 completed shipped dumps plus one partial,
not 302 completed. Membership was reconstructed from 104 FILE OK entries in probe-log.txt
and 197 additional entries before the 2026-10-13T00:32:39Z host-pull marker in winvm-log.txt.
The partial AZUREAPPSERVICES_M dump is not eligible. Generated/blank 213 dumps remain old.
The independently reviewed normalized membership SHA256 is
05375146b6ef812251baa5f22a41266dd2d261a43469fc927bf85b71cdf9bc81.

| Population | Completed dumps | Passed / eligible | COM rate |
|---|---:|---:|---:|
| Old, including generated/blank | 514 | 20805945 / 21055732 | 98.813686458% |
| New shipped | 401 | 23240476 / 23671431 | 98.179429879% |
| Combined | 915 | 44046421 / 44727163 | 98.478012120% |

The old rate meets the 98.6241% contract. This is coverage growth, not regression.
Charles's P3-closure M3.10 explicitly authorizes rebasing the expanded denominator.
The COM floor is now 98.478012100% (achieved fraction rounded down at nine decimals),
and may only rise from this population. Text ratchets, cached floor, exclusions and
render ceilings are unchanged. The shared directory remains read-only.
Measurement: cargo run -q --release -p diagramcraft-corpus --example com_coverage --
corpus/_state/m3/old-coverage-paths.txt; log/m3-10-coverage.log.

New-population failures by called function (overlapping, one row can name several):

| Function | Failed rows | Witness |
|---|---:|---|
| `<no function>` | 397145 | `corpus/ms-shipped-dumps/1033/KUBERNETESVISIOSTENCIL_U.VSSX.tsv` |
| THEMEVAL | 20497 | `corpus/ms-shipped-dumps/1033/LANDSCAPE_M.VSSX.tsv` |
| IF | 7509 | `corpus/ms-shipped-dumps/1033/MATRIXDOT_M.VSSX.tsv` |
| LUM | 4162 | `corpus/ms-shipped-dumps/1033/MEDIUMENTERPRISECORPNET_M.VSTX.tsv` |
| GUARD | 3672 | `corpus/ms-shipped-dumps/1033/LDAPOB_M.VSSX.tsv` |
| SAT | 2748 | `corpus/ms-shipped-dumps/1033/MEDIUMENTERPRISECORPNET_M.VSTX.tsv` |
| HUE | 2706 | `corpus/ms-shipped-dumps/1033/MEDIUMENTERPRISECORPNET_M.VSTX.tsv` |
| HSL | 2698 | `corpus/ms-shipped-dumps/1033/MEDIUMENTERPRISECORPNET_M.VSTX.tsv` |
| THEMEGUARD | 2395 | `corpus/ms-shipped-dumps/1033/LEGEND_M.VSSX.tsv` |
| LUMDIFF | 1969 | `corpus/ms-shipped-dumps/1033/LEGEND_M.VSSX.tsv` |
| MODULUS | 1890 | `corpus/ms-shipped-dumps/1033/OBJREL_M.VSSX.tsv` |
| SHADE | 1824 | `corpus/ms-shipped-dumps/1033/MATRIXDOT_M.VSSX.tsv` |

## M3.11 acceptance rerun, 2026-10-13 (partial)

| Set | Files | Opened | Repaired | Failed |
|---|---:|---:|---:|---:|
| generated | 204 | 204 | 204 | 204 |
| blank | 1 | 1 | 1 | 1 |
| from-scratch | 5 | 5 | 5 | 5 |
| shipped-stencil | 10 | 10 | 10 | 10 |
| shipped-template | 10 | 10 | 10 | 10 |

Old COM population: 20,805,945/21,055,732 =98.813686458% over 514 dumps
(301 completed shipped plus 213 generated/blank). This meets 98.6241%, so Charles's
P3-closure M3.10 coverage rebase is authorized. New 401 dumps:
23,240,476/23,671,431 =98.179429879%. Combined 915 dumps:
44,046,421/44,727,163 =98.478012120%; floor 98.478012100%.
The historical 302 artifacts were 301 complete plus one partial, independently verified.
The later measurement has 702 completed shipped dumps; earlier 413/523 snapshots are historical.
Text stays promoted and its existing ratchets stay unchanged.


The changed harness completed every file. Scratch dynamic glue now has exactly the two
COM rows: shape 3 BeginX -> shape 1 PinX (9,3), EndX -> shape 2 PinX (12,3).
This closes the position-only glue defect without claiming repair-free serialization.

### Observed, undocumented: post-open saves

Literal/part witnesses, retained under corpus/_vm/out/accept/:

- generated__basic__0001-probe:9 literal differences,17 source COM differences; BulletStr
  changes from 0 to an empty string, LangID 1033 to en-US, TextBkgnd 26 to #ffffff.
- blank__blank:4 literal differences,0 source COM differences, no part changes.
- scratch__rectangle-text:12 differences including added docProps/thumbnail.emf.
- scratch__dynamic-glue:13 differences including added thumbnail.emf;2 live connects;
  zero source COM differences.
- scratch__metric-background-layers:99 differences,323 source COM differences; added
  thumbnail.emf and visio/pages/_rels/page2.xml.rels.
- scratch__three-masters:89 differences,321 source COM differences; Visio removes all
  three authored master parts and masters.xml.rels, and adds thumbnail.emf.
- shipped-template__SOCIAL_DISTANCING_M:1,446 literal/part differences, the largest count.

All five scratch inputs omit thumbnails as explicitly required by M3.7. Visio adds an
EMF thumbnail on SaveAs in all five, so the unchanged part-list definition fails.
This observation prevents a claim of zero repair while retaining the explicit no-thumbnail
construction rule. No thumbnail exclusion, oracle output or threshold was changed.
Other serialization and render disagreements remain real bugs, not waived exceptions.

### Measured

Host round-trip:15,080,738/15,080,738 cells (100%); all successful image pairs have
MAD 0/fraction 0, no size mismatch. Equal errors are 568 masters + 74 pages, separately
reported without pixel scores. Every round-trip group passes the authorized M3.9 rule.
Generated Visio groups remain within frozen ceilings, with 10 individual outliers,
4 tolerated one-pixel differences and0 hard size failures.

Full cached floor passes: 4,675,663/4,783,865 =97.738189% versus 97.491100%.
Full COM main floor passes at 98.478012120%. Text cached 61,748/67,797 =91.077776%;
text COM 27,360/30,261 =90.413403%, below its unchanged 90.713900% sticky floor.
Remaining text failures: 6,049 cached and 2,901 COM. corpus check therefore exits 1.
Coverage: 1,387 packages, 1,181 shipped sources, 702 completed shipped dumps, 915 TSVs.
Parity 25% (1 D, 17 P, 27 M of 45 Core rows), explicitly partial.

Independent discovery and source followups found and closed diagnostic storage and
presentation issues. The production glue seed was caught (exit 101); exact source bytes
were restored and the same regression passed (exit 0). Frozen verification remains
blocked by measured M3-Gates failures; do not claim two clean closure passes.


| Acceptance render group | Images | Errors | Mean MAD (0 to 255) | Pixels over16 | Pass |
|---|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 0.543633 | 1.071225% | YES |
| basic | 1 | 0 | 0.443949 | 0.745332% | YES |
| blank-pages | 1 | 1 | n/a | n/a | NO |
| connector | 6 | 0 | 2.142340 | 1.225363% | YES |
| fill | 44 | 0 | 1.112028 | 1.373298% | YES |
| from-scratch-masters | 8 | 3 | 1.160070 | 2.572484% | NO |
| from-scratch-pages | 5 | 2 | 3.767709 | 2.309100% | NO |
| generated-masters | 9 | 0 | 1.202102 | 2.648070% | NO |
| group | 1 | 0 | 0.690956 | 1.251292% | YES |
| image | 3 | 0 | 1.602828 | 1.731289% | YES |
| line | 37 | 0 | 1.076040 | 1.030076% | YES |
| master | 1 | 0 | 1.608085 | 1.791412% | YES |
| page | 4 | 0 | 2.821761 | 1.171389% | YES |
| shipped-stencil-masters | 186 | 18 | 15.839527 | 12.408190% | NO |
| shipped-template-masters | 115 | 29 | 8.180108 | 2.313173% | NO |
| shipped-template-pages | 11 | 10 | 1.926767 | 3.725650% | NO |
| text | 16 | 0 | 3.890125 | 3.318851% | YES |
