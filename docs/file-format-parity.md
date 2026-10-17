# File format parity

Status: pre-alpha, M0 through M3 met. P4a met on 2026-10-15 under Charles's straight-route ruling: 100% strict round-trip, empty authoring/glue diffs and four Visio opens with zero residual repairs and COM differences. Individual connector pixels remain measured and informational until connector stroke rendering is resolved; M4 awaits 4b. Current P4a measurements are at the end; earlier entries retain their dated results.

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
mismatch or render error still fails its group. `corpus check` prints all 204
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
PDFs. Complete reading order remains a separate gap.
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

`misterdiagram-text::tests::report_default_font_substitutes` records the selected faces.
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

### Source caches on open and save (M3.12)

A plain save preserves the local V text, F, U and E of each unchanged cell,
including F="Inh". A source index and dependency checks distinguish these caches
from authored cells and cells affected by edits. The writer evaluates dirty caches
in internal inches, then serializes colors, language tags, strings and document
font ordinals by cell type. Evaluator font IDs are not FaceName ordinals.
FaceName records do not gain an ID attribute.

Witnesses: `generated/arrow/begin-00.vsdx`,
`generated/basic/0001-probe.vsdx` and `ms-shipped/1033/SITACC_M.vssx`.
The writer regression compares every local V/F/U/E in these packages literally.
The original probe already has LangID `en-US`, empty BulletStr and TextBkgnd
`#ffffff`. Earlier saves that replaced them with `1033`, `0` and `26` were
writer defects, not normalization by Visio. Earlier acceptance COM counts require
the M3.18 reporter correction before they can support a conclusion.

M3.13 inherited instance caches: the writer preserves existing local `Inh` values
and materializes missing master cells when their instance result differs from the
master cache. The serialized cache uses the source unit. Witness:
`corpus/generated/master/basic-rect.vsdx`, page 1 shape 1, resized to 3 by 2 inches.
`resized_basic_rectangle_reproduces_inherited_instance_cache_set` verifies the exact
cell and `Inh` sets, including reconstruction after removing those local caches.
Expanded strings are charged against the sheet byte bound before insertion.

### Master content IDs (M3.14, observed)

The diagnostic `corpus/_vm/out/diag-masters/probe-log.txt` records that `tm-v0`
with authored content ID 1 opens with no master shapes, while `tm-v1-shapeid5`
loads the shape. New master content must use IDs from 5 upward; invalid IDs are
rejected rather than silently renumbered. The three-master fixture uses ID 5.

There is a narrow source-preservation exception: 691 original shipped shape records
use ID 4, with no IDs 0 through 3 found. Witness:
`corpus/ms-shipped/1033/PHONE_CONN_U.vstx`, `visio/masters/master2.xml`, and its
COM dump (`master:4:Procedure/Sheet.4`, Width 1, Height 0.75). The writer retains
ID 4 only when the original bytes of that master part contain it as master content.
An editable XML tree or unrelated extension cannot authorize the exception.

Authored and edited literal caches also use the typed serializer: numeric language
IDs become language tags, color indices become RGB strings, Font literals retain their numeric value or supplied name, and empty strings remain empty. Imported unchanged literals
retain their source text. These cases extend the M3.12 formula-cache regressions.

M3.15 background and unit witnesses: a foreground BackPage carries a relationship to
its background content part. Visio's saved scratch metric drawing adds page2.xml.rels
with target page1.xml. The skeleton now creates background-first page order and the
observed page view defaults. Removing pages or their background links removes obsolete
relationships; cycles and Connects to missing live sheets fail before serialization.
Sheet 0 remains a valid enclosing PageSheet target (BLOCKP_M master 2, User.VPX).
Metric document/style formatting defaults without explicit U resolve using drawing units:
the scratch metric COM dump reports millimetres (70), while SOCIAL_DISTANCING_M uses
DrawingScale U=M and reports metres (71). Explicit cell units remain authoritative.
Live acceptance of the corrected scratch drawing is pending the M3.23 rerun.

M3.16 dynamic connector instances now carry a straight route and its cached transform,
text-position controls, deleted unused Geometry row, ConFixedCode and Connector layer.
Witness: generated/connector/dynamic-glue.vsdx page1 shape3. Four page-space direction
cases pass after write/read; endpoint hooks retain caller-supplied caches. The two
Connect records remain. Live Visio ink, size and Connect verification is still pending.

M3.17 round-trip comparison: unchanged source formulas require identical cached V
text as well as the existing evaluated comparison. This exposes altered Inh caches
that resolve to the same provider. The local-cell union denominator and 99% floor
are unchanged; the evaluation-only rate is printed separately for comparison.

M3.17 full corpus literal-cache comparison: 15079024/15091678=99.916152% across 1,387 packages, above the unchanged 99% floor. The prior M3.4 raw-cache observation was 578769/584822=98.96% before the writer corrections; its replacement by evaluation-only comparison hid real differences. The current diagnostic prints both strict and evaluation-only rates on the same local-cell union.
Evaluation-only diagnostic for that full run: 15079024/15091678=99.916152%.

M3.18 acceptance comparison uses literal local V/F/U/E records, including Inh
instance caches, without evaluating them. COM differences have their own column;
they remain a gate but do not by themselves classify a file as repaired. Row type
and deletion, shape/master/page attributes, structural presence, Text and Icon
character data, and SHA256 digests of opaque payloads are compared. Exclusion
filters apply to both sides. Unmodified docProps and windows XML retains its
original bytes. XML physical newlines normalize on read according to XML 1.0;
explicit character references remain literal. No observation file is rewritten.
The PageSheet XForm row 1 unit 32 versus 73 difference is disclosed as a computed
harness artifact only when neither package has a local source cell. Source cache
values remain mandatory. Historical P3-closure counts are restated separately
before the later acceptance extent and allowed-repair amendments.

## M3.18 historical P3-closure restatement

The inputs and Visio outputs are unchanged. Expectations now use literal local caches; COM differences remain mandatory but are separate from repair classification. This is the historical run, before M3.19/M3.20/M3.22 amendments and before the fixed writer rerun.

| Set | Files | Opened | Repaired | Failed |
|---|---:|---:|---:|---:|
| blank | 1 | 1 | 1 | 1 |
| from-scratch | 5 | 5 | 5 | 5 |
| generated | 204 | 204 | 204 | 204 |
| shipped-stencil | 10 | 10 | 10 | 10 |
| shipped-template | 10 | 10 | 10 | 10 |

Literal/part differences: 30237; source COM differences: 197517; disclosed phantom XForm unit slots: 1296.
MasterType rewrites disclosed separately: 14.
Reporter exit is recorded in log/m3-18-historical-report.exit. Full evidence: corpus/_state/m3/accept-report-restated-p3-closure.json and log/m3-18-historical-report.log.

M3.18 CI exposed and fixed a text-adapter dependency on raw CRLF: BA_DEC_BAN_M
master 25 shape 6 switches from 28 pt field text to a 14 pt paragraph terminator.
After XML 1.0 normalization, the empty chunk before LF lost that terminal run,
changing User.Width from 1.66889274052908 to 1.7128533108181425. Preserve the
terminator's character style explicitly for both LF and referenced CR+LF.
The unchanged corpus regression and all 19 render-output tests now pass; writer 39/39
still passes, including zero probe literal differences. Earlier CI exit 101 is a
real fixed regression, not a waived gate. Full CI and historical expectation/render
regeneration restart on the corrected source. No observations or thresholds changed.

M3.18 final measurement classification: 180294 source COM differences; 17223 style-local
XML cells are unobserved by both collectors and separately listed in each report
row. Literal XML comparison remains mandatory. Numeric eligibility is identical
for every file before and after this classification (verified). A metadata-only or
observation-only cell still fails. Witness: generated__arrow__begin-00 style0 has
75 such rows, including HelpTopic and Copyright. The collector does not promise
these rows are inaccessible in COM. Previous197517 included this unobserved class
and is superseded. Literal/part differences30237, opened230, repaired230 unchanged.
Python19/19 tests pass under python3 with Pillow; project .venv lacks Pillow and
was not used for the final measurement. No VM output or input was modified.

### M3.19: acceptance export extent

Observed, undocumented: Page.Export uses shape alignment boxes for the logical
geometry extent, rather than arbitrary Geometry coordinates. Every visible shape
contributes its transformed Width by Height box, including geometry-free groups.
Stroke, marker and text overhangs remain separate painted extents: the original
`text/multiline-wrap` exports 206 pixels for a 2 inch alignment width, and
`arrow/begin-02` exports 482 pixels for a 5 inch alignment width. The acceptance
renderer uses this extent; ordinary generated-corpus rendering retains its frozen
M2 contract.

An empty page or master exports 480 by 288: witnesses `blank__blank`, BASFLO_M
page 0, WWFC_M page 4 and UCMNME_U page 0. Only pairs whose input and saved
packages both contain no shapes skip pixel comparison, and our output must be
blank. Empty pairs are counted separately. Every other acceptance size difference,
including one pixel, is a hard failure. Charles authorized these definitions in
P3-closure-2 M3.19; no pixel ceiling changes.

### M3.20: authorized SaveAs differences

Observed, undocumented, authorized by Charles in P3-closure-2: the repair gate
allows only an added `docProps/thumbnail.emf`, a rewritten MasterType value, and
an otherwise unchanged master UniqueID whose final groups become
`8E40-00608CF305B2`. Each class has its own count and witnesses in the acceptance
report. All other part, attribute and cell differences remain repairs, including
LayerMember additions and changed V text. An existing thumbnail changing bytes
is not the added-thumbnail exception.

Witnesses: the probe master changes MasterType 0 to 541 on Visio's own re-save;
`corpus/_vm/out/accept/scratch__rectangle-text/saved.vsdx` supplies the UniqueID
form. The thumbnail bisect in `corpus/_state/m3/diag-variants/` and
`corpus/_vm/out/diag-scratch/probe-log.txt` accepted no thumbnail and rejected the
2 by 2 PNG. We author no thumbnail and will not implement the minimal EMF option.
New synthetic masters use random first three GUID groups and the observed suffix.

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
| Set | Files | Opened | Repaired | Failed | Thumbnail added | MasterType | UniqueID |
|---|---:|---:|---:|---:|---:|---:|---:|
| generated | 204 | 204 | 203 | 203 | 0 | 7 | 0 |
| blank | 1 | 1 | 1 | 1 | 0 | 0 | 0 |
| from-scratch | 5 | 5 | 5 | 5 | 5 | 7 | 0 |
| shipped-stencil | 10 | 10 | 7 | 10 | 1 | 0 | 0 |
| shipped-template | 10 | 10 | 10 | 10 | 0 | 0 | 0 |

Allowed classes are six added thumbnails (all five scratch files and
shipped-stencil__AZURECOMPUTE_M), 14 MasterType rewrites (seven generated,
seven scratch) and zero UniqueID rewrites. Witnesses: scratch__rectangle-text adds
docProps/thumbnail.emf; generated__basic__0001-probe master 2 changes MasterType
0 to 541. MasterType counts are attributes, thumbnail counts are parts. No
existing opaque thumbnail, Icon, media or embedding change is waived.

The historical P3-closure restatement on unchanged old inputs/outputs is 230 opened,
230 repaired, 30,237 literal/part differences and 180,294 source COM differences.
The former 4,107/295,787 report was unreliable. Historical evaluator-only round-trip
100% was also insufficient: the old raw-cache result 578,769/584,822 = 98.96% was
real. After the writer fixes, the strict local-cache comparison is
15,079,024/15,091,678 = 99.916152% across 1,387 packages, above the unchanged 99%.
The current evaluation-only diagnostic has the same numerator and denominator.

The harness-only PageSheet XForm row 1 unit artifacts remain disclosed separately:
1,296 absent-in-both-XML slots, with COM codes 32/73, are not source cell changes.
17,223 style-local cells unobserved by COM remain covered by literal XML comparison.
The numeric COM eligible population was not changed to conceal these failures.

| Group | Images | Empty | Errors | MAD (0 to 255) | Pixels over 16 | Gate |
|---|---:|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 0 | 0.543633 | 1.071225% | PASS |
| basic | 1 | 0 | 0 | 0.443949 | 0.745332% | PASS |
| blank-pages | 1 | 1 | 0 | n/a | n/a | PASS |
| connector | 6 | 0 | 1 | 2.368622 | 1.116240% | FAIL |
| fill | 44 | 0 | 0 | 1.112028 | 1.373298% | PASS |
| from-scratch-masters | 8 | 0 | 0 | 119.602383 | 59.281546% | FAIL |
| from-scratch-pages | 5 | 0 | 0 | 235.505866 | 96.519357% | FAIL |
| generated-masters | 9 | 0 | 0 | 1.202102 | 2.648070% | FAIL |
| group | 1 | 0 | 0 | 0.690956 | 1.251292% | PASS |
| image | 3 | 0 | 0 | 1.602828 | 1.731289% | PASS |
| line | 37 | 0 | 1 | 1.105930 | 1.047197% | FAIL |
| master | 1 | 0 | 0 | 1.608085 | 1.791412% | PASS |
| page | 4 | 0 | 0 | 2.821761 | 1.171389% | PASS |
| shipped-stencil-masters | 186 | 0 | 102 | 10.127434 | 8.677037% | informational |
| shipped-template-masters | 115 | 0 | 63 | 5.829746 | 2.323606% | informational |
| shipped-template-pages | 11 | 6 | 4 | 1.917666 | 3.722501% | informational |
| text | 16 | 0 | 5 | 1.740866 | 2.243005% | FAIL |

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

| COM cohort | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Old, 301 shipped plus 213 generated/blank | 20805945 / 21055732 | 98.813686458% | 12807 / 14118 | 90.713982150% |
| New within pinned 702, 401 dumps | 23240476 / 23671431 | 98.179429879% | 14553 / 16143 | 90.150529641% |
| Pinned 702 shipped, 915 total | 44046421 / 44727163 | 98.478012120% | 27360 / 30261 | 90.413403391% |
| Later 170 shipped, outside rebase | 9819068 / 9991250 | 98.276672088% | 11477 / 11764 | 97.560353621% |
| Full measured 872 shipped, 1085 total | 53865489 / 54718413 | 98.441248652% | 38837 / 42025 | 92.414039% |

Old text meets 90.7139%, authorizing M3.21's pinned combined text floor
90.413403300%. Cached text is unchanged: 61,748/67,797 = 91.077776%, above
90.9875%. Main COM retains 98.478012100%; later coverage fails that floor and is
not rebased. Cached main is 4,675,663/4,783,865 = 97.738189%, above 97.4911%.
The preceding 413-dump measurement was 27,416,668/27,803,388 = 98.609090%, framed
in the brief as 302 old plus 111 new; proven complete membership is 301 plus 112.
Both pinned main/text memberships and SHA256 values are committed in
apps/diagramcraft-corpus/tests/cohorts/ and named in floors.toml. Ordinary full
checks keep consuming all completed coverage. Workers' shared input remains read-only.

Round-trip render: every scored original/rewritten pair has MAD 0 and fraction 0,
with no size mismatch. Identical renderer errors remain 568 master pairs and 74
page pairs, separately disclosed without pixel scores. Strict cell mismatches total
12,654; largest names are FillBkgnd 1,307, Char.Size 593,
User.SubShapeType.Prompt 242, Rounding 233 and User.ValueFormat 184.
Complete name counts are in log/m3-23-roundtrip.log.

Final frozen verification passes1and2 each returned10YES/4NO. The NO lines are
M3.13 (dropped-master pixels), M3.15 (metric COM units), M3.16 (connector pixels)
and M3.23-Gates. Both seeds were caught101/restored0. No two-clean-pass claim.
M3.23 records partial results only. Full private report:
corpus/_state/P3-closure-2-report.md; detailed register:
plan/m3-verification-register.md. Final Python reporter suite25/25 passes.

### Observed, undocumented: original-file baseline (M3.24)

Charles's P3-closure-3 replaces the M3.20 repair gate with a paired experiment.
For each of the 225 files with an original, the same OpenEx/dump/export/SaveAs/close
sequence runs on an unchanged copy of that original. D_ours compares our writer
output with Visio's save of it. D_base compares the original with Visio's save of
that original. Repair differences are D_ours minus D_base, matched by exact path
and change kind. Both full sizes and each subtraction count are reported per file.
Cells compare literal V/F/U/E, row T/Del and shape/master attributes remain visible,
Text/Icon and opaque payloads retain hashes, and metadata XML fields are compared
alongside part hashes. A baseline change to one metadata field does not authorize
a different field elsewhere in that part. Every original, copied input, observation
and source COM metadata artifact is pinned by SHA256.

Scratch files have no original. Their baseline is the blank experiment restricted
to document.xml, windows.xml and docProps; the existing three M3.20 classes apply
to the remaining scratch differences. Their counts stay visible. Raw, baseline
and remaining COM differences are separate; scratch COM is never subtracted.
Shipped render rows remain informational. The precise original-baseline witnesses
will be recorded when this run completes, without inventing evidence while it runs.

Charles also corrects M3.19: the M2 human gate's one-pixel export tolerance applies
to M3 acceptance. Compare the common area when either dimension differs by one;
a difference of two or more stays a hard failure. No pixel ceiling changes.

### M3.25: observed baseline save behaviours (2026-10-14)

The completed 225-file original baseline contains 4,624 differences. All 225
files opened; input and output hashes were verified before inspection. These are observations,
not comparator exceptions: subtraction always uses the same path and change kind.

| Behaviour | Original-file baseline witness |
|---|---|
| Window state changes | `generated/basic/0001-probe.vsdx`: ViewCenterX changes from `4.2538462786703` to `4.2461539707522` in pages.xml and windows.xml; ViewCenterY changes too. No ViewScale change appears in the completed 225-file baseline. |
| Empty clipping formula removed | `generated/image/png.vsdx`, shape 1: ClippingPath keeps its value and loses `F='""'`. The JPEG and EMF drawings show the same behaviour. |
| Redundant paragraph marker removed | `generated/text/align-1-0.vsdx`, shape 1: the leading `<pp IX="0"/>` disappears, while the text remains. |
| Field display text refreshed | `generated/text/fields.vsdx`, shape 1, field 1: `10/11/2026` becomes `10/13/2026` in this baseline run. The field is not replaced with a comparison-time expected value. |
| Older sheet schema updated | `ENTITY_U.vssx`: style cells listed below are added or removed; DocumentSheet also gains DocLockDuplicatePage. |
| Existing icons regenerated | `ENTITY_U.vssx`, master 10: the Icon digest changes. There are 266 Icon changes in the completed set. A scratch icon addition still needs its own authorized baseline identity. |
| Font metadata refreshed | `AZUREVMWARESOLUTION_M.vssx`, FaceName 0: Flags changes from 325 to 357. `ENTITY_U.vssx` changes the attribute spelling Panos to Panose. |
| Older row/section metadata updated | `ENTITY_U.vssx`, master2.xml shape 5: a Connection row gains T. An empty Paragraph section disappears in master1.xml. |
| Save metadata refreshed | `generated/arrow/begin-00.vsdx`: core modified time and custom filetime change. Their part hashes change as well. `AZUREVMWARESOLUTION_M.vssx` gains lastModifiedBy. |
| Missing preview generated | `AZURECOMPUTE_M.vssx` gains docProps/thumbnail.emf. No preview bytes are authored or copied into the project. |
| Master type updated | `generated/basic/0001-probe.vsdx`, master 2: MasterType changes from 0 to 541. The reporting column remains visible. |
| Locale refreshed | `PIN_MARKING_TEMPLATE_U.vstx`, DocumentSheet DocLangID: en-IN becomes en-US. |
| Inferred unit metadata simplified | `USTRME_U.vssx`, master1.xml shape 5, User.BackGrndLine: explicit U=COLOR is removed. |
| Older window schema updated | `ENTITY_U.vssx`: 2011 Windows attributes such as ClientHeight are replaced during reserialization. The comparison records attributes and byte hashes. |
| Palette entries appended | The saved scratch rectangle demonstrates entries 25 to 27, recorded in M3.26. No original-file ColorEntry difference appears in the completed 225-file baseline, so no palette difference is subtracted on that evidence. |

The ENTITY_U style-cell list is EmbellishmentIndex, FillGradientAngle,
FillGradientDir, FillGradientEnabled, LineGradientAngle, LineGradientDir,
LineGradientEnabled, LockThemeIndex, LockVariation, NoProofing,
QuickStyleFontMatrix, QuickStyleVariation, ReplaceCopyCells,
RotateGradientWithShape, ShdwBkgnd, ShdwBkgndTrans, UseGroupGradient,
VariationColorIndex and VariationStyleIndex. The private inventory records the
exact path, direction and values for each change.

The interim inventory exposed writer-introduced OPC byte changes: six for the
probe and four for arrow begin-00. M3.26 preserves unchanged wrapper bytes; the
fresh acceptance rerun tests that fix. These were implementation defects, not
formatting exceptions. Private evidence: `baseline-observations.json`, the earlier
`baseline-observations-partial.json` and the matching input/saved packages.


Complete baseline comparison inventory (225 files, 4,624 path/kind differences):

| Comparison category | Count | One original-file witness |
|---|---:|---|
| Part byte hash | 709 | generated__arrow__begin-00: `('docProps/core.xml/Opaque', 0)` |
| Structure/metadata V | 485 | generated__arrow__begin-00: `('docProps/core.xml/{http://schemas.openxmlformats.org/package/2006/metadata/core-properties}coreProperties:0/{http://purl.org/dc/terms/}modif...` |
| Page view attributes | 854 | generated__arrow__begin-00: `('visio/pages/pages.xml/Page:0/Attribute:ViewCenterX', 0)` |
| Other XML attribute V | 748 | generated__basic__0001-probe: `('visio/masters/masters.xml/Master:2/Attribute:MasterType', 0)` |
| ClippingPath cell | 3 | generated__image__emf: `('visio/pages/page1.xml/Shape:1/Cell:ClippingPath', 0)` |
| Shape Text serialization or field reevaluation | 31 | generated__text__align-1-0: `('visio/pages/page1.xml/Shape:1/Text', 0)` |
| Structure/metadata added | 24 | shipped-stencil__AZUREVMWARESOLUTION_M: `('docProps/core.xml/{http://schemas.openxmlformats.org/package/2006/metadata/core-properties}coreProperties:0/{http://schemas.openxmlformats.o...` |
| FaceNames table V | 48 | shipped-stencil__AZUREVMWARESOLUTION_M: `('visio/document.xml/FaceName:0/Attribute:Flags', 0)` |
| Literal cell added | 33 | shipped-stencil__ENTITY_U: `('visio/document.xml/DocumentSheet/Cell:DocLockDuplicatePage', 0)` |
| FaceNames table removed | 1 | shipped-stencil__ENTITY_U: `('visio/document.xml/FaceName:0/Attribute:Panos', 0)` |
| FaceNames table added | 1 | shipped-stencil__ENTITY_U: `('visio/document.xml/FaceName:0/Attribute:Panose', 0)` |
| Literal cell removed | 836 | shipped-stencil__ENTITY_U: `('visio/document.xml/StyleSheet:0/Cell:ShdwBkgnd', 0)` |
| Structure/metadata removed | 141 | shipped-stencil__ENTITY_U: `('visio/masters/master1.xml/Shape:5/Section:Paragraph:/Present', 0)` |
| Other XML attribute added | 204 | shipped-stencil__ENTITY_U: `('visio/masters/master2.xml/Shape:5/Section:Connection:/Row::0/Attribute:T', 0)` |
| Master Icon V | 266 | shipped-stencil__ENTITY_U: `('visio/masters/masters.xml/Master:10/Icon', 0)` |
| Other XML attribute removed | 237 | shipped-stencil__ENTITY_U: `('visio/windows.xml/{http://schemas.microsoft.com/office/visio/2011/1/core}Windows:0/Attribute:ClientHeight', 0)` |
| Literal cell U | 1 | shipped-stencil__USTRME_U: `('visio/masters/master1.xml/Shape:5/Section:User:/Row:BackGrndLine:/Cell:Value', 0)` |
| Part added | 1 | shipped-stencil__AZURECOMPUTE_M: `part:docProps/thumbnail.emf` |
| Literal cell V | 1 | shipped-template__PIN_MARKING_TEMPLATE_U: `('visio/document.xml/DocumentSheet/Cell:DocLangID', 0)` |

These counts describe D_base before intersection with D_ours. The acceptance
report retains the intersected and residual counts per file. No category is an
exception in code. The complete baseline has zero ViewScale and zero ColorEntry
changes; the authored palette uses the separate saved-scratch witness.

### M3.26: authored defaults and literal colour caches

Visio's saved scratch rectangle supplies the observed no-theme palette: 24 is
`#7F7F7F`, 25 `#000000`, 26 `#FFFFFF`, 27 `#0000FF`. The skeleton authors these
four entries, empty style-0 Copyright/HelpTopic, and the connector master's empty
inherited Comment/ShapeKeywords. A cached literal RGB carries its colour even
without a palette declaration. Reading a speculative cache does not allocate a
dynamic palette ID, so it cannot change subsequent formula results.

The metric scratch save uses 16 significant digits in authored numeric caches:
297 mm is `11.69291338582677` internal inches. Unchanged read caches still bypass
formatting. The complete authored layer row contains Name, Color, Status, Visible,
Print, Active, Lock, Snap, Glue, NameUniv and ColorTrans. Omitted metric length
units resolve in the evaluator from the owning PageSheet, including paragraph
indents and text-after-bullet position. Explicit master DL also resolves through
its PageSheet, as basic-rect master 4 Geometry1.A4 demonstrates.

An authored instance omits an inherited scalar whose evaluated value equals the
actual master cache. The dynamic scratch witness therefore omits redundant PinY,
LocPinX and Width. Existing local cells from a read file retain M3.12 preservation.
A diagnostic on the old metric scratch local-cell set now has 623 eligible COM
cells and zero differences, versus 65 before. Added authored cells and final
pixels still require the new scratch observations.

The baseline additionally proved that unchanged OPC wrappers must retain their
source bytes: the probe had six unmatched hashes and arrow begin-00 had four,
while their original-file baseline left those parts unchanged. Relationships are
compared by their resolved model and content types by their parsed tree before
reusing bytes. Changed relationships and content types are regenerated, so removed
pages and macro removal retain their validation. This fix requires regenerating
and observing all 230 acceptance inputs, not only the five scratch files.

## M3.27 pinned COM populations (2026-10-14)

Charles's P3-closure-3 authorizes the 702-to-872 coverage rebase. The old population
still meets both contracts; no evaluator regression is hidden.

| Population | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Old 702 shipped plus generated/blank | 44046421 / 44727163 | 98.478012120% | 27360 / 30261 | 90.413403391% |
| Additional 170 shipped | 9819068 / 9991250 | 98.276672088% | 11477 / 11764 | 97.560353621% |
| Combined 872 shipped plus generated/blank | 53865489 / 54718413 | 98.441248652% | 38837 / 42025 | 92.414039262% |

Both main and text COM gates now measure exactly the 1085 dumps named in
`apps/diagramcraft-corpus/tests/cohorts/m3-872-dumps.txt`, including 213 generated/blank
dumps. SHA256: c6e799b605821b38e35a3c3ad372af75344738364c4d23904736e6da4a390ff7.
The 872 package list has SHA256
3fe68431102e8e8f46b154bafb7bac0d5c9e38cb872c325c114aa45c72a1a2b3.
Missing members or changed membership hashes fail. CI uses the full COM population;
its cached and round-trip sample remains 50 shipped files. Later dump additions
are printed separately and cannot move the gate. At the audit, another 246 dumps
were informational: main 9751471/9999792 (97.516738348%), text 8574/9193
(93.266615903%). Cached floors and render ceilings are unchanged.

### M3.28 observed export extent (2026-10-14)

Page.Export includes visible painted geometry, its pen/marker bounds and logical
text bounds. An unpainted alignment box contributes nothing. Witness
`generated/line/pattern-00.vsdx` becomes 384 x 288, matching Visio, when the hidden
line is omitted. `generated/connector/obstacle.vsdx` becomes 529 x 307 when the
routed geometry outside its alignment box is included. The earlier alignment-only
M3.19 rule was incomplete. The M2 one-pixel tolerance over the common area remains
in force; no shape or size failure is excluded. Empty views retain the explicit
480 x 288 blank-export handling. Regression tests cover an unpainted group and both
corpus witnesses. Host diagnostics on the previous 230-file VM run show zero hard
size failures in required generated/scratch views; shipped views remain informational.
The final regenerated-input comparison is still required.

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

## M3.30 acceptance after original-file baseline (2026-10-14): partial

All 230 inputs opened. Baseline has 225/225 opened originals. Generated and blank
packages have zero residual repairs and zero residual source COM differences.
Eight files still have 4,705 repair differences; SOCIAL_DISTANCING_M retains 309
COM differences. Five scratch files have zero source COM differences. No required
repair or pixel failure is waived.

| Set | Opened | D_ours | D_base | Subtracted | Residual repairs (files / differences) | Thumbnail | MasterType | UniqueID | Residual COM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| generated | 204/204 | 1843 | 1843 | 1843 | 0 / 0 | 0 | 7 | 0 | 0 |
| blank | 1/1 | 9 | 9 | 9 | 0 / 0 | 0 | 0 | 0 | 0 |
| from-scratch | 5/5 | 725 | 35 | 15 | 5 / 693 | 5 | 7 | 0 | 0 |
| shipped-stencil | 10/10 | 781 | 767 | 767 | 1 / 14 | 1 | 0 | 0 | 0 |
| shipped-template | 10/10 | 5991 | 2005 | 1993 | 2 / 3998 | 0 | 0 | 0 | 309 |


| Render group | Images | Errors | Mean MAD (0 to 255) | Pixels over 16 | Gate |
|---|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 0.543633 | 1.071225% | PASS |
| basic | 1 | 0 | 0.443949 | 0.745332% | PASS |
| blank-pages | 1 | 0 | blank | blank | PASS |
| connector | 6 | 0 | 2.142340 | 1.225363% | PASS |
| fill | 44 | 0 | 1.112028 | 1.373298% | PASS |
| from-scratch-masters | 8 | 0 | 2.823616 | 4.452799% | FAIL |
| from-scratch-pages | 5 | 0 | 4.609743 | 3.019190% | FAIL |
| generated-masters | 9 | 0 | 1.202102 | 2.648070% | FAIL |
| group | 1 | 0 | 0.690956 | 1.251292% | PASS |
| image | 3 | 0 | 1.602828 | 1.731289% | PASS |
| line | 37 | 0 | 1.076040 | 1.030076% | PASS |
| master | 1 | 0 | 1.608085 | 1.791412% | PASS |
| page | 4 | 0 | 2.821761 | 1.171389% | PASS |
| shipped-stencil-masters | 186 | 25 | 14.604653 | 11.776394% | informational |
| shipped-template-masters | 115 | 29 | 8.206572 | 2.316836% | informational |
| shipped-template-pages | 11 | 4 | 1.917666 | 3.722501% | informational |
| text | 16 | 0 | 3.889404 | 3.318799% | PASS |


The final evaluator is used for both host metadata sets. PLANT_M has 125,169 raw
COM differences and the same 125,169 in its original baseline: residual zero.
SOCIAL_DISTANCING_M has 16,021 raw, 15,712 baseline and 309 residual differences,
including page 0 shape 337 Prop.VisWallHeight and User.3D_extrude_length, local
V=`2.6 m` versus observed `2.6`. The earlier interim residual unit counts mixed
host metadata versions; regenerating our own baseline metadata removes that skew
without changing any Visio input or observation.

Authored inherited empty Comment/ShapeKeywords now preserve empty caches rather
than the synthetic COM numeric default zero. A genuine authored string `0` stays
`0`. The regression fails before the fix and passes after it. Only the five scratch
input hashes changed; their separate 3600-second rerun opened all five.

Strict host round-trip remains 15,079,024/15,091,678 (99.916152%), all scored image
pairs identical, 568 master and 74 page equal-error pairs disclosed. Final CI has
all eight gates green; corpus, parity, roundtrip, malformed and from_scratch pass.
Fuzz: 1,387 files, 5,548 mutations, zero panics. Acceptance reporter exits 1.

Scratch allowance columns count five thumbnail additions and seven MasterType
rewrites. They remove 17 path/kind records after baseline subtraction: each new
thumbnail contributes both a part-presence record and its opaque payload record.
No UniqueID rewrite is present. The remaining 693 scratch records are repairs.

### Unchanged instance structure (M3.31)

An open/save preserves the local section, row and cell keys of a read shape,
including literal V, F and U. Inherited cache expansion is an authoring operation:
it runs for a new or edited instance, or a changed dependency. The witnesses are
SOCIAL_DISTANCING_M page1 shape 1076 (spurious Geometry1/23), shape 1057 (User.Text),
and PTSKST_M shape 27/34 (User.ValueFormat). The regression
`shipped_acceptance_shapes_preserve_local_structure_and_literal_attributes`
compares every shape in the 20 shipped acceptance originals with a fresh save.

The strict round-trip sweep after M3.31 matches 15,080,738/15,080,738 cells
(100%) over 1,387 packages, from 15,079,024/15,091,678 (99.916152%) before
the fix. No comparison or floor changed. The union denominator falls because
invented local instance cells are no longer emitted. Visio repair/COM validation
requires the subsequent changed-input rerun and is not inferred from this test.

### Authored package completion (M3.32, observed)

The five saved scratch witnesses retain `Application=Diagramcraft` and rewrite
`AppVersion=1.0000` to `16.0000`. The skeleton now writes 16.0000 and the complete
app summary, refreshing page/master counts and names before its first save.
The metric witness lists foreground names before background names. Authored
app.xml uses the observed declaration line ending and vector attribute order;
unchanged read metadata still retains its original bytes.

Created/modified and TimeEdited are fresh W3CDTF UTC values; language comes from
DocLangID. Visio adds lastModifiedBy and lastPrinted on the first save. The
reporter lists these two first-save stamps separately. Thumbnail addition is one
event including its relationship and emf Default. A generated master Icon is a
separate first-save class. All three classes have witnesses in
`corpus/_vm/out/accept/scratch__rectangle-text/saved.vsdx`.

The blank's Calibri FaceName carries UnicodeRanges `-469750017 -1040178053 9 0`,
CharSets `536871423 0`, Panose `2 15 5 2 2 2 4 3 2 4` and Flags `357`.
These are authored font metadata, with no Calibri font embedded. The complete
blank Window settings and OPC defaults/order are also authored from that witness.
The current scratch input's master Comment/ShapeKeywords are empty inherited
cells which Visio removes; the skeleton omits them. Style sheet 0's empty
Copyright/HelpTopic survive and are retained. A fresh VM rerun is still required
before claiming zero residual repairs for the completed skeleton.

The FaceName reference table spells the signature attribute `Panos`; the blank
package writes `Panose`. Authored output follows the witnessed `Panose` spelling.
The reference was read from MicrosoftDocs' `facename-element-facenames_type-complextypevisio-xml.md`; the XML witness is `corpus/blank/blank.vsdx`, document.xml.

The M3.32 three-class contract replaces the former scratch MasterType and
UniqueID waivers. Their counts remain informational columns; only matching
original-file baseline changes can subtract them. Fresh authored MasterTypes
use the values retained in the saved witnesses.

Host checks confirm all five authored app.xml parts match their saved witness
bytes. The Window tree regression also checks the root xml:space=preserve
attribute and every child against blank.vsdx, together with the Calibri FaceName
attributes. These part-level checks do not establish whole-package acceptance.

### Vertical export mapping investigation (M3.33, unresolved)

Coverage-weighted centres were measured on 66 views with separated, solid black horizontal edges, including 40 required views. Each measurement retained at least three consistent sample columns. The local diagnostic `log/m3-33-coverage-table.json` records the samples, known edge coordinates, stroke weight and fitted scale. Coordinates below use distance from the top of the evaluated extent in inches; scale is pixels per inch. These are observations, not a replacement gate.

The existing transform has `sy=(h-2)/extent_height` and top offset 1. The first candidate uses the existing horizontal scale `sx=(w-2)/extent_width` for Y and centres the vertical extent. The second uses `sy=(h-1)/extent_height`, offset 0.5. A third uses the larger original axis scale and centres Y. Additional diagnostic variants extend the vertical span by 0.25 pixel and shift its centre down 0.0625 pixel, to test the measured eighth-pixel coverage lattice. These variants are hypotheses, not an established export rule. Horizontal mapping was untouched in all trials.

None passes every required group. No production transform was changed, no ceiling was relaxed and no group was waived. The M3.33 explicit stop rule applies. The prior M3.29 profile diagnosis remains valid; it did not establish a general mapping.

An additional counterexample is `scratch__dynamic-glue/page-0.png`: at row 10 the dark-core pixels (RGB below 200) of our rectangle edges occupy x=1,192..193,288,479; Visio occupies x=5,193,287,475. Both images are 481 x 97. The horizontal displacement is outside this item's authorized vertical-only correction. Its vertical edge span also differs from the isolated rectangle despite equal image height and equal extent height.

| View | Image height | Extent height (px) | Measured top offset | Measured scale (px/in) |
|---|---:|---:|---:|---:|
| generated__basic__0001-probe / page-0 | 529 | 529.000000 | 0.988761 | 95.685789 |
| generated__connector__dynamic-glue / page-0 | 145 | 145.000000 | 0.964243 | 94.833987 |
| generated__connector__line-jumps / page-0 | 673 | 673.000000 | 1.029336 | 95.726719 |
| generated__connector__move-after / page-0 | 241 | 241.000000 | 0.486786 | 95.691855 |
| generated__connector__move-before / page-0 | 145 | 145.000000 | 0.964243 | 94.833987 |
| generated__connector__obstacle / page-0 | 307 | 307.000000 | 0.792960 | 95.610204 |
| generated__connector__static-glue / page-0 | 145 | 145.000000 | 0.964243 | 94.833987 |
| generated__fill__pattern-00 / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__line__pattern-01 / page-0 | 385 | 385.000000 | 0.973293 | 95.567572 |
| generated__line__weight-0.25 / page-0 | 385 | 384.333333 | 1.112089 | 95.682881 |
| generated__line__weight-1 / page-0 | 386 | 385.333333 | 1.115025 | 95.682504 |
| generated__master__basic-rect / page-0 | 193 | 192.320000 | 0.654144 | 95.749020 |
| generated__master__basic-rect / master-2 | 97 | 96.320000 | 0.841252 | 95.249020 |
| generated__master__basic-rect / master-4 | 97 | 96.320000 | 0.841252 | 95.249020 |
| generated__page__layers / page-0 | 193 | 193.000000 | 1.004555 | 95.125490 |
| generated__text__align-0-0 / page-0 | 289 | 289.000000 | 1.003036 | 95.416993 |
| generated__text__align-0-1 / page-0 | 289 | 289.000000 | 1.003036 | 95.416993 |
| generated__text__align-0-2 / page-0 | 291 | 290.366667 | 1.002604 | 95.500000 |
| generated__text__align-1-0 / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__align-1-1 / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__align-1-2 / page-0 | 291 | 290.366667 | 1.002604 | 95.500000 |
| generated__text__align-2-0 / page-0 | 289 | 289.000000 | 1.003036 | 95.416993 |
| generated__text__align-2-1 / page-0 | 289 | 289.000000 | 1.003036 | 95.416993 |
| generated__text__align-2-2 / page-0 | 291 | 290.366667 | 1.002604 | 95.500000 |
| generated__text__bullets / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__fields / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__margins / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__multiline-wrap / page-0 | 385 | 385.000000 | 1.002277 | 95.562745 |
| generated__text__rotated-30 / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| generated__text__sizes / page-0 | 531 | 530.900000 | 1.096898 | 95.681232 |
| generated__text__tabs / page-0 | 289 | 289.000000 | 0.751188 | 95.583660 |
| scratch__rectangle-text / page-0 | 97 | 97.000000 | 0.503911 | 95.249020 |
| scratch__dynamic-glue / page-0 | 97 | 97.000000 | 0.967279 | 94.250980 |
| scratch__dropped-master / page-0 | 97 | 96.320000 | 0.841252 | 95.249020 |
| scratch__dropped-master / master-3 | 97 | 96.320000 | 0.841252 | 95.249020 |
| scratch__metric-background-layers / page-1 | 289 | 289.000000 | 0.751188 | 95.583660 |
| scratch__metric-background-layers / page-0 | 289 | 289.000000 | 0.709624 | 95.595425 |
| scratch__three-masters / master-1 | 97 | 97.000000 | 1.009109 | 94.250980 |
| scratch__three-masters / master-2 | 97 | 97.000000 | 0.755525 | 94.750980 |
| scratch__three-masters / master-3 | 97 | 97.000000 | 0.503911 | 95.249020 |
| shipped-stencil__USTRME_U / master-4 | 27 | 26.500000 | 1.031250 | 90.000000 |
| shipped-stencil__USTRME_U / master-8 | 223 | 223.000000 | 0.945460 | 95.264964 |
| shipped-stencil__USTRME_U / master-9 | 49 | 49.000000 | 0.507823 | 94.498039 |
| shipped-stencil__EEPATH_U / master-18 | 68 | 67.225227 | 0.612746 | 94.169935 |
| shipped-stencil__EEPATH_U / master-22 | 31 | 30.346457 | 1.142799 | 92.308907 |
| shipped-stencil__EEPATH_U / master-25 | 64 | 64.025600 | 0.022434 | 94.776458 |
| shipped-stencil__FURN_VISIO2013_M / master-5 | 64 | 63.716923 | 1.129676 | 93.454715 |
| shipped-stencil__FURN_VISIO2013_M / master-14 | 58 | 57.988571 | 1.262400 | 92.363834 |
| shipped-stencil__FURN_VISIO2013_M / master-16 | 31 | 30.556220 | 0.593558 | 94.453137 |
| shipped-stencil__FURN_VISIO2013_M / master-20 | 105 | 104.257008 | 1.104597 | 94.896406 |
| shipped-template__UMLSEQUENCE_INTERACTIONFRAME_M / master-18 | 114 | 113.705827 | 0.841604 | 95.037503 |
| shipped-template__UMLSEQUENCE_INTERACTIONFRAME_M / master-26 | 190 | 189.976378 | 0.519244 | 95.503941 |
| shipped-template__UCOMME_SUBSYSTEM_R2L / master-13 | 114 | 113.705827 | 0.841604 | 95.037503 |
| shipped-template__UCOMME_SUBSYSTEM_R2L / master-34 | 96 | 95.488189 | 0.672986 | 95.296084 |
| shipped-template__NEW_ORGCH_U / master-11 | 49 | 49.000000 | 0.507823 | 94.498039 |
| shipped-template__NEW_ORGCH_U / master-12 | 49 | 49.000000 | 0.507823 | 94.498039 |
| shipped-template__PTSKST_M / master-9 | 67 | 67.000000 | 0.758482 | 94.183244 |
| shipped-template__PTSKST_M / master-21 | 97 | 97.000000 | 0.972962 | 94.271919 |
| shipped-template__PTSKST_M / master-22 | 97 | 97.000000 | 1.009109 | 94.250980 |
| shipped-template__SOCIAL_DISTANCING_M / master-16 | 190 | 189.976378 | 0.502583 | 95.504000 |
| shipped-template__SOCIAL_DISTANCING_M / master-31 | 228 | 227.771652 | 1.026389 | 95.347175 |
| shipped-template__SOCIAL_DISTANCING_M / master-32 | 153 | 152.181101 | 1.069228 | 94.726601 |
| shipped-template__SOCIAL_DISTANCING_M / master-34 | 153 | 152.181101 | 1.083764 | 94.710745 |
| shipped-template__SOCIAL_DISTANCING_M / master-35 | 228 | 227.771652 | 1.101376 | 95.300777 |
| shipped-template__SOCIAL_DISTANCING_M / master-36 | 304 | 303.362202 | 1.186951 | 95.539670 |
| shipped-template__SOCIAL_DISTANCING_M / master-63 | 39 | 38.115276 | 1.097674 | 90.807490 |

Candidate group results follow. Each pair is MAD in 0..255 and percent of common-area pixels over 16. The full candidate/worst-three inventory is in `log/m3-33-candidate-groups.json`. Original means the accepted closure-3 measurement. These private trial renders do not replace any oracle output.

| Group | Original | Horizontal scale, centred | Half-pixel margin | Horizontal plus grid trial | Original plus grid trial |
|---|---:|---:|---:|---:|---:|
| arrow | 0.543633 / 1.071225% | 0.339529 / 0.826557% | 1.080179 / 1.202958% | 0.568590 / 1.231447% | 0.470488 / 0.961404% |
| basic | 0.443949 / 0.745332% | 0.429310 / 0.734193% | 1.006456 / 1.409743% | 0.457146 / 0.891123% | 0.378041 / 0.614285% |
| connector | 2.142340 / 1.225363% | 2.944047 / 1.342807% | 2.767241 / 1.401775% | 3.463569 / 1.651282% | 2.146155 / 1.314943% |
| fill | 1.112028 / 1.373298% | 0.353900 / 0.641160% | 0.719831 / 1.206906% | 0.180841 / 0.000000% | 0.922423 / 1.351728% |
| from-scratch-masters | 2.823616 / 4.452799% FAIL | 1.542937 / 3.443171% FAIL | 2.992387 / 4.262162% FAIL | 1.077698 / 2.055466% FAIL | 2.036446 / 3.463406% FAIL |
| from-scratch-pages | 4.609743 / 3.019190% FAIL | 3.735655 / 2.490908% FAIL | 3.744781 / 3.070555% FAIL | 3.946167 / 3.074973% FAIL | 3.922172 / 2.915080% FAIL |
| generated-masters | 1.202102 / 2.648070% FAIL | 0.659890 / 2.053344% FAIL | 1.933918 / 2.633175% FAIL | 0.033830 / 0.101071% | 0.296808 / 1.208601% |
| group | 0.690956 / 1.251292% | 0.687326 / 1.249354% | 1.117804 / 1.676357% | 0.690226 / 1.213824% | 0.701505 / 1.266796% |
| image | 1.602828 / 1.731289% | 1.618461 / 1.997733% | 1.751487 / 2.057412% FAIL | 1.705902 / 2.022449% FAIL | 1.588297 / 1.347295% |
| line | 1.076040 / 1.030076% | 1.059749 / 1.117933% | 1.669901 / 1.515044% | 1.259480 / 1.290825% | 1.000730 / 0.853385% |
| master | 1.608085 / 1.791412% | 0.304458 / 0.917252% | 0.300580 / 0.911096% | 0.409296 / 0.665880% | 0.962284 / 1.781152% |
| page | 2.821761 / 1.171389% | 2.724654 / 1.415789% | 3.356183 / 1.439824% | 2.802486 / 1.138871% | 3.035145 / 1.121811% |
| text | 3.889404 / 3.318799% | 3.856522 / 3.450185% | 4.411715 / 3.838339% | 3.844625 / 2.985802% | 3.731781 / 3.086633% |

### M3.34: OPC entry identity (2026-10-14)

The acceptance comparison keys content types by Extension or PartName, and
relationships by (Type, Target). Entry order and relationship Id are not repairs.
Missing, extra, duplicate or changed entries remain failures in both directions.
When these keyed records agree, differing OPC byte hashes are reported separately
as `opc_byte_only`: quote style and attribute order alone do not change meaning.
Other opaque parts keep their byte contract. Regression:
`test_keyed_opc_order_ids_and_byte_only_disclosure`.
P3-closure-5 leaves the three small-image groups measured, ruling pending:
generated-masters, from-scratch-masters and from-scratch-pages. Their ceilings
and measured pass/fail values remain visible; no renderer change is made.

## M3.36 final COM coverage (2026-10-14)

Charles authorizes this final coverage rebase in P3-closure-5. The pinned 872
population retains its measured main and text rates, so this is coverage growth,
not an evaluator regression. The 309 additions were evaluated as a separate
cohort through the same oracle, with both partitions reconciled to the directory.

| Population | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Pinned 872 shipped plus 213 generated/blank dumps | 53,865,489 / 54,718,413 | 98.441248652% | 38,837 / 42,025 | 92.414039262% |
| Additional 309 shipped dumps | 21,088,976 / 21,434,670 | 98.387220330% | 15,354 / 16,155 | 95.041782730% |
| Final 1,181 shipped plus 213 generated/blank dumps | 74,954,465 / 76,153,083 | 98.426041399% | 54,191 / 58,180 | 93.143691990% |

The final gate pins 1,394 dump paths (1,181 shipped plus 213 generated/blank) in
`apps/diagramcraft-corpus/tests/cohorts/m3-1181-dumps.txt`, SHA256
`d571c4e903ea24a08ed1b2190d6984f8aed0add91977b08e9429b360b0a75b44`.
The package membership SHA256 is
`0370d8758e00ca1573c70d909e504bbade52acce674418f466a9b5d375f77a4a`.
Main floor: 0.984260413; sticky text COM floor: 0.931436919, each rounded
down to nine decimal places as before. Cached floors and render ceilings are
unchanged. The old 872 and earlier cohort lists remain committed as evidence.
The worker is retired: 1,181 of 1,181 shipped dumps are complete, with no partials.

## M3.38 final closure-5 measurement (2026-10-14): partial

The changed-input run selected exactly seven of 230 packages: five scratch files,
PTSKST_M and SOCIAL_DISTANCING_M. The first 3600s run completed six and timed
out (124) on SOCIAL. Its unchanged-budget single-file retry completed in about
58 minutes (exit 0). Checks were at least 900 seconds apart; baseline was not
rerun. All 230 files now have complete opened observations.

The acceptance reporter exits 1: five files retain 15 genuine repair records.
Four scratch Page 0 entries change ViewCenterX, ViewCenterY and ViewScale (12
records); AZURECOMPUTE_M has three added root-thumbnail relationship records
after our writer removed a relationship present in the original and its baseline.
No broader exception is applied. All 230 files have zero residual COM differences.
SOCIAL has D_ours 1011, D_base 1012, subtracted 1011 and residual repairs 0;
its 15712 raw COM differences all occur in the baseline, leaving residual 0.
The previous 309 added-cell differences are gone. PTSKST_M also has zero repairs
and residual COM.

All required page render groups pass the frozen ceilings and one-pixel rule.
Generated masters: MAD 1.202102, over16 2.648070%; scratch masters: 2.823616,
4.452799%; scratch pages: 4.609743, 3.019190%. These three groups remain
measured, ruling pending. There is no renderer change.

CI (8 gates), parity, corpus, current-target roundtrip, malformed and from_scratch
all exit 0. Strict cells: 15080738/15080738 (100%), every mismatch class zero.
Writer fuzz: 1387 files, 5548 mutations, zero panics. The stale standalone
roundtrip log from a reused seed Cargo target is superseded by the main-target
-current logs; CI and corpus independently agree with the corrected run.
Five source seeds were caught and restored exactly. M3 remains unaccepted
because the reporter's zero-repair condition is unmet, not because of the three
small-image groups. Phase 4 has not started.

Two independent frozen passes completed in order: closure5_review then
m3_writer_review. Each has 8 YES and 2 NO (M3.35 and M3.38-Gates), both due to
the 15 residual metadata repairs. All five production seeds are caught and
restored byte for byte. These are two completed passes, not two clean passes.
No additional source changes, comparison exceptions or render changes were
introduced to manufacture closure.

## Observed, undocumented: Page views and M3 mapping ruling (2026-10-14)

M3 render acceptance rests on the ten page-size groups, with unchanged frozen
ceilings and the one-pixel/common-area rule. Generated-masters,
from-scratch-masters and from-scratch-pages are informational until Visio's
export mapping for small images is derived, then reopen as required gates.
Corpus check prints their saved acceptance measurements explicitly as such;
the acceptance reporter still measures and prints them. No ceiling changes.

M3.39 authors Page ViewScale=-1 and ViewCenterX/Y at the page centre in internal
inches. The Page-level class "Page view attributes rewritten on save" is counted
separately beside the scratch document-level blank baseline. Only the three
exact Page attributes with finite saved numbers qualify; removals, other paths
and unrelated attributes remain repairs. Witnesses: corpus/blank/blank.vsdx,
the generated D_base Page records and Visio's saved scratch drawings.

### Observed, undocumented: read dangling relationships (M3.40)

AZURECOMPUTE_M.vssx carries a root thumbnail relationship without the target
part. Visio preserves that relationship when saving the original. We now write
read relationships as read, including this dangling link; no thumbnail part is
fabricated. Witnesses: corpus/ms-shipped/1033/AZURECOMPUTE_M.vssx and its
baseline saved file. All twenty shipped acceptance originals have identical
relationship entries after our plain save, compared by Type and Target.
Newly authored dangling links remain errors. Explicit page deletion or background
retargeting updates only the links owned by that model edit; unrelated read links
survive. Existing relationship bytes remain unchanged when their entries do.

### M3.41 changed-input acceptance (2026-10-14)

M3.39 and M3.40 close the fifteen closure-5 residual records. Six inputs changed
SHA256: rectangle-text, dynamic-glue, dropped-master, metric-background-layers,
three-masters and AZURECOMPUTE_M. The stencil also changed because fresh authored
IDs and timestamps were regenerated. Only these six ran through accept.ps1
with 3600 seconds; all 224 other input hashes and all 1664 baseline artifact
hashes stayed identical. The reporter exits 0: 230 opened, zero residual repair
records and zero residual COM cells. Dynamic glue retains two Connects rows.

| Set | Opened | D_ours | D_base | Subtracted | Residual | Thumbnail events | Icons | First-save stamps | Page view rewrites |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| generated | 204/204 | 1843 | 1843 | 1843 | 0 | 0 | 0 | 0 | 405 |
| blank | 1/1 | 9 | 9 | 9 | 0 | 0 | 0 | 0 | 2 |
| from-scratch | 5/5 | 110 | 35 | 33 | 0 | 5 | 7 | 10 | 10 |
| shipped-stencil | 10/10 | 460 | 460 | 460 | 0 | 1 | 0 | 4 | 0 |
| shipped-template | 10/10 | 1667 | 1668 | 1667 | 0 | 0 | 0 | 7 | 20 |

Class columns disclose events or records rather than interchangeable counts.
For original files the classes are ordinary baseline differences. Scratch uses
the blank document baseline plus the four explicitly authorized classes. Seven
generated MasterType rewrites and zero UniqueID rewrites are informational
columns, not additional scratch exceptions.

| Render group | Images | Errors | MAD (0 to 255) | Pixels over 16 (%) | Status |
|---|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 0.543633 | 1.071225 | PASS |
| basic | 1 | 0 | 0.443949 | 0.745332 | PASS |
| blank-pages | 1 | 0 | N/A | N/A | PASS |
| connector | 6 | 0 | 2.142340 | 1.225363 | PASS |
| fill | 44 | 0 | 1.112028 | 1.373298 | PASS |
| from-scratch-masters | 8 | 0 | 2.823616 | 4.452799 | informational |
| from-scratch-pages | 5 | 0 | 4.609743 | 3.019190 | informational |
| generated-masters | 9 | 0 | 1.202102 | 2.648070 | informational |
| group | 1 | 0 | 0.690956 | 1.251292 | PASS |
| image | 3 | 0 | 1.602828 | 1.731289 | PASS |
| line | 37 | 0 | 1.076040 | 1.030076 | PASS |
| master | 1 | 0 | 1.608085 | 1.791412 | PASS |
| page | 4 | 0 | 2.821761 | 1.171389 | PASS |
| shipped-stencil-masters | 186 | 25 | 14.604653 | 11.776394 | informational |
| shipped-template-masters | 115 | 29 | 8.206572 | 2.316836 | informational |
| shipped-template-pages | 11 | 4 | 1.917666 | 3.722501 | informational |
| text | 16 | 0 | 3.889404 | 3.318799 | PASS |

The ten page-size groups pass the unchanged ceilings with one-pixel/common-area
comparison. The blank export is checked blank. Small-image groups are retained
as measured under Charles's 2026-10-14 ruling and reopen when their mapping is
derived. Shipped render rows remain informational. No renderer or floor changed.

M3.42 final verification (2026-10-14): all host and VM acceptance commands exit
0, with two consecutive independent frozen passes of 12 YES and 0 NO. Seven
production seeds were caught and restored byte for byte. Strict literal-cache
roundtrip is 15080738/15080738 (100%); every mismatch class is zero. Identical
original-side render failures are separately disclosed: 568 masters, 74 pages.
Scored roundtrip pairs have MAD 0 and fraction-over16 0. Full corpus prints the
three saved small-image measurements above, and its ten-outlier ratchet passes.
The final cohort floors and memberships are unchanged. M3 is met under the
dated ruling, with the small-image mapping work retained as a Phase 4 gate.

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

### M4.1: session save and transactional cache updates

The session's file.open/file.save round-trip retains 998/998 strict literal cells
of generated/basic/0001-probe.vsdx. A failed model edit restores the document and
produces byte-identical saved output. Successful edits reuse the writer's existing
source/dependency comparison before re-importing the validated result. A Width
edit updates LocPinX; undo and redo restore the respective cached states.

This is host-side session evidence. No new Visio acceptance run or oracle change
is part of M4.1. Page parameters exposed by file.export are one-based ordinals;
the session's internal page selection is zero-based.

### M4.2: authored shape and group cells

Rectangle geometry follows the five-row DrawRectangle witness; ellipses use the
observed Ellipse row with A/B/C/D values and DL units. DrawLine uses endpoint-derived
Width and Angle. The rotated two-rectangle group witness stores each child PinX,
PinY, Width and Height relative to its group's Width/Height. Session grouping writes
those same formulas, and ungrouping composes the transforms into the parent frame.
Witness: corpus/generated/group/two-rects-rotated.vsdx. The exact probe rebuild
comparison is pending the P4a dynamic glue and CLI steps. No new oracle disagreement.

### M4.3: authored master instances

The BASIC_U Rectangle witness in generated/master/basic-rect.vsdx has nominal
master size1.5x1, drop PinX2/PinY6 and final size3x2. The instance writer matches
its exact local V/F/U and Inh cell set. Master imports retain IDs at or above5
when possible, otherwise remap collision-free IDs from5, including nested
MasterShape references. UniqueID follows the M3.20 form. New instances contain
only overrides, while the writer supplies the observed M3.13 inherited set.
Ten-master VM acceptance remains pending; this is host-side evidence only.

### M4.4: author connection points and static glue

Static glue targets Connection rows by universal name and records FromPart9/12 and ToPart100+row index. Witness: generated/connector/static-glue.vsdx and its immutable Connects dump.

### M4.5: evaluate dynamic glue through route host hooks

Target-transform changes invalidate _XFTRIGGER and _WALKGLUE through HostHooks. The four supplied straight drawings use overlap-facing box endpoints. The move-after witness changes BeginY to6.25 and EndX to7; Width becomes4.5.

### M4.6: write and recompute straight connector routes

Dynamic-glue, move-before, move-after and static-glue use MoveTo(0,Height/2), LineTo(Width,Height/2), a deleted third row, signed horizontal Width and Height0.25DL. Other slopes use endpoint-derived length and angle. Right-angle routing, obstacles and jumps remain4b. Individual over16 fractions are3.190144%,3.190144%,0.736069%,3.175803%; the first, second and fourth exceed2%.

### M4.7: expose session commands through the CLI

The CLI authoring path uses the same dependency-aware edit transaction and writer as direct engine calls. Page arguments are one-based ordinals; shape and master arguments retain document IDs.

### M4.8: add generator scripts and command composition regressions

Engine blank documents use the initial gray24/white25 palette observed in the probe; the lower-level skeleton retains its existing post-save palette contract. Standard authored palette literals remain indices. Layer defaults, first master names and revived Geometry rows match the supplied files. The probe elbow is authored through explicit ShapeSheet values, without implementing4b routing.

## P4a initial partial record: authored package observations and pixel differences

P4a remains partial. Exact authored comparisons retain the existing M1 exclusions:
probe rebuild 6381 eligible cells (12 excluded), static glue 5349 (2 excluded) plus
Connects, and move-after 10 endpoint/transform/control cells within 1e-6. All three
private diff files are empty. Four reroute Geometry/XForm diffs are also empty.

### Observed, undocumented: incremental authoring and imported masters

Witnesses: the private P4a probe, two-boxes and ten-master acceptance inputs and
Visio resaves under corpus/_vm/out/accept. These were generated from the supplied
basic/connector witnesses and BASFLO_U/NETSME_U, never committed as assets.

- A first explicit RGB edit replaces a Themed sentinel, registers its palette
  entry, and persists its RGB literal. Standard palette numbers 0 through 23 remain
  numbers when their existing numeric cache matches evaluation through Inh or a THEMEVAL fallback. Newly authored uncached formulas still use typed RGB serialization. The ten-master
  StyleSheet8 FillBkgnd fallback 15 is the witness for the latter rule.
- A copied master has no local Icon payload. Visio generates its Icon on save,
  counted in the existing generated-Icon class. Connection row type defaults to
  Connection; making it explicit does not invalidate unrelated cached geometry.
- Imported Connection Prompt and cleared LayerMember may have an empty literal;
  converting the empty value to numeric zero causes a save repair. Imported User
  row COLOR unit attributes are removed because these User.Value cells report
  numeric unit 32 in the supplied master dumps.
- A dropped master does not inherit a foreign document's layer membership. Page
  and master name edits refresh the app.xml heading/name vectors. Unchanged source
  packages keep their existing summary bytes.
- Redundant Inh overrides are pruned only after a dependency changes, and only if
  provider units and evaluated caches match. An explicit formula edit can similarly
  disappear when its formula, units and value equal the master. Unrelated local
  caches and originally empty/deleted rows remain preserved.
- Prop Verify retains its universal cell name but uses Ask's COM SRC 243,row,7.
  Relative Geometry coordinates and RelEllipticalArcTo D use unit 32; its C uses
  angle unit 81. The published relative elliptical-arc example is a regression.

These fixes address our serialization and COM metadata errors. They do not claim
that Visio contradicts a published reference; no oracle values were changed.

### 4b observations only

In generated/connector/obstacle.vsdx, connector 4 has Begin (2.5,6.75), End (5.75,7.5),
Width 3.25, Height 0.75 and Angle 0. Its local path is (0,0), (0.3125,0),
(0.3125,1.4375), (3.25,1.4375), (3.25,0.75). The obstacle rectangle occupies
page x 3..4 and y 5..8, so Visio's path passes above its top edge at y 8.1875.

In generated/connector/line-jumps.vsdx, horizontal connector 5 runs from
(1.5,5) to (6.5,5). Its local path is MoveTo(0,0.125),
LineTo(2.458333333333333,0.125), ArcTo(2.541666666666667,0.125)
with A = -0.041666666666666963, then LineTo(5,0.125). Vertical connector 6
runs x 4 from y 2.5 to 7.5 with Width 0.25, Height 5 and a straight path
(0.125,0) to(0.125,5), plus the unused deleted row. Both pages have
LineJumpCode 1 and RouteStyle 0; LineJumpStyle is 0 for obstacle and 1 for
line-jumps. AvoidPageBreaks is 0 and jump factors are 2/3. These are observations,
not an implementation of obstacle routing or line jumps.

### Four-file Visio acceptance, 2026-10-15

The final scheduled-task run observed three regenerated inputs and resumed the
unchanged move-after input. It exited 0, with 47,329 cells and 0 cell-read errors.
All four input hashes have opened observations. Baseline subtraction and the
existing first-save classes give the following result.

| File | Opened | Residual repairs | COM residual | Thumbnail | Generated Icon | First-save stamps | Page views |
|---|---:|---:|---:|---:|---:|---:|---:|
| p4a__probe-rebuild | 1 | 0 | 0 | 1 | 1 | 2 | 2 |
| p4a__two-boxes-glued | 1 | 0 | 0 | 1 | 1 | 2 | 2 |
| p4a__ten-masters | 1 | 0 | 0 | 1 | 11 | 2 | 2 |
| p4a__move-after | 1 | 0 | 0 | 0 | 0 | 0 | 2 |

The move-after MasterType record also occurs in its baseline. Each scratch file
has four byte-only OPC relationship changes; these are not keyed repairs. COM
eligible counts are 659, 609, 4,210 and 611 respectively. Unobserved style-local
metadata remains disclosed by the existing reporter.

The P4a reporter exits 1: two-boxes at 529 x 145 has MAD 3.453986 and over16
2.976338%, above the connector 2% ceiling. Both exports have ink between the
boxes. Move-after at 721 x 241 passes with MAD 0.632085 and over16 0.736069%.
No image ceiling or comparison rule changes.

| Informational group | Images | MAD | Over16 |
|---|---:|---:|---:|
| P4a generated-masters | 1 | 0.644702 | 2.008715% |
| P4a from-scratch-masters | 13 | 5.326534 | 7.577710% |
| P4a from-scratch-pages | 2 | 0.632586 | 1.279545% |
| M3 generated-masters | 9 | 1.202102 | 2.648070% |
| M3 from-scratch-masters | 8 | 2.823616 | 4.452799% |
| M3 from-scratch-pages | 5 | 4.609743 | 3.019190% |

The unchanged default M3 profile was also rerun in this session: exit0,
230 opened, zero residual repairs and COM differences, ten required page-size
groups passing. The new four-file profile is an additional acceptance cohort.

### Final host measurements

The full corpus command exits 0: 15,080,738/15,080,738 strict cells (100%)
across 1,387 packages, with zero writer errors and warnings. Coverage remains
1,181/1,181 shipped dumps and 1,394 pinned dumps. Main COM improves to
74,956,810/76,153,083 (98.429121%); text COM remains 54,191/58,180
(93.143692%). Shipped cached improves to 4,677,579/4,783,865
(97.778240%); cached text remains 61,748/67,797 (91.077776%). Existing
floors and denominators are unchanged. The aggregate connector group passes
(MAD 2.142340, over16 1.225363%), while its three per-file outliers remain
in the unchanged total of 10. Equal-error renderer limitations remain: 568
master scenes and 74 shipped pages fail identically before and after rewrite.

## P4a closure, 2026-10-15

P4a is met under Charles's dated straight-route ruling. Cell and Geometry/XForm
diffs agree within 1e-6; all three required and four route diff files are empty.
The existing four VM files opened with zero residual repairs and COM differences.
No input changed and no VM run was repeated. The route example and P4a reporter
now exit 0 while printing the same informational per-pair pixel measurements.
No ceiling, floor, denominator, volatile list or ten-outlier ratchet changed.

M4.10 (741b7cf) fixed Windows oversized-argument handling. Its first GitHub run,
38720669993, passed Linux and macOS but exposed a separate Windows test launcher
failure: the two-boxes test selected the WSL Bash stub. M4.10.1 (e4ec160) selects
Git for Windows Bash without skipping the script or ink assertion. Its full
`cargo xtask ci` passed on Linux, Windows and macOS in
[run 38721761383](https://github.com/charles-hood/DiagramCraft/actions/runs/38721761383).
Local acceptance used the prescribed light fmt/Clippy/CLI gates, with 15 CLI and
38 reporter tests passing. Three source seeds each failed with exit 101 and
passed with exit 0 after exact restoration. Frozen passes 3 and 4 each have
21 YES and 0 NO. Parity remains 47% over 45 Core=Y rows, with no row moves.

The M4.9 corpus figures stand without a new sweep: strict 15080738/15080738,
main COM 98.429121%, text COM 93.143692%, shipped cached 97.778240%, cached
text 91.077776%, shipped dumps 1181/1181. Connector group MAD 2.142340 and
over16 1.225363% remain within the unchanged ceiling; G8 retains ten outliers.
M4 stays open until 4b. Its estimate remains 24 to 40 engineering hours before
review; connector stroke rendering and small-image export mapping stay recorded
as separate renderer work.

## Observed, undocumented: P4b route witness sweep

These tables are observations of synthetic drawings made by the P4b recipe,
recorded before router implementation. They are not copied vendor artwork.
All values are internal units; formulas are the live COM FormulaU values.
Layout rows include the complete Page Layout and Shape Layout cell sets.
Geometry includes the resolved cells and the saved local row types/deletions.
Private originals retain their full document, master and style dumps.

### route/jump-code-2

Witness: `corpus/generated/route/jump-code-2.vsdx`, SHA-256 `a035a860c099b9d4b63dce07e165d69e4a900067615eeaccc6eaa7a4d40ced20`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 2 | 2 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 1 | 1 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 5 in | 5 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 6 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 6 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 6 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 6 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 6 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 6 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | 2.4583333333333 in | 2.458333333333333 | 73 |
| shape | 6 | 10,3,0 | Geometry1.X3 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,3,1 | Geometry1.Y3 | 2.5416666666667 in | 2.541666666666667 | 73 |
| shape | 6 | 10,3,2 | Geometry1.A3 | -0.041666666666667 in | -0.041666666666666963 | 73 |
| shape | 6 | 10,4,0 | Geometry1.X4 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,4,1 | Geometry1.Y4 | 5 in | 5 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | LineTo | 1 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | ArcTo | 0 |
| 6 | 0 | 4 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |
| 6 | BeginX | 9 | 3 | PinX | 3 |
| 6 | EndX | 12 | 4 | PinX | 3 |

### route/jump-factor-x

Witness: `corpus/generated/route/jump-factor-x.vsdx`, SHA-256 `c8ce0931f24cbdef0e6d1ddc89c4ed6e5c234f9bd8d603efd6f774aaae36248b`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 1 | 1 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 1 | 1 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 2.4375 in | 2.4375 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,0 | Geometry1.X3 | 2.5625 in | 2.5625 | 73 |
| shape | 5 | 10,3,1 | Geometry1.Y3 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,2 | Geometry1.A3 | -0.0625 in | -0.0625 | 73 |
| shape | 5 | 10,4,0 | Geometry1.X4 | 5 in | 5 | 73 |
| shape | 5 | 10,4,1 | Geometry1.Y4 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 6 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 6 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 6 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 6 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 6 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 6 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | ArcTo | 0 |
| 5 | 0 | 4 | LineTo | 0 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |
| 6 | BeginX | 9 | 3 | PinX | 3 |
| 6 | EndX | 12 | 4 | PinX | 3 |

### route/jump-gap

Witness: `corpus/generated/route/jump-gap.vsdx`, SHA-256 `3d4c29ef073780b99022eedf954ea49379a4115c5a24f903edc25f55bc741142`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 2 | 2 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 2.4583333333333 in | 2.458333333333333 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,0 | Geometry1.X3 | 2.5416666666667 in | 2.541666666666667 | 73 |
| shape | 5 | 10,3,1 | Geometry1.Y3 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,4,0 | Geometry1.X4 | 5 in | 5 | 73 |
| shape | 5 | 10,4,1 | Geometry1.Y4 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 6 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 6 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 6 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 6 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 6 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 6 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | MoveTo | 0 |
| 5 | 0 | 4 | LineTo | 0 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |
| 6 | BeginX | 9 | 3 | PinX | 3 |
| 6 | EndX | 12 | 4 | PinX | 3 |

### route/jump-square

Witness: `corpus/generated/route/jump-square.vsdx`, SHA-256 `25ad46a27935e5cf83bbc630adb52e4f579a3e98dfd95e7f880a4a850b4296b0`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 3 | 3 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 2.4583333333333 in | 2.458333333333333 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,0 | Geometry1.X3 | 2.5416666666667 in | 2.541666666666667 | 73 |
| shape | 5 | 10,3,1 | Geometry1.Y3 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,2 | Geometry1.A3 | POLYLINE(0, 0, 0.49166666666667,0.66666666666667, 0.50833333333333,0.66666666666667) | 0 | 139 |
| shape | 5 | 10,4,0 | Geometry1.X4 | 5 in | 5 | 73 |
| shape | 5 | 10,4,1 | Geometry1.Y4 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 6 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 6 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 6 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 6 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 6 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 6 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | PolylineTo | 0 |
| 5 | 0 | 4 | LineTo | 0 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |
| 6 | BeginX | 9 | 3 | PinX | 3 |
| 6 | EndX | 12 | 4 | PinX | 3 |

### route/jump-three-crossings

Witness: `corpus/generated/route/jump-three-crossings.vsdx`, SHA-256 `0b96e8113ad7cdb66381bd3cf3411c9513b41585bfa18e0fc326e3e509770888`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 1 | 1 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 2 in | 2 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 2 in | 2 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 5 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 5 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 5 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 5 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 5 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 5 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 5 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 5 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 5 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 5 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 6 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 6 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 6 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 6 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 6 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 6 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 6 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 6 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 6 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 6 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 6 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 7 | 1,1,0 | PinX | 6 in | 6 | 73 |
| shape | 7 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 7 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 7 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 7 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 7 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 7 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 7 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 7 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 7 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 7 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 7 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 7 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 7 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 7 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 7 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 7 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 7 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 7 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 7 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 7 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 7 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 7 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 7 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 7 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 7 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 7 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 7 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 7 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 7 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 7 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 7 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 7 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 7 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 7 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 7 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 7 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 7 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 7 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 7 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 7 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 7 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 7 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 7 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 7 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 7 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 7 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 7 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 7 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 7 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 7 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 7 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 7 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 7 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 7 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 7 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 7 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 7 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 7 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 7 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 7 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 7 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 7 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 7 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 7 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 7 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 7 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 7 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 7 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 7 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 7 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 7 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 7 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 7 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 7 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 7 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 7 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 7 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 7 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 7 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 7 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 7 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 7 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 8 | 1,1,0 | PinX | 6 in | 6 | 73 |
| shape | 8 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 8 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 8 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 8 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 8 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 8 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 8 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 8 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 8 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 8 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 8 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 8 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 8 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 8 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 8 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 8 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 8 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 8 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 8 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 8 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 8 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 8 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 8 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 8 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 8 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 8 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 8 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 8 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 8 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 8 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 8 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 8 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 8 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 8 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 8 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 8 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 8 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 8 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 8 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 8 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 8 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 8 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 8 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 8 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 8 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 8 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 8 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 8 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 8 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 8 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 8 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 8 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 8 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 8 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 8 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 8 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 8 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 8 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 8 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 8 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 8 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 8 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 8 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 8 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 8 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 8 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 8 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 8 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 8 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 8 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 8 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 8 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 8 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 8 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 8 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 8 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 8 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 8 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 8 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 8 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 8 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 8 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 9 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 9 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 9 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 9 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 9 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 9 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 9 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 9 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 9 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 9 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 9 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 9 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 9 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 9 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 9 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 9 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 9 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 9 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 9 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 9 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 9 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,2,0 | Geometry1.X2 | 0.47395833333333 in | 0.47395833333333304 | 73 |
| shape | 9 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,3,0 | Geometry1.X3 | 0.55729166666667 in | 0.557291666666667 | 73 |
| shape | 9 | 10,3,1 | Geometry1.Y3 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,3,2 | Geometry1.A3 | -0.041666666666667 in | -0.041666666666666963 | 73 |
| shape | 9 | 10,4,0 | Geometry1.X4 | 2.4583333333333 in | 2.458333333333333 | 73 |
| shape | 9 | 10,4,1 | Geometry1.Y4 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,5,0 | Geometry1.X5 | 2.5416666666667 in | 2.541666666666667 | 73 |
| shape | 9 | 10,5,1 | Geometry1.Y5 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,5,2 | Geometry1.A5 | -0.041666666666667 in | -0.041666666666666963 | 73 |
| shape | 9 | 10,6,0 | Geometry1.X6 | 4.4427083333333 in | 4.442708333333333 | 73 |
| shape | 9 | 10,6,1 | Geometry1.Y6 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,7,0 | Geometry1.X7 | 4.5260416666667 in | 4.526041666666667 | 73 |
| shape | 9 | 10,7,1 | Geometry1.Y7 | 0.125 in | 0.125 | 73 |
| shape | 9 | 10,7,2 | Geometry1.A7 | -0.041666666666667 in | -0.041666666666666963 | 73 |
| shape | 9 | 10,8,0 | Geometry1.X8 | 5 in | 5 | 73 |
| shape | 9 | 10,8,1 | Geometry1.Y8 | 0.125 in | 0.125 | 73 |
| shape | 9 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 9 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 9 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 9 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 9 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 9 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 9 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 9 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 9 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 9 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 9 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 9 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 9 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 9 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 9 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 9 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 9 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 9 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 9 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 9 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 9 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 9 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 9 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 9 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 9 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 9 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 9 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 9 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 9 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 9 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 9 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 9 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 9 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 9 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 9 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 9 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 9 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 9 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 9 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 9 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 9 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 9 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 9 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 9 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 9 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 9 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 9 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 9 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 9 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 9 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 9 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 9 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 9 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 9 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 9 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 9 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 9 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 9 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 10 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 2.015625 | 73 |
| shape | 10 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 10 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 10 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 10 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 10 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 10 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 10 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 10 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 10 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.015625 | 73 |
| shape | 10 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 10 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 2.015625 | 73 |
| shape | 10 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 10 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 10 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 10 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 10 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 10 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 10 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 10 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 10 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 10 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 10 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 10 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 10 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 10 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 10 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 10 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 10 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 10 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 10 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 10 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 10 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 10 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 10 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 10 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 10 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 10 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 10 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 10 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 10 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 10 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 10 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 10 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 10 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 10 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 10 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 10 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 10 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 10 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 10 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 10 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 10 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 10 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 10 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 10 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 10 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 10 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 10 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 10 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 10 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 10 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 10 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 10 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 10 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 10 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 10 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 10 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 10 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 10 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 10 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 10 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 10 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 10 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 10 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 10 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 10 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 10 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 10 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 10 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 10 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 11 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 11 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 11 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 11 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 11 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 11 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 11 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 11 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 11 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 11 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 11 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 11 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 11 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 11 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 11 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 11 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 11 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 11 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 11 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 11 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 11 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 11 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 11 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 11 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 11 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 11 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 11 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 11 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 11 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 11 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 11 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 11 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 11 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 11 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 11 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 11 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 11 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 11 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 11 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 11 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 11 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 11 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 11 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 11 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 11 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 11 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 11 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 11 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 11 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 11 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 11 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 11 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 11 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 11 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 11 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 11 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 11 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 11 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 11 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 11 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 11 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 11 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 11 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 11 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 11 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 11 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 11 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 11 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 11 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 11 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 11 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 11 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 11 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 11 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 11 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 11 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 11 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 11 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 11 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 11 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 11 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 12 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 5.984375 | 73 |
| shape | 12 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 12 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 12 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 12 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 12 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 12 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 12 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 12 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 12 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5.984375 | 73 |
| shape | 12 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 12 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5.984375 | 73 |
| shape | 12 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 12 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 12 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 12 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 12 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 12 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 12 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 12 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 12 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 12 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 12 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 12 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 12 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 12 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 12 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 12 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 12 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 12 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 12 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 12 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 12 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 12 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 12 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 12 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 12 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 12 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 12 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 12 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 12 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 12 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 12 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 12 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 12 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 12 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 12 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 12 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 12 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 12 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 12 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 12 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 12 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 12 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 12 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 12 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 12 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 12 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 12 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 12 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 12 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 12 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 12 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 12 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 12 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 12 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 12 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 12 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 12 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 12 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 12 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 12 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 12 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 12 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 12 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 12 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 12 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 12 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 12 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 12 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 12 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | LineTo | 0 |
| 5 | 0 | 4 | LineTo | 0 |
| 5 | 0 | 5 | LineTo | 0 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | LineTo | 0 |
| 6 | 0 | 4 | LineTo | 0 |
| 6 | 0 | 5 | LineTo | 0 |
| 7 | 0 | 1 | MoveTo | 0 |
| 7 | 0 | 2 | LineTo | 0 |
| 7 | 0 | 3 | LineTo | 0 |
| 7 | 0 | 4 | LineTo | 0 |
| 7 | 0 | 5 | LineTo | 0 |
| 8 | 0 | 1 | MoveTo | 0 |
| 8 | 0 | 2 | LineTo | 0 |
| 8 | 0 | 3 | LineTo | 0 |
| 8 | 0 | 4 | LineTo | 0 |
| 8 | 0 | 5 | LineTo | 0 |
| 9 | 0 | 1 | MoveTo | 0 |
| 9 | 0 | 2 | LineTo | 0 |
| 9 | 0 | 3 | ArcTo | 0 |
| 9 | 0 | 4 | LineTo | 0 |
| 9 | 0 | 5 | ArcTo | 0 |
| 9 | 0 | 6 | LineTo | 0 |
| 9 | 0 | 7 | ArcTo | 0 |
| 9 | 0 | 8 | LineTo | 0 |
| 10 | 0 | 1 | MoveTo | 0 |
| 10 | 0 | 2 | LineTo | 0 |
| 10 | 0 | 3 | LineTo | 1 |
| 11 | 0 | 1 | MoveTo | 0 |
| 11 | 0 | 2 | LineTo | 0 |
| 11 | 0 | 3 | LineTo | 1 |
| 12 | 0 | 1 | MoveTo | 0 |
| 12 | 0 | 2 | LineTo | 0 |
| 12 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 9 | BeginX | 9 | 1 | PinX | 3 |
| 9 | EndX | 12 | 2 | PinX | 3 |
| 10 | BeginX | 9 | 3 | PinX | 3 |
| 10 | EndX | 12 | 4 | PinX | 3 |
| 11 | BeginX | 9 | 5 | PinX | 3 |
| 11 | EndX | 12 | 6 | PinX | 3 |
| 12 | BeginX | 9 | 7 | PinX | 3 |
| 12 | EndX | 12 | 8 | PinX | 3 |

### route/jump-two-sides

Witness: `corpus/generated/route/jump-two-sides.vsdx`, SHA-256 `f7c068752d38898780a7f2b17edfd2af9e8d7b7cff8a9ca2a1ddf2de85115b94`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 4 | 4 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1 in | 1 | 73 |
| shape | 1 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 1 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 1 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 7 in | 7 | 73 |
| shape | 2 | 1,1,1 | PinY | 5 in | 5 | 73 |
| shape | 2 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 2 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 3 | 1,1,1 | PinY | 2 in | 2 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 4 in | 4 | 73 |
| shape | 4 | 1,1,1 | PinY | 8 in | 8 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1 in | 1 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 2.5 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 2.4583333333333 in | 2.458333333333333 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,0 | Geometry1.X3 | 2.5416666666667 in | 2.541666666666667 | 73 |
| shape | 5 | 10,3,1 | Geometry1.Y3 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,3,2 | Geometry1.A3 | POLYLINE(0, 0, 0.5,0.66666666666667) | 0 | 139 |
| shape | 5 | 10,4,0 | Geometry1.X4 | 5 in | 5 | 73 |
| shape | 5 | 10,4,1 | Geometry1.Y4 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4 | 73 |
| shape | 6 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5 | 73 |
| shape | 6 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 6 | 1,1,3 | Height | GUARD(EndY-BeginY) | 5 | 73 |
| shape | 6 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 6 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 2.5 | 73 |
| shape | 6 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 6 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 6 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 6 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4 | 73 |
| shape | 6 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 6 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 6 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 6 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 6 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 6 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 6 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 6 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 6 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 6 | 10,2,1 | Geometry1.Y2 | 5 in | 5 | 73 |
| shape | 6 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 6 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 6 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 6 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 6 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 6 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 6 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 6 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 6 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 6 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 6 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 6 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 6 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 6 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 6 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 6 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 6 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 6 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 6 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 6 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 6 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 6 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 6 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 6 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 6 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 6 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 6 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 6 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 6 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 6 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 6 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 6 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 6 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 6 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 6 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 6 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 6 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | PolylineTo | 0 |
| 5 | 0 | 4 | LineTo | 0 |
| 6 | 0 | 1 | MoveTo | 0 |
| 6 | 0 | 2 | LineTo | 0 |
| 6 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |
| 6 | BeginX | 9 | 3 | PinX | 3 |
| 6 | EndX | 12 | 4 | PinX | 3 |

### route/obstacle-below-centre

Witness: `corpus/generated/route/obstacle-below-centre.vsdx`, SHA-256 `5f608d6bc2b2c2cb3a578454b1807c01623d508427d1fb362b080d1cba5226fd`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 5.75 in | 5.75 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 2.5 in | 2.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.25 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 2.5 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 2.5 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.25 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.25 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.25 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/obstacle-objtype-zero

Witness: `corpus/generated/route/obstacle-objtype-zero.vsdx`, SHA-256 `f64a6dd1a4aabdfe2d5c7de9cbdbc4cc5182e4a95e25e26a5f397aca9c9deb69`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 6.5 in | 6.5 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 3 in | 3 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 3 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 3 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 0 | 0 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 6.75 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/obstacle-permeable

Witness: `corpus/generated/route/obstacle-permeable.vsdx`, SHA-256 `8575fec88846ac92ee9f3825cd349f12e079b02cb382a257d855147f74f5b607`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 6.5 in | 6.5 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 3 in | 3 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 3 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 3 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | TRUE | 1 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | TRUE | 1 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 6.75 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/obstacle-stacked

Witness: `corpus/generated/route/obstacle-stacked.vsdx`, SHA-256 `7f216329166ac31ab910bf6af181481c20035c9534699531d4ba20cbad50e0ba`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 5.125 in | 5.125 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 2.25 in | 2.25 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.125 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 2.25 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 2.25 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 4 | 1,1,1 | PinY | 8.125 in | 8.125 | 73 |
| shape | 4 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 4 | 1,1,3 | Height | 1.75 in | 1.75 | 73 |
| shape | 4 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.875 | 73 |
| shape | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.75 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.75 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 5 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 6.75 | 73 |
| shape | 5 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 5 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 5 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 5 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 5 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 5 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 5 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 5 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 5 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 5 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.75 | 73 |
| shape | 5 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 5 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 5 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 5 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 5 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 5 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 5 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |
| 5 | 0 | 1 | MoveTo | 0 |
| 5 | 0 | 2 | LineTo | 0 |
| 5 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 5 | BeginX | 9 | 1 | PinX | 3 |
| 5 | EndX | 12 | 2 | PinX | 3 |

### route/obstacle-touching

Witness: `corpus/generated/route/obstacle-touching.vsdx`, SHA-256 `f30be4b40ff42c8cd2267d80ce31a855e282c7719950ae937bb9f9951d55a5b0`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 5.875 in | 5.875 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 1.75 in | 1.75 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.875 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.75 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.75 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.125 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.125 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.125 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 1 | MoveTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/offset-above

Witness: `corpus/generated/route/offset-above.vsdx`, SHA-256 `be8d09e2baa8df04c61558cfa7bd113ed153a8d547ba01a9544ddef010cf02cd`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 1.75 in | 1.75 | 65 |
| shape | 2 | 1,1,1 | PinY | 9.75 in | 9.75 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 1.75 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 8.25 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 1.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.75 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 9 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 1.5 in | 1.5 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/offset-below

Witness: `corpus/generated/route/offset-below.vsdx`, SHA-256 `53f59f8873daf56b5544f28a0b71e81f3c166908188a34df10de315605ed7a50`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 1.75 in | 1.75 | 65 |
| shape | 2 | 1,1,1 | PinY | 3.75 in | 3.75 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 1.75 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 5.25 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(0.25 in) | 0.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | -1.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 0.125 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | -0.75 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 4.5 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0.125 in | 0.125 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0.125 in | 0.125 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | -1.5 in | -1.5 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/offset-left

Witness: `corpus/generated/route/offset-left.vsdx`, SHA-256 `30d36fc940ac6ff5efbef5ce5304259937bef7a99a1e6d9c66d3cf2b67968c50`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 6.75 in | 6.75 | 65 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 1.75 in | 1.75 | 65 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4.25 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 6.75 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | -3.5 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(0.25 in) | 0.25 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | -1.75 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.125 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 2.5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 6.75 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0.125 in | 0.125 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | -3.5 in | -3.5 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/offset-right-above

Witness: `corpus/generated/route/offset-right-above.vsdx`, SHA-256 `dd4c2a469445a9e2a7f4dc9b266a7b8b9be493f4e6cc3ffce34c626fe993a9a2`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.375 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 8.25 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 1.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.75 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 9 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0 in | 0 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 1.5 in | 1.5 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | 3.25 in | 3.25 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | 1.5 in | 1.5 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/offset-right-below

Witness: `corpus/generated/route/offset-right-below.vsdx`, SHA-256 `fc08da7a029e6dedde4c2f727ab219ab21700da52dd97defb863bef45f9ce1e0`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 3 in | 3 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.375 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 4.5 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | -3 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | -1.5 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 3 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0 in | 0 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | -3 in | -3 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | 3.25 in | 3.25 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | -3 in | -3 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/preference-avoid-page-breaks

Witness: `corpus/generated/route/preference-avoid-page-breaks.vsdx`, SHA-256 `08066b8b120607bac3b039d5c5b41974a51a443745a45b26bab63c83ed871494`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | TRUE | 1 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 6 in | 6 | 65 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 10 in | 10 | 65 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 7.625 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 8.25 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 1.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.75 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 9.25 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 9 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0 in | 0 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 1.5 in | 1.5 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | 3.25 in | 3.25 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | 1.5 in | 1.5 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/preference-block-size

Witness: `corpus/generated/route/preference-block-size.vsdx`, SHA-256 `27aa61f3451d829fd80b34a5223017aa4be8a8bba0801f3fc8a7dfa72bc9a450`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.5 in | 0.5 | 65 |
| page | 0 | 1,24,19 | BlockSizeY | 0.5 in | 0.5 | 65 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 6.5 in | 6.5 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 3 in | 3 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 3 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 3 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4.125 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.125 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(EndY-BeginY) | 0.75 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.375 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5.75 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 0.3125 in | 0.3125 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0 in | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | 0.3125 in | 0.3125 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | 1.4375 in | 1.4375 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | 3.25 in | 3.25 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | 1.4375 in | 1.4375 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | 3.25 in | 3.25 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | 0.75 in | 0.75 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/preference-line-to-node

Witness: `corpus/generated/route/preference-line-to-node.vsdx`, SHA-256 `71f34a3d3a61e0fad8e1fa81e5e028a7aba2ea4c6d8051bc0fcc0c2731d11d43`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.25 in | 0.25 | 65 |
| page | 0 | 1,24,17 | LineToNodeY | 0.25 in | 0.25 | 65 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | 3.5 in | 3.5 | 73 |
| shape | 3 | 1,1,1 | PinY | 6.5 in | 6.5 | 73 |
| shape | 3 | 1,1,2 | Width | 1 in | 1 | 73 |
| shape | 3 | 1,1,3 | Height | 3 in | 3 | 73 |
| shape | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.5 | 73 |
| shape | 3 | 1,1,5 | LocPinY | Height*0.5 | 1.5 | 73 |
| shape | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 3 | 73 |
| shape | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 3 | 73 |
| shape | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 4.125 | 73 |
| shape | 4 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.125 | 73 |
| shape | 4 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 4 | 1,1,3 | Height | GUARD(EndY-BeginY) | 0.75 | 73 |
| shape | 4 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 4 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.375 | 73 |
| shape | 4 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 4 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 4 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 4 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 6.75 | 73 |
| shape | 4 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5.75 | 73 |
| shape | 4 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 7.5 | 73 |
| shape | 4 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 4 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 4 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 4 | 10,2,0 | Geometry1.X2 | 0.3125 in | 0.3125 | 73 |
| shape | 4 | 10,2,1 | Geometry1.Y2 | 0 in | 0 | 73 |
| shape | 4 | 10,3,0 | Geometry1.X3 | 0.3125 in | 0.3125 | 73 |
| shape | 4 | 10,3,1 | Geometry1.Y3 | 1.4375 in | 1.4375 | 73 |
| shape | 4 | 10,4,0 | Geometry1.X4 | 3.25 in | 3.25 | 73 |
| shape | 4 | 10,4,1 | Geometry1.Y4 | 1.4375 in | 1.4375 | 73 |
| shape | 4 | 10,5,0 | Geometry1.X5 | 3.25 in | 3.25 | 73 |
| shape | 4 | 10,5,1 | Geometry1.Y5 | 0.75 in | 0.75 | 73 |
| shape | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 4 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 4 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 1 | MoveTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |
| 3 | 0 | 4 | LineTo | 0 |
| 3 | 0 | 5 | LineTo | 0 |
| 4 | 0 | 2 | LineTo | 0 |
| 4 | 0 | 3 | LineTo | 0 |
| 4 | 0 | 4 | LineTo | 0 |
| 4 | 0 | 5 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 4 | BeginX | 9 | 1 | PinX | 3 |
| 4 | EndX | 12 | 2 | PinX | 3 |

### route/style-1

Witness: `corpus/generated/route/style-1.vsdx`, SHA-256 `19f22f19e85b957241a255834651daceea795fb9ea95b862b44d0e1ddaf0b01a`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 1 | 1 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.375 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 8.25 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 3.25 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 1.5 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.625 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.75 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 1.75 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 9 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 0 in | 0 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 1.5 in | 1.5 | 73 |
| shape | 3 | 10,3,0 | Geometry1.X3 | 3.25 in | 3.25 | 73 |
| shape | 3 | 10,3,1 | Geometry1.Y3 | 1.5 in | 1.5 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/style-16

Witness: `corpus/generated/route/style-16.vsdx`, SHA-256 `977a33ebeaedf8dff78ecde289575b59c8479cbc58c9c9aa3084283469bc4bcd`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 16 | 16 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.875 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 1.40625 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.703125 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.171875 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 8.578125 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 1.40625 in | 1.40625 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 6 | 6 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/style-2

Witness: `corpus/generated/route/style-2.vsdx`, SHA-256 `5ca6422be46551f6e688b97559be8dbed6b327682e510f743ab4f996876f4ca4`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 2 | 2 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.875 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 0.75 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.375 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 8.25 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 0.75 in | 0.75 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### route/style-override

Witness: `corpus/generated/route/style-override.vsdx`, SHA-256 `9c4b51326a340b1fba658d259214ed301f4080b62a1b4ce767df57ba5a451b66`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 1 | 1 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 9 in | 9 | 65 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 1 | 1 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,1,0 | PinX | GUARD((BeginX+EndX)/2) | 3.75 | 73 |
| shape | 3 | 1,1,1 | PinY | GUARD((BeginY+EndY)/2) | 7.875 | 73 |
| shape | 3 | 1,1,2 | Width | GUARD(EndX-BeginX) | 2.5 | 73 |
| shape | 3 | 1,1,3 | Height | GUARD(EndY-BeginY) | 0.75 | 73 |
| shape | 3 | 1,1,4 | LocPinX | GUARD(Width*0.5) | 1.25 | 73 |
| shape | 3 | 1,1,5 | LocPinY | GUARD(Height*0.5) | 0.375 | 73 |
| shape | 3 | 1,1,6 | Angle | GUARD(0 deg) | 0 | 81 |
| shape | 3 | 1,1,7 | FlipX | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,1,8 | FlipY | GUARD(FALSE) | 0 | 32 |
| shape | 3 | 1,4,0 | BeginX | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 2.5 | 73 |
| shape | 3 | 1,4,1 | BeginY | _WALKGLUE(BegTrigger,EndTrigger,WalkPreference) | 7.5 | 73 |
| shape | 3 | 1,4,2 | EndX | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 5 | 73 |
| shape | 3 | 1,4,3 | EndY | _WALKGLUE(EndTrigger,BegTrigger,WalkPreference) | 8.25 | 73 |
| shape | 3 | 10,0,0 | Geometry1.NoFill | TRUE | 1 | 32 |
| shape | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 3 | 10,1,0 | Geometry1.X1 | 0 in | 0 | 73 |
| shape | 3 | 10,1,1 | Geometry1.Y1 | 0 in | 0 | 73 |
| shape | 3 | 10,2,0 | Geometry1.X2 | 2.5 in | 2.5 | 73 |
| shape | 3 | 10,2,1 | Geometry1.Y2 | 0.75 in | 0.75 | 73 |
| shape | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 3 | 1,17,13 | ObjType | 2 | 2 | 32 |
| shape | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 3 | 1,23,10 | ShapeRouteStyle | 2 | 2 | 32 |
| shape | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 3 | 1,23,21 | ShapeSplittable | 1 | 1 | 32 |
| shape | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |
| 3 | 0 | 2 | LineTo | 0 |
| 3 | 0 | 3 | LineTo | 1 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| 3 | BeginX | 9 | 1 | PinX | 3 |
| 3 | EndX | 12 | 2 | PinX | 3 |

### theme/builtin-1

Witness: `corpus/generated/theme/builtin-1.vsdx`, SHA-256 `1e4553da618bb3aadf78ce52b961778b63a38cb11aca7ce8cc668ddedb8b4397`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page | 0 | 1,1,0 | PinX |  | 0 | 32 |
| page | 0 | 1,1,1 | PinY |  | 0 | 32 |
| page | 0 | 1,1,2 | Width |  | 0 | 32 |
| page | 0 | 1,1,3 | Height |  | 0 | 32 |
| page | 0 | 1,1,4 | LocPinX |  | 0 | 32 |
| page | 0 | 1,1,5 | LocPinY |  | 0 | 32 |
| page | 0 | 1,1,6 | Angle |  | 0 | 32 |
| page | 0 | 1,1,7 | FlipX |  | 0 | 32 |
| page | 0 | 1,1,8 | FlipY |  | 0 | 32 |
| page | 0 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page | 0 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page | 0 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page | 0 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page | 0 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page | 0 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page | 0 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page | 0 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page | 0 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page | 0 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page | 0 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page | 0 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page | 0 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page | 0 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page | 0 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page | 0 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page | 0 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page | 0 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page | 0 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page | 0 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page | 0 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page | 0 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page | 0 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page | 0 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page | 0 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page | 0 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page | 0 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page | 0 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page | 0 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page | 0 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page | 0 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page | 0 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page | 0 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page | 0 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page | 0 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page | 0 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page | 0 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page | 0 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page | 0 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page | 0 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page | 0 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page | 0 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page | 0 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page | 0 | 1,24,30 | PageShapeSplit | 1 | 1 | 32 |
| page | 0 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| page | 0 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| page | 0 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| page | 0 | 1,30,3 | RotationType | 0 | 0 | 32 |
| page | 0 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| page | 0 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| page | 0 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| page | 0 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| page | 0 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| page | 0 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| page | 0 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| page | 0 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| page | 0 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| page | 0 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| page | 0 | 1,31,0 | ColorSchemeIndex | 1 | 1 | 32 |
| page | 0 | 1,31,1 | EffectSchemeIndex | 1 | 1 | 32 |
| page | 0 | 1,31,2 | ConnectorSchemeIndex | 1 | 1 | 32 |
| page | 0 | 1,31,3 | FontSchemeIndex | 1 | 1 | 32 |
| page | 0 | 1,31,4 | ThemeIndex | 1 | 1 | 32 |
| page | 0 | 1,31,5 | VariationColorIndex | 0 | 0 | 32 |
| page | 0 | 1,31,6 | VariationStyleIndex | 0 | 0 | 32 |
| page | 0 | 1,31,7 | EmbellishmentIndex | 0 | 0 | 32 |
| shape | 1 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 1 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 1 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 1 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 1 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 1 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 1 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 1 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 1 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 1 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 1 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 1 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 1 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 1 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 1 | 1,1,0 | PinX | 1.75 in | 1.75 | 73 |
| shape | 1 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 1 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 1 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 1 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 1 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 1 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 1 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 1 | 1,17,13 | ObjType | 0 | 0 | 32 |
| shape | 1 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 1 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 1 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 1 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 1 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 1 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 1 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 1 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 1 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 1 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 1 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 1 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 1 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 1 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 1 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 1 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 1 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 1 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 1 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 1 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 1 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 1 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 1 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 1 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 1 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 1 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 1 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 1 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 1 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 1 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 1 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 1 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 1 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 1 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| shape | 1 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| shape | 1 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| shape | 1 | 1,30,3 | RotationType | 0 | 0 | 32 |
| shape | 1 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| shape | 1 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| shape | 1 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| shape | 1 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| shape | 1 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| shape | 1 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| shape | 1 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| shape | 1 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| shape | 1 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| shape | 1 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| shape | 1 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| shape | 1 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |
| shape | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| shape | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| shape | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| shape | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| shape | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| shape | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| shape | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| shape | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| shape | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| shape | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| shape | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| shape | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 1.5 | 73 |
| shape | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| shape | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| shape | 2 | 1,1,0 | PinX | 5.75 in | 5.75 | 73 |
| shape | 2 | 1,1,1 | PinY | 6.75 in | 6.75 | 73 |
| shape | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,3 | Height | 1.5 in | 1.5 | 73 |
| shape | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.75 | 73 |
| shape | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| shape | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| shape | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| shape | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| shape | 2 | 1,17,13 | ObjType | 0 | 0 | 32 |
| shape | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| shape | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| shape | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| shape | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| shape | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| shape | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| shape | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| shape | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| shape | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| shape | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| shape | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| shape | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| shape | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| shape | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| shape | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| shape | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| shape | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| shape | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| shape | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| shape | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| shape | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| shape | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| shape | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| shape | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| shape | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| shape | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| shape | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| shape | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| shape | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| shape | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| shape | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| shape | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| shape | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| shape | 2 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| shape | 2 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| shape | 2 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| shape | 2 | 1,30,3 | RotationType | 0 | 0 | 32 |
| shape | 2 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| shape | 2 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| shape | 2 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| shape | 2 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| shape | 2 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| shape | 2 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| shape | 2 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| shape | 2 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| shape | 2 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| shape | 2 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| shape | 2 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| shape | 2 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |

Saved local Geometry row structure:

| Shape | Section IX | Row IX | Type | Del |
|---:|---:|---:|---|---|
| 1 | 0 | 1 | MoveTo | 0 |
| 1 | 0 | 2 | LineTo | 0 |
| 1 | 0 | 3 | LineTo | 0 |
| 1 | 0 | 4 | LineTo | 0 |
| 1 | 0 | 5 | LineTo | 0 |
| 2 | 0 | 1 | MoveTo | 0 |
| 2 | 0 | 2 | LineTo | 0 |
| 2 | 0 | 3 | LineTo | 0 |
| 2 | 0 | 4 | LineTo | 0 |
| 2 | 0 | 5 | LineTo | 0 |

Connects:

| FromSheet | FromCell | FromPart | ToSheet | ToCell | ToPart |
|---:|---|---:|---:|---|---:|
| none | | | | | |


### P4b interpretation of the sweep

All 23 route cases and `theme/builtin-1` were opened, dumped and exported by
Visio. `SetThemeVariant(1,1,2)` was refused with "Requested operation is presently
disabled"; the failed attempt has no drawing and no synthesized oracle.

The route frames remain unrotated: Width and Height are signed endpoint
differences, with 0.25DL for a zero difference. The offset-above and offset-below
witnesses use Width 0.25 and an X coordinate of 0.125 throughout the route.
LineJumpCode 2 selects the vertical crossing connector. This agrees with the
Microsoft cell reference, despite the brief's parenthetical description.
Styles 3 and 4 write PolylineTo rows with normalized POLYLINE control points;
style 2 writes a MoveTo across the gap. These are not ArcTo rows with a different
A value. The full raw values and row types are retained in the tables above.

`theme/builtin-1` has no variation style or color tables, but QuickStyle slot
100 resolves to its first base style and accent1. The themed shadow adds its
offset and blur extent to the painted bounds: Visio exports 540 by 156 pixels.
Its SVG Gaussian standard deviation is two thirds of ShapeShdwBlur. The engine
reads these parameters and colors from the input theme; it embeds no vendor
visual design. The portrait route exports also witness isotropic rather than
independent-axis canvas fitting. The existing compound-1 drawing witnesses the
transparent central third of a double stroke.

M4.13 implementation: route offsets use signed, unrotated endpoint-difference
frames, with0.25DL on a zero axis. MoveTo/LineTo Geometry is expressed in that
frame. Dynamic right-angle connectors carry ConFixedCode6; straight style2 uses0.
Observed clearance is1.5 times LineToLineX/Y. ObjType1 alignment boxes block
routes unless both ShapePermeableX/Y are true. All17 non-jump sweep cases,
connector/obstacle and four P4a straight witnesses compare within1e-6.

M4.14 observed jump rule: radius equals half LineToLine times LineJumpFactor.
Page code1 selects horizontal crossings; code2 selected vertical crossings in
the actual sweep, despite the brief's parenthetical description. ArcTo A is
negative in the positive-direction horizontal witness. Gap uses MoveTo; square
and two-side styles use PolylineTo controls. The three-crossing attachment
packing difference is retained in docs/gaps.md rather than explained by a guess.

M4.15: moving connector/obstacle.vsdx shape3 to3.5,2 yields the unobstructed
straight route; restoring3.5,6.5 restores the original Geometry/XForm. Both
transitions are session regressions. Routing evaluation uses the existing text
and theme hosts so master dimensions depending on text remain evaluable.

### Observed, undocumented: P4b route frames and acceptance

Sources: the23 route witnesses tabulated above, plus generated/connector/obstacle
and line-jumps. Coordinates are page inches. Right-angle frames are unrotated:
Width and Height are signed endpoint differences, with a0.25DL extent on a zero
axis. Geometry points subtract Begin and add half that fallback extent. The
unused third master row is deleted only when the route has two points.
Static connection-point glue keeps its own cell set; dynamic connectors carry
ConFixedCode6 (straight style2 carries0). Overlapping alignment boxes choose a
free common band; offset boxes choose perpendicular attachment segments. The
observed obstacle clearance is1.5 times LineToLineX/Y. Tight SAN adjacency reduces
inflated clearance only when the actual obstacle does not contain the endpoint.

Jump radius is half LineToLine times LineJumpFactor. Page code1 selects horizontal
crossings, code2 selects vertical crossings in the actual sweep, despite the
brief's parenthetical description. ArcTo A is negative for the positive-direction
horizontal witness, with DL units. Gap uses MoveTo; square and two-side styles
use PolylineTo controls. Local connector jump selectors override page values.
The three-crossing endpoint-packing discrepancy is retained as a named gap in
docs/gaps.md, not explained by a guessed rule.

The second authorized VM run opens rebuilt obstacle, rebuilt line-jumps and SAN.
The first two have zero residual repairs and COM differences. SAN keeps all40
routes across1344 Geometry/XForm cells, but has1336 residual serialization changes,
three Char.Font COM differences and a10pixel export-height difference. Visio adds
missing Property row cells and normalizes string formulas/units, text runs,
paragraph rows and the new master's type. It coalesces Liberation Sans face1 into
face0. Witnesses: private `_state/m4b/accept-inputs/p4b__san-topology.vsdx` and
`_vm/out/accept/p4b__san-topology/saved.vsdx`. These remain failures pending code
fixes and fresh acceptance; no oracle output or rewrite allowance was changed.

SAN residual inventory:1128 Property,6 Character,9 Paragraph,3 Text,2 FaceName,
1 MasterType and187 other differences. Visio shortens authored decimal spellings
(e.g.0.8500000000000001 to0.85), removes redundant inherited-zero Geometry cells
and label ObjType 0, and rewrites the page LineJumpCode. These remain residual
repair records; the separate1e-6 route comparison does not suppress them.

P4b verification record (2026-10-15): final local CI passes all eight gates;
main route_sweep exits0 with28 empty diffs and the named10-coordinate packing gap;
route_resave exits0 for40 SAN connectors/1344 cells/0 differences. The acceptance
report exits1 for the disclosed SAN failures. Independent frozen passes are
9 YES/6 NO and12 YES/3 NO, neither clean. M4 is not met. All three source seeds
are detected and pass after exact restoration. Final GitHub run metadata and
ordered commit hashes are retained in the private P4b report.

## Observed, undocumented: M4.19 authored Property and master originals

The two synthetic originals were saved by Visio before the closure writer changes.
They take precedence over the earlier SAN re-save for the same construct.
References: [Shape Data](https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section),
[Value](https://learn.microsoft.com/en-us/office/client-developer/visio/value-cell-shape-data-section),
[Masters.Drop](https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop).

Property rows contain Value, Prompt, Label, Format, SortKey, Type, Invisible, Verify,
DataLinked, LangID and Calendar. Explicit literals have no F attribute. Only string
Value cells carry U=STR; labels, prompts, formats and sort keys have no U.
The master has MasterType=2, no MasterShortcut, and a generated Icon. The Icon
is observed privately, never copied into source or authored output. Plain text
has no cp/pp/tp markers; explicit paragraph alignment produces pp IX=0. The SAN
re-save additionally witnesses cp IX=0 before pp IX=0 when character formatting
is local. These are formatting markers, not mandatory markers on every string.

### data/property-rows

SHA256 `11a6892c0526485c6443231aa057dd4910fc444e46590cb387dbde8d170c5342`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| page:0:Page-1/Sheet.1 | 1 | 243,0,0 | Prop.Name | Switch A | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,1 | Prop.Name.Prompt |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,2 | Prop.Name.Label | Name | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,3 | Prop.Name.Format |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,4 | Prop.Name.SortKey |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,5 | Prop.Name.Type | 0 | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,6 | Prop.Name.Invisible | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,7 | Prop.Name.Verify | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,8 | Prop.Name.DataLinked | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,9 | Prop.NameH7 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,10 | Prop.NameI7 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,11 | Prop.NameJ7 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,12 | Prop.NameK7 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,13 | Prop.NameL7 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,14 | Prop.Name.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,0,15 | Prop.Name.Calendar | 0 | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,0 | Prop.DomainID | 42 | 42 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,1 | Prop.DomainID.Prompt |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,2 | Prop.DomainID.Label | DomainID | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,3 | Prop.DomainID.Format |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,4 | Prop.DomainID.SortKey |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,5 | Prop.DomainID.Type | 2 | 2 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,6 | Prop.DomainID.Invisible | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,7 | Prop.DomainID.Verify | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,8 | Prop.DomainID.DataLinked | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,9 | Prop.DomainIDH8 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,10 | Prop.DomainIDI8 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,11 | Prop.DomainIDJ8 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,12 | Prop.DomainIDK8 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,13 | Prop.DomainIDL8 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,14 | Prop.DomainID.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,1,15 | Prop.DomainID.Calendar | 0 | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,0 | Prop.Fabric | A | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,1 | Prop.Fabric.Prompt |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,2 | Prop.Fabric.Label | Fabric | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,3 | Prop.Fabric.Format | A;B | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,4 | Prop.Fabric.SortKey |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,5 | Prop.Fabric.Type | 1 | 1 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,6 | Prop.Fabric.Invisible | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,7 | Prop.Fabric.Verify | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,8 | Prop.Fabric.DataLinked | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,9 | Prop.FabricH9 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,10 | Prop.FabricI9 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,11 | Prop.FabricJ9 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,12 | Prop.FabricK9 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,13 | Prop.FabricL9 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,14 | Prop.Fabric.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,2,15 | Prop.Fabric.Calendar | 0 | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,0 | Prop.Installed | 45000 | 45000 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,1 | Prop.Installed.Prompt |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,2 | Prop.Installed.Label | Installed | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,3 | Prop.Installed.Format | yyyy-MM-dd | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,4 | Prop.Installed.SortKey |  | 0 | 231 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,5 | Prop.Installed.Type | 5 | 5 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,6 | Prop.Installed.Invisible | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,7 | Prop.Installed.Verify | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,8 | Prop.Installed.DataLinked | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,9 | Prop.InstalledH10 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,10 | Prop.InstalledI10 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,11 | Prop.InstalledJ10 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,12 | Prop.InstalledK10 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,13 | Prop.InstalledL10 |  | 0 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,14 | Prop.Installed.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.1 | 1 | 243,3,15 | Prop.Installed.Calendar | 0 | 0 | 32 |
Text visio/pages/page1.xml shape 1: markers `[]`; content `'A\n'`.
Text visio/pages/page1.xml shape 2: markers `[]`; content `'B\n'`.
### master/from-shape

SHA256 `967adac88a1f2218931c2ed472476eab9f65d6f3815bb6060b37e4f0f9155797`.

| Scope | Shape | SRC | Cell | FormulaU | ResultIU | Units |
|---|---:|---|---|---|---|---:|
| master:2:Switch/Sheet.5 | 5 | 3,0,0 | Char.Font | THEMEVAL() | 4 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,1 | Char.Color | THEMEVAL() | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 3,0,2 | Char.Style | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,3 | Char.Case | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,4 | Char.Pos | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,5 | Char.FontScale | 100% | 1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 3,0,6 | Char._Unused6 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,7 | Char.Size | 12 pt | 0.16666666666666666 | 50 |
| master:2:Switch/Sheet.5 | 5 | 3,0,8 | Char.DblUnderline | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,9 | Char.Overline | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,10 | Char.Strikethru | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,11 | Char._Unused11 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,12 | Char._Unused12 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,13 | Char.DoubleStrikethrough | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,14 | Char._Unused14 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,15 | Char._Unused15 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,16 | Char.Letterspace | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 3,0,17 | Char.ColorTrans | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 3,0,18 | Char.Q1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,19 | Char.R1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,20 | Char.S1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,21 | Char.T1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,22 | Char.U1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,23 | Char.V1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,24 | Char.W1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,25 | Char.Z1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,26 | Char.AA1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,27 | Char.AB1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,28 | Char.AC1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,29 | Char.AD1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,30 | Char.AE1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,31 | Char.AF1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,32 | Char.AG1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,33 | Char.AH1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,34 | Char.AI1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,35 | Char.AJ1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,36 | Char.AK1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,37 | Char.AL1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,38 | Char.AM1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,39 | Char.AN1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,40 | Char.AO1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,41 | Char.AP1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,42 | Char.AQ1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,43 | Char.AR1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,44 | Char.AS1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,45 | Char.AT1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,46 | Char.AU1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,47 | Char.AV1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,48 | Char.AW1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,49 | Char.AX1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,50 | Char.AY1 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,51 | Char.AsianFont | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,52 | Char.ComplexScriptFont | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,53 | Char._Unused53 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 3,0,54 | Char.ComplexScriptSize | -100% | -1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 3,0,55 | Char.BD1 |  | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 3,0,56 | Char.BE1 |  | 0 | 141 |
| master:2:Switch/Sheet.5 | 5 | 3,0,57 | Char.LangID | 1033 | 1033 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,0 | Para.IndFirst | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 4,0,1 | Para.IndLeft | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 4,0,2 | Para.IndRight | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 4,0,3 | Para.SpLine | -120% | -1.2 | 33 |
| master:2:Switch/Sheet.5 | 5 | 4,0,4 | Para.SpBefore | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 4,0,5 | Para.SpAfter | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 4,0,6 | Para.HorzAlign | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,7 | Para.Bullet | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,8 | Para.BulletStr |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 4,0,9 | Para.BulletFont | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,10 | Para._Unused10 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,11 | Para.BulletFontSize | -100% | -1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 4,0,12 | Para.TextPosAfterBullet | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 4,0,13 | Para.Flags | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 4,0,14 | Para.M1 |  | 0 | 141 |
| master:2:Switch/Sheet.5 | 5 | 5,0,0 | Tabs.X1 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,3,1 | Geometry1.Y3 | Height*1 | 0.75 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,4,1 | Geometry1.Y4 | Height*1 | 0.75 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 248,0,0 | LineGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,0,1 | LineGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,0,2 | LineGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,1,0 | LineGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,1,1 | LineGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,1,2 | LineGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,2,0 | LineGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,2,1 | LineGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,2,2 | LineGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,3,0 | LineGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,3,1 | LineGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,3,2 | LineGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,4,0 | LineGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,4,1 | LineGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,4,2 | LineGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,5,0 | LineGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,5,1 | LineGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,5,2 | LineGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,6,0 | LineGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,6,1 | LineGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,6,2 | LineGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,7,0 | LineGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,7,1 | LineGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,7,2 | LineGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,8,0 | LineGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,8,1 | LineGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,8,2 | LineGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,9,0 | LineGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 248,9,1 | LineGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 248,9,2 | LineGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,0,0 | FillGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,0,1 | FillGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,0,2 | FillGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,1,0 | FillGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,1,1 | FillGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,1,2 | FillGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,2,0 | FillGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,2,1 | FillGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,2,2 | FillGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,3,0 | FillGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,3,1 | FillGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,3,2 | FillGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,4,0 | FillGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,4,1 | FillGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,4,2 | FillGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,5,0 | FillGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,5,1 | FillGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,5,2 | FillGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,6,0 | FillGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,6,1 | FillGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,6,2 | FillGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,7,0 | FillGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,7,1 | FillGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,7,2 | FillGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,8,0 | FillGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,8,1 | FillGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,8,2 | FillGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,9,0 | FillGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 249,9,1 | FillGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 249,9,2 | FillGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,1,0 | PinX | 0.75520833333333 in | 0.75520833333333348 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,1 | PinY | 0.38020833333333 in | 0.38020833333333304 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,3 | Height | 0.75 in | 0.75 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,5 | LocPinY | Height*0.5 | 0.375 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,1,9 | ResizeMode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,0 | LineWeight | THEMEVAL() | 0.010416666666666666 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,2,1 | LineColor | THEMEVAL() | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,2,2 | LinePattern | THEMEVAL() | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,3 | Rounding | THEMEVAL() | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,2,4 | EndArrowSize | 2 | 2 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,5 | BeginArrow | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,6 | EndArrow | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,7 | LineCap | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,8 | BeginArrowSize | 2 | 2 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,2,9 | LineColorTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,2,10 | CompoundType | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,3,0 | FillForegnd | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,3,1 | FillBkgnd | THEMEVAL() | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,3,2 | FillPattern | THEMEVAL() | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,3,3 | ShdwForegnd | THEMEVAL() | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,3,4 | ShdwBkgnd | 1 | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,3,5 | ShdwPattern | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,3,6 | FillForegndTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,3,7 | FillBkgndTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,3,8 | ShdwForegndTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,3,9 | ShdwBkgndTrans | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,3,10 | ShapeShdwType | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,3,11 | ShapeShdwOffsetX | THEMEVAL() | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,3,12 | ShapeShdwOffsetY | THEMEVAL() | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,3,13 | ShapeShdwObliqueAngle | THEMEVAL() | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,3,14 | ShapeShdwScaleFactor | THEMEVAL() | 1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,3,15 | ShapeShdwBlur | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,3,16 | ShapeShdwShow | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,0 | TheData |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,1 | TheText |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,2 | EventDblClick |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,3 | EventXFMod |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,4 | EventDrop |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,5 | _EventPatMas |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,6 | EventTextOverflow |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,7 | Sect1.F6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,8 | Sect1.G6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,9 | Sect1.H6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,10 | Sect1.I6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,11 | Sect1.J6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,12 | Sect1.K6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,13 | Sect1.L6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,14 | Sect1.M6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,15 | Sect1.N6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,16 | Sect1.O6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,17 | Sect1.P6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,18 | Sect1.Q6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,19 | Sect1.R6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,20 | Sect1.S6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,21 | Sect1.T6 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,5,22 | EventMultiDrop |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,0 | EnableLineProps | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,1 | EnableFillProps | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,2 | EnableTextProps | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,3 | HideForApply | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,4 | Sect1.C9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,5 | Sect1.D9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,6 | Sect1.E9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,7 | Sect1.F9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,8 | Sect1.G9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,9 | Sect1.H9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,8,10 | Sect1.I9 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,0 | PageWidth |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,1 | PageHeight |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,2 | ShdwOffsetX |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,3 | ShdwOffsetY |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,4 | PageScale |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,5 | DrawingScale |  | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,10,6 | DrawingSizeType |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,7 | DrawingScaleType |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,8 | _Unused5F |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,9 | _Unused60 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,10 | Sect1.I11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,11 | Sect1.J11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,12 | _UpdateNotes |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,13 | Sect1.L11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,14 | Sect1.M11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,15 | Sect1.N11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,16 | Sect1.O11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,17 | Sect1.P11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,18 | Sect1.Q11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,19 | Sect1.R11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,20 | Sect1.S11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,21 | Sect1.T11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,22 | Sect1.U11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,23 | Sect1.V11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,24 | Sect1.W11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,25 | Sect1.Z11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,26 | InhibitSnap |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,27 | PageLockReplace |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,28 | PageLockDuplicate |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,29 | Sect1.AD11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,30 | Sect1.AE11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,31 | Sect1.AF11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,32 | Sect1.AG11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,33 | Sect1.AH11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,34 | UIVisibility |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,35 | ShdwType |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,36 | ShdwObliqueAngle |  | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,10,37 | ShdwScaleFactor |  | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,10,38 | DrawingResizeType |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,10,39 | Sect1.AN11 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,0 | LeftMargin | 4 pt | 0.055555555555555552 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,11,1 | RightMargin | 4 pt | 0.055555555555555552 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,11,2 | TopMargin | 4 pt | 0.055555555555555552 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,11,3 | BottomMargin | 4 pt | 0.055555555555555552 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,11,4 | VerticalAlign | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,5 | TextBkgnd | 0 | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,11,6 | DefaultTabStop | 0.5 in | 0.5 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,11,7 | TextMaxDepth | 4294967295 | 4294967295 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,8 | TextGeometry | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,9 | TextContainer | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,10 | TextDirection | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,11 | TextBkgndTrans | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,11,12 | _UnusedE5 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,11,13 | _UnusedE6 |  | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,11,14 | _UnusedE7 |  | 0 | 141 |
| master:2:Switch/Sheet.5 | 5 | 1,15,0 | LockWidth | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,1 | LockHeight | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,2 | LockMoveX | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,3 | LockMoveY | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,4 | LockAspect | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,5 | LockDelete | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,6 | LockBegin | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,7 | LockEnd | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,8 | LockRotate | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,9 | LockCrop | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,10 | LockVtxEdit | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,11 | LockTextEdit | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,12 | LockFormat | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,13 | LockGroup | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,14 | LockCalcWH | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,15 | LockSelect | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,16 | LockCustProp | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,17 | LockFromGroupFormat | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,18 | LockThemeColors | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,19 | LockThemeEffects | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,20 | LockThemeConnectors | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,21 | LockThemeFonts | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,22 | LockThemeIndex | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,23 | LockReplace | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,15,24 | LockVariation | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,16,0 | HelpTopic |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,16,1 | Copyright |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,16,2 | Sect1.A17 |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,16,3 | Sect1.B17 |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,17,0 | NoObjHandles | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,1 | NonPrinting | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,2 | NoCtlHandles | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,3 | NoAlignBox | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,4 | UpdateAlignBox | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,5 | HideText | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,6 | _Unused83 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,7 | ShapeTabStop | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,8 | DynFeedback | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,9 | GlueType | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,11 | BegTrigger |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,12 | EndTrigger |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,13 | ObjType | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,14 | _Unused78 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,15 | _Unused79 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,16 | Comment |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,17,17 | IsDropSource | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,18 | NoLiveDynamics | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,19 | LocalizeMerge | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,20 | NoProofing | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,21 | Sect1.T18 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,22 | Sect1.U18 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,23 | Sect1.V18 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,24 | Sect1.W18 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,25 | Calendar | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,26 | LangID | 1033 | 1033 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,27 | ShapeKeywords |  | 0 | 231 |
| master:2:Switch/Sheet.5 | 5 | 1,17,28 | DropOnPageScale | 100% | 1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,17,29 | Sect1.AD18 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,17,30 | Sect1.AE18 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,18,0 | XRulerDensity | 32 | 32 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,18,1 | YRulerDensity | 32 | 32 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,18,2 | XRulerSpacing | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,3 | YRulerSpacing | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,4 | XRulerOrigin | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,5 | YRulerOrigin | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,6 | XGridDensity | 8 | 8 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,18,7 | YGridDensity | 8 | 8 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,18,8 | XGridSpacing | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,9 | YGridSpacing | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,10 | XGridOrigin | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,18,11 | YGridOrigin | 0 in | 0 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,21,0 | Gamma | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,21,1 | Contrast | 50% | 0.5 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,2 | Brightness | 50% | 0.5 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,3 | Sharpen | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,4 | Blur | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,5 | Denoise | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,6 | Transparency | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,7 | _Unused8F | 0% | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,21,8 | _Unused90 | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,0 | SelectMode | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,1 | DisplayMode | 2 | 2 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,2 | IsDropTarget | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,3 | IsSnapTarget | TRUE | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,4 | IsTextEditTarget | TRUE | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,5 | DontMoveChildren | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,6 | Sect1.E23 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,7 | Sect1.F23 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,8 | Sect1.G23 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,22,9 | Sect1.H23 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,0 | PageLeftMargin | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,25,1 | PageRightMargin | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,25,2 | PageTopMargin | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,25,3 | PageBottomMargin | 0.25 in | 0.25 | 73 |
| master:2:Switch/Sheet.5 | 5 | 1,25,4 | ScaleX | 100% | 1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,25,5 | ScaleY | 100% | 1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,25,6 | PagesX | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,7 | PagesY | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,8 | CenterX | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,9 | CenterY | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,10 | OnPage | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,11 | PrintGrid | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,12 | Sect1.K26 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,13 | Sect1.L26 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,14 | Sect1.M26 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,15 | Sect1.N26 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,16 | PrintPageOrientation | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,17 | PaperKind | 1 | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,25,18 | PaperSource | 7 | 7 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,0 | LineGradientDir | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,1 | LineGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,26,2 | FillGradientDir | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,3 | FillGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,26,4 | LineGradientEnabled | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,5 | FillGradientEnabled | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,6 | RotateGradientWithShape | THEMEVAL() | 1 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,7 | UseGroupGradient | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,8 | Sect1.G27 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,9 | Sect1.H27 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,10 | Sect1.I27 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,26,11 | Sect1.J27 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,0 | QuickStyleLineColor | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,1 | QuickStyleFillColor | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,2 | QuickStyleShadowColor | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,3 | QuickStyleFontColor | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,4 | QuickStyleLineMatrix | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,5 | QuickStyleFillMatrix | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,6 | QuickStyleEffectsMatrix | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,7 | QuickStyleFontMatrix | 100 | 100 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,8 | QuickStyleType | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,27,9 | QuickStyleVariation | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,0 | ReflectionTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,28,1 | ReflectionSize | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,28,2 | ReflectionDist | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,28,3 | ReflectionBlur | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,28,4 | GlowColor | THEMEVAL() | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,28,5 | GlowColorTrans | THEMEVAL() | 0 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,28,6 | GlowSize | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,28,7 | SoftEdgesSize | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,28,8 | SketchSeed | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,9 | SketchEnabled | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,10 | SketchAmount | THEMEVAL() | 5 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,11 | SketchLineWeight | THEMEVAL() | 0.041666666666666664 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,28,12 | SketchLineChange | THEMEVAL() | 0.14 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,28,13 | SketchFillChange | THEMEVAL() | 0.1 | 33 |
| master:2:Switch/Sheet.5 | 5 | 1,28,14 | Sect1.M29 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,15 | Sect1.N29 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,28,16 | Sect1.O29 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,29,0 | BevelTopType | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,29,1 | BevelTopWidth | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,2 | BevelTopHeight | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,3 | BevelBottomType | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,29,4 | BevelBottomWidth | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,5 | BevelBottomHeight | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,6 | BevelDepthColor | 1 | 1 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,29,7 | BevelDepthSize | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,8 | BevelContourColor | THEMEVAL() | 0 | 251 |
| master:2:Switch/Sheet.5 | 5 | 1,29,9 | BevelContourSize | THEMEVAL() | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,29,10 | BevelMaterialType | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,29,11 | BevelLightingType | THEMEVAL() | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,29,12 | BevelLightingAngle | THEMEVAL() | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,30,3 | RotationType | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| master:2:Switch/Sheet.5 | 5 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| master:2:Switch/Sheet.5 | 5 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,32,0 | ReplaceLockShapeData | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,32,1 | ReplaceLockText | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,32,2 | ReplaceLockFormat | 0 | 0 | 32 |
| master:2:Switch/Sheet.5 | 5 | 1,32,3 | ReplaceCopyCells |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,0 | Char.Font | THEMEVAL() | 4 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,1 | Char.Color | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch | 2 | 3,0,2 | Char.Style | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,3 | Char.Case | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,4 | Char.Pos | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,5 | Char.FontScale | 100% | 1 | 33 |
| page:0:Page-1/Switch | 2 | 3,0,6 | Char._Unused6 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,7 | Char.Size | 12 pt | 0.16666666666666666 | 50 |
| page:0:Page-1/Switch | 2 | 3,0,8 | Char.DblUnderline | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,9 | Char.Overline | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,10 | Char.Strikethru | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,11 | Char._Unused11 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,12 | Char._Unused12 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,13 | Char.DoubleStrikethrough | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,14 | Char._Unused14 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,15 | Char._Unused15 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,16 | Char.Letterspace | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 3,0,17 | Char.ColorTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 3,0,18 | Char.Q1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,19 | Char.R1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,20 | Char.S1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,21 | Char.T1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,22 | Char.U1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,23 | Char.V1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,24 | Char.W1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,25 | Char.Z1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,26 | Char.AA1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,27 | Char.AB1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,28 | Char.AC1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,29 | Char.AD1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,30 | Char.AE1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,31 | Char.AF1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,32 | Char.AG1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,33 | Char.AH1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,34 | Char.AI1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,35 | Char.AJ1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,36 | Char.AK1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,37 | Char.AL1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,38 | Char.AM1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,39 | Char.AN1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,40 | Char.AO1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,41 | Char.AP1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,42 | Char.AQ1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,43 | Char.AR1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,44 | Char.AS1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,45 | Char.AT1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,46 | Char.AU1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,47 | Char.AV1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,48 | Char.AW1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,49 | Char.AX1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,50 | Char.AY1 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,51 | Char.AsianFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,52 | Char.ComplexScriptFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,53 | Char._Unused53 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 3,0,54 | Char.ComplexScriptSize | -100% | -1 | 33 |
| page:0:Page-1/Switch | 2 | 3,0,55 | Char.BD1 |  | 0 | 50 |
| page:0:Page-1/Switch | 2 | 3,0,56 | Char.BE1 |  | 0 | 141 |
| page:0:Page-1/Switch | 2 | 3,0,57 | Char.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,0 | Para.IndFirst | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 4,0,1 | Para.IndLeft | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 4,0,2 | Para.IndRight | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 4,0,3 | Para.SpLine | -120% | -1.2 | 33 |
| page:0:Page-1/Switch | 2 | 4,0,4 | Para.SpBefore | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 4,0,5 | Para.SpAfter | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 4,0,6 | Para.HorzAlign | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,7 | Para.Bullet | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,8 | Para.BulletStr |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 4,0,9 | Para.BulletFont | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,10 | Para._Unused10 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,11 | Para.BulletFontSize | -100% | -1 | 33 |
| page:0:Page-1/Switch | 2 | 4,0,12 | Para.TextPosAfterBullet | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 4,0,13 | Para.Flags | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 4,0,14 | Para.M1 |  | 0 | 141 |
| page:0:Page-1/Switch | 2 | 5,0,0 | Tabs.X1 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| page:0:Page-1/Switch | 2 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| page:0:Page-1/Switch | 2 | 10,3,1 | Geometry1.Y3 | Height*1 | 0.75 | 73 |
| page:0:Page-1/Switch | 2 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 10,4,1 | Geometry1.Y4 | Height*1 | 0.75 | 73 |
| page:0:Page-1/Switch | 2 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| page:0:Page-1/Switch | 2 | 248,0,0 | LineGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,0,1 | LineGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,0,2 | LineGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,1,0 | LineGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,1,1 | LineGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,1,2 | LineGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,2,0 | LineGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,2,1 | LineGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,2,2 | LineGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,3,0 | LineGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,3,1 | LineGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,3,2 | LineGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,4,0 | LineGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,4,1 | LineGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,4,2 | LineGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,5,0 | LineGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,5,1 | LineGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,5,2 | LineGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,6,0 | LineGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,6,1 | LineGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,6,2 | LineGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,7,0 | LineGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,7,1 | LineGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,7,2 | LineGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,8,0 | LineGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,8,1 | LineGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,8,2 | LineGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,9,0 | LineGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 248,9,1 | LineGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 248,9,2 | LineGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,0,0 | FillGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,0,1 | FillGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,0,2 | FillGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,1,0 | FillGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,1,1 | FillGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,1,2 | FillGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,2,0 | FillGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,2,1 | FillGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,2,2 | FillGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,3,0 | FillGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,3,1 | FillGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,3,2 | FillGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,4,0 | FillGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,4,1 | FillGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,4,2 | FillGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,5,0 | FillGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,5,1 | FillGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,5,2 | FillGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,6,0 | FillGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,6,1 | FillGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,6,2 | FillGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,7,0 | FillGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,7,1 | FillGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,7,2 | FillGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,8,0 | FillGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,8,1 | FillGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,8,2 | FillGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,9,0 | FillGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 249,9,1 | FillGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 249,9,2 | FillGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,1,0 | PinX | 2 in | 2 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,1 | PinY | 6 in | 6 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,3 | Height | 0.75 in | 0.75 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,5 | LocPinY | Height*0.5 | 0.375 | 73 |
| page:0:Page-1/Switch | 2 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,1,9 | ResizeMode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,0 | LineWeight | THEMEVAL() | 0.010416666666666666 | 50 |
| page:0:Page-1/Switch | 2 | 1,2,1 | LineColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,2,2 | LinePattern | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,3 | Rounding | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,2,4 | EndArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,5 | BeginArrow | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,6 | EndArrow | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,7 | LineCap | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,8 | BeginArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Switch | 2 | 1,2,9 | LineColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,2,10 | CompoundType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,3,0 | FillForegnd | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 1,3,1 | FillBkgnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,3,2 | FillPattern | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,3,3 | ShdwForegnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,3,4 | ShdwBkgnd | 1 | 1 | 251 |
| page:0:Page-1/Switch | 2 | 1,3,5 | ShdwPattern | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,3,6 | FillForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,3,7 | FillBkgndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,3,8 | ShdwForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,3,9 | ShdwBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,3,10 | ShapeShdwType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,3,11 | ShapeShdwOffsetX | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,3,12 | ShapeShdwOffsetY | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,3,13 | ShapeShdwObliqueAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,3,14 | ShapeShdwScaleFactor | THEMEVAL() | 1 | 33 |
| page:0:Page-1/Switch | 2 | 1,3,15 | ShapeShdwBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,3,16 | ShapeShdwShow | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,0 | TheData |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,1 | TheText |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,2 | EventDblClick |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,3 | EventXFMod |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,4 | EventDrop |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,5 | _EventPatMas |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,6 | EventTextOverflow |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,7 | Sect1.F6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,8 | Sect1.G6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,9 | Sect1.H6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,10 | Sect1.I6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,11 | Sect1.J6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,12 | Sect1.K6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,13 | Sect1.L6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,14 | Sect1.M6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,15 | Sect1.N6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,16 | Sect1.O6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,17 | Sect1.P6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,18 | Sect1.Q6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,19 | Sect1.R6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,20 | Sect1.S6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,21 | Sect1.T6 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,5,22 | EventMultiDrop |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,0 | EnableLineProps | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,1 | EnableFillProps | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,2 | EnableTextProps | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,3 | HideForApply | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,4 | Sect1.C9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,5 | Sect1.D9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,6 | Sect1.E9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,7 | Sect1.F9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,8 | Sect1.G9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,9 | Sect1.H9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,8,10 | Sect1.I9 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,0 | PageWidth |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,1 | PageHeight |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,2 | ShdwOffsetX |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,3 | ShdwOffsetY |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,4 | PageScale |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,5 | DrawingScale |  | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,10,6 | DrawingSizeType |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,7 | DrawingScaleType |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,8 | _Unused5F |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,9 | _Unused60 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,10 | Sect1.I11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,11 | Sect1.J11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,12 | _UpdateNotes |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,13 | Sect1.L11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,14 | Sect1.M11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,15 | Sect1.N11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,16 | Sect1.O11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,17 | Sect1.P11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,18 | Sect1.Q11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,19 | Sect1.R11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,20 | Sect1.S11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,21 | Sect1.T11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,22 | Sect1.U11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,23 | Sect1.V11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,24 | Sect1.W11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,25 | Sect1.Z11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,26 | InhibitSnap |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,27 | PageLockReplace |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,28 | PageLockDuplicate |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,29 | Sect1.AD11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,30 | Sect1.AE11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,31 | Sect1.AF11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,32 | Sect1.AG11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,33 | Sect1.AH11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,34 | UIVisibility |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,35 | ShdwType |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,36 | ShdwObliqueAngle |  | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,10,37 | ShdwScaleFactor |  | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,10,38 | DrawingResizeType |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,10,39 | Sect1.AN11 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,0 | LeftMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch | 2 | 1,11,1 | RightMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch | 2 | 1,11,2 | TopMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch | 2 | 1,11,3 | BottomMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch | 2 | 1,11,4 | VerticalAlign | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,5 | TextBkgnd | 0 | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,11,6 | DefaultTabStop | 0.5 in | 0.5 | 73 |
| page:0:Page-1/Switch | 2 | 1,11,7 | TextMaxDepth | 4294967295 | 4294967295 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,8 | TextGeometry | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,9 | TextContainer | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,10 | TextDirection | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,11 | TextBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,11,12 | _UnusedE5 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,11,13 | _UnusedE6 |  | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,11,14 | _UnusedE7 |  | 0 | 141 |
| page:0:Page-1/Switch | 2 | 1,15,0 | LockWidth | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,1 | LockHeight | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,2 | LockMoveX | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,3 | LockMoveY | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,4 | LockAspect | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,5 | LockDelete | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,6 | LockBegin | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,7 | LockEnd | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,8 | LockRotate | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,9 | LockCrop | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,10 | LockVtxEdit | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,11 | LockTextEdit | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,12 | LockFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,13 | LockGroup | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,14 | LockCalcWH | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,15 | LockSelect | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,16 | LockCustProp | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,17 | LockFromGroupFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,18 | LockThemeColors | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,19 | LockThemeEffects | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,20 | LockThemeConnectors | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,21 | LockThemeFonts | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,22 | LockThemeIndex | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,23 | LockReplace | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,15,24 | LockVariation | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,16,0 | HelpTopic |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,16,1 | Copyright |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,16,2 | Sect1.A17 |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,16,3 | Sect1.B17 |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,17,0 | NoObjHandles | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,1 | NonPrinting | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,2 | NoCtlHandles | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,3 | NoAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,4 | UpdateAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,5 | HideText | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,6 | _Unused83 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,7 | ShapeTabStop | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,8 | DynFeedback | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,9 | GlueType | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,11 | BegTrigger |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,12 | EndTrigger |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,14 | _Unused78 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,15 | _Unused79 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,16 | Comment |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,17,17 | IsDropSource | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,18 | NoLiveDynamics | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,19 | LocalizeMerge | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,20 | NoProofing | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,21 | Sect1.T18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,22 | Sect1.U18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,23 | Sect1.V18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,24 | Sect1.W18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,25 | Calendar | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,26 | LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,27 | ShapeKeywords |  | 0 | 231 |
| page:0:Page-1/Switch | 2 | 1,17,28 | DropOnPageScale | 100% | 1 | 33 |
| page:0:Page-1/Switch | 2 | 1,17,29 | Sect1.AD18 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,17,30 | Sect1.AE18 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,18,0 | XRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Switch | 2 | 1,18,1 | YRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Switch | 2 | 1,18,2 | XRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,3 | YRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,4 | XRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,5 | YRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,6 | XGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Switch | 2 | 1,18,7 | YGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Switch | 2 | 1,18,8 | XGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,9 | YGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,10 | XGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,18,11 | YGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch | 2 | 1,21,0 | Gamma | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,21,1 | Contrast | 50% | 0.5 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,2 | Brightness | 50% | 0.5 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,3 | Sharpen | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,4 | Blur | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,5 | Denoise | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,6 | Transparency | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,7 | _Unused8F | 0% | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,21,8 | _Unused90 | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,0 | SelectMode | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,1 | DisplayMode | 2 | 2 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,2 | IsDropTarget | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,3 | IsSnapTarget | TRUE | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,4 | IsTextEditTarget | TRUE | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,5 | DontMoveChildren | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,6 | Sect1.E23 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,7 | Sect1.F23 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,8 | Sect1.G23 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,22,9 | Sect1.H23 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch | 2 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,0 | PageLeftMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,25,1 | PageRightMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,25,2 | PageTopMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,25,3 | PageBottomMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch | 2 | 1,25,4 | ScaleX | 100% | 1 | 33 |
| page:0:Page-1/Switch | 2 | 1,25,5 | ScaleY | 100% | 1 | 33 |
| page:0:Page-1/Switch | 2 | 1,25,6 | PagesX | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,7 | PagesY | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,8 | CenterX | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,9 | CenterY | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,10 | OnPage | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,11 | PrintGrid | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,12 | Sect1.K26 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,13 | Sect1.L26 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,14 | Sect1.M26 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,15 | Sect1.N26 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,16 | PrintPageOrientation | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,17 | PaperKind | 1 | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,25,18 | PaperSource | 7 | 7 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,0 | LineGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,1 | LineGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Switch | 2 | 1,26,2 | FillGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,3 | FillGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Switch | 2 | 1,26,4 | LineGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,5 | FillGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,6 | RotateGradientWithShape | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,7 | UseGroupGradient | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,8 | Sect1.G27 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,9 | Sect1.H27 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,10 | Sect1.I27 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,26,11 | Sect1.J27 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,0 | QuickStyleLineColor | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,1 | QuickStyleFillColor | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,2 | QuickStyleShadowColor | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,3 | QuickStyleFontColor | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,4 | QuickStyleLineMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,5 | QuickStyleFillMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,6 | QuickStyleEffectsMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,7 | QuickStyleFontMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,8 | QuickStyleType | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,27,9 | QuickStyleVariation | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,0 | ReflectionTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,28,1 | ReflectionSize | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,28,2 | ReflectionDist | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,28,3 | ReflectionBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,28,4 | GlowColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch | 2 | 1,28,5 | GlowColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch | 2 | 1,28,6 | GlowSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,28,7 | SoftEdgesSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,28,8 | SketchSeed | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,9 | SketchEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,10 | SketchAmount | THEMEVAL() | 5 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,11 | SketchLineWeight | THEMEVAL() | 0.041666666666666664 | 50 |
| page:0:Page-1/Switch | 2 | 1,28,12 | SketchLineChange | THEMEVAL() | 0.14 | 33 |
| page:0:Page-1/Switch | 2 | 1,28,13 | SketchFillChange | THEMEVAL() | 0.1 | 33 |
| page:0:Page-1/Switch | 2 | 1,28,14 | Sect1.M29 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,15 | Sect1.N29 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,28,16 | Sect1.O29 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,29,0 | BevelTopType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,29,1 | BevelTopWidth | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,2 | BevelTopHeight | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,3 | BevelBottomType | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,29,4 | BevelBottomWidth | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,5 | BevelBottomHeight | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,6 | BevelDepthColor | 1 | 1 | 251 |
| page:0:Page-1/Switch | 2 | 1,29,7 | BevelDepthSize | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,8 | BevelContourColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch | 2 | 1,29,9 | BevelContourSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,29,10 | BevelMaterialType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,29,11 | BevelLightingType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,29,12 | BevelLightingAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,30,3 | RotationType | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| page:0:Page-1/Switch | 2 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| page:0:Page-1/Switch | 2 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch | 2 | 1,32,0 | ReplaceLockShapeData | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,32,1 | ReplaceLockText | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,32,2 | ReplaceLockFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch | 2 | 1,32,3 | ReplaceCopyCells |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,0 | Char.Font | THEMEVAL() | 4 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,1 | Char.Color | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 3,0,2 | Char.Style | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,3 | Char.Case | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,4 | Char.Pos | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,5 | Char.FontScale | 100% | 1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 3,0,6 | Char._Unused6 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,7 | Char.Size | 12 pt | 0.16666666666666666 | 50 |
| page:0:Page-1/Switch.3 | 3 | 3,0,8 | Char.DblUnderline | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,9 | Char.Overline | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,10 | Char.Strikethru | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,11 | Char._Unused11 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,12 | Char._Unused12 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,13 | Char.DoubleStrikethrough | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,14 | Char._Unused14 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,15 | Char._Unused15 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,16 | Char.Letterspace | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 3,0,17 | Char.ColorTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 3,0,18 | Char.Q1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,19 | Char.R1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,20 | Char.S1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,21 | Char.T1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,22 | Char.U1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,23 | Char.V1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,24 | Char.W1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,25 | Char.Z1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,26 | Char.AA1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,27 | Char.AB1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,28 | Char.AC1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,29 | Char.AD1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,30 | Char.AE1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,31 | Char.AF1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,32 | Char.AG1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,33 | Char.AH1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,34 | Char.AI1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,35 | Char.AJ1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,36 | Char.AK1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,37 | Char.AL1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,38 | Char.AM1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,39 | Char.AN1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,40 | Char.AO1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,41 | Char.AP1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,42 | Char.AQ1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,43 | Char.AR1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,44 | Char.AS1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,45 | Char.AT1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,46 | Char.AU1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,47 | Char.AV1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,48 | Char.AW1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,49 | Char.AX1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,50 | Char.AY1 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,51 | Char.AsianFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,52 | Char.ComplexScriptFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,53 | Char._Unused53 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 3,0,54 | Char.ComplexScriptSize | -100% | -1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 3,0,55 | Char.BD1 |  | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 3,0,56 | Char.BE1 |  | 0 | 141 |
| page:0:Page-1/Switch.3 | 3 | 3,0,57 | Char.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,0 | Para.IndFirst | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 4,0,1 | Para.IndLeft | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 4,0,2 | Para.IndRight | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 4,0,3 | Para.SpLine | -120% | -1.2 | 33 |
| page:0:Page-1/Switch.3 | 3 | 4,0,4 | Para.SpBefore | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 4,0,5 | Para.SpAfter | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 4,0,6 | Para.HorzAlign | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,7 | Para.Bullet | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,8 | Para.BulletStr |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 4,0,9 | Para.BulletFont | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,10 | Para._Unused10 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,11 | Para.BulletFontSize | -100% | -1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 4,0,12 | Para.TextPosAfterBullet | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 4,0,13 | Para.Flags | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 4,0,14 | Para.M1 |  | 0 | 141 |
| page:0:Page-1/Switch.3 | 3 | 5,0,0 | Tabs.X1 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,2,0 | Geometry1.X2 | Width*1 | 1.5 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,3,0 | Geometry1.X3 | Width*1 | 1.5 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,3,1 | Geometry1.Y3 | Height*1 | 0.75 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,4,1 | Geometry1.Y4 | Height*1 | 0.75 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 248,0,0 | LineGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,0,1 | LineGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,0,2 | LineGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,1,0 | LineGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,1,1 | LineGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,1,2 | LineGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,2,0 | LineGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,2,1 | LineGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,2,2 | LineGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,3,0 | LineGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,3,1 | LineGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,3,2 | LineGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,4,0 | LineGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,4,1 | LineGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,4,2 | LineGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,5,0 | LineGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,5,1 | LineGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,5,2 | LineGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,6,0 | LineGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,6,1 | LineGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,6,2 | LineGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,7,0 | LineGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,7,1 | LineGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,7,2 | LineGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,8,0 | LineGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,8,1 | LineGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,8,2 | LineGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,9,0 | LineGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 248,9,1 | LineGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 248,9,2 | LineGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,0,0 | FillGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,0,1 | FillGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,0,2 | FillGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,1,0 | FillGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,1,1 | FillGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,1,2 | FillGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,2,0 | FillGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,2,1 | FillGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,2,2 | FillGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,3,0 | FillGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,3,1 | FillGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,3,2 | FillGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,4,0 | FillGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,4,1 | FillGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,4,2 | FillGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,5,0 | FillGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,5,1 | FillGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,5,2 | FillGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,6,0 | FillGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,6,1 | FillGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,6,2 | FillGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,7,0 | FillGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,7,1 | FillGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,7,2 | FillGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,8,0 | FillGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,8,1 | FillGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,8,2 | FillGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,9,0 | FillGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 249,9,1 | FillGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 249,9,2 | FillGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,1,0 | PinX | 5 in | 5 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,1 | PinY | 6 in | 6 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,2 | Width | 1.5 in | 1.5 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,3 | Height | 0.75 in | 0.75 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,4 | LocPinX | Width*0.5 | 0.75 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,5 | LocPinY | Height*0.5 | 0.375 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,1,9 | ResizeMode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,0 | LineWeight | THEMEVAL() | 0.010416666666666666 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,2,1 | LineColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,2,2 | LinePattern | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,3 | Rounding | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,2,4 | EndArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,5 | BeginArrow | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,6 | EndArrow | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,7 | LineCap | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,8 | BeginArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,2,9 | LineColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,2,10 | CompoundType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,3,0 | FillForegnd | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,3,1 | FillBkgnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,3,2 | FillPattern | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,3,3 | ShdwForegnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,3,4 | ShdwBkgnd | 1 | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,3,5 | ShdwPattern | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,3,6 | FillForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,3,7 | FillBkgndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,3,8 | ShdwForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,3,9 | ShdwBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,3,10 | ShapeShdwType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,3,11 | ShapeShdwOffsetX | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,3,12 | ShapeShdwOffsetY | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,3,13 | ShapeShdwObliqueAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,3,14 | ShapeShdwScaleFactor | THEMEVAL() | 1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,3,15 | ShapeShdwBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,3,16 | ShapeShdwShow | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,0 | TheData |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,1 | TheText |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,2 | EventDblClick |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,3 | EventXFMod |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,4 | EventDrop |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,5 | _EventPatMas |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,6 | EventTextOverflow |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,7 | Sect1.F6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,8 | Sect1.G6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,9 | Sect1.H6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,10 | Sect1.I6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,11 | Sect1.J6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,12 | Sect1.K6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,13 | Sect1.L6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,14 | Sect1.M6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,15 | Sect1.N6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,16 | Sect1.O6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,17 | Sect1.P6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,18 | Sect1.Q6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,19 | Sect1.R6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,20 | Sect1.S6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,21 | Sect1.T6 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,5,22 | EventMultiDrop |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,0 | EnableLineProps | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,1 | EnableFillProps | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,2 | EnableTextProps | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,3 | HideForApply | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,4 | Sect1.C9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,5 | Sect1.D9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,6 | Sect1.E9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,7 | Sect1.F9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,8 | Sect1.G9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,9 | Sect1.H9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,8,10 | Sect1.I9 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,0 | PageWidth |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,1 | PageHeight |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,2 | ShdwOffsetX |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,3 | ShdwOffsetY |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,4 | PageScale |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,5 | DrawingScale |  | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,10,6 | DrawingSizeType |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,7 | DrawingScaleType |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,8 | _Unused5F |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,9 | _Unused60 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,10 | Sect1.I11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,11 | Sect1.J11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,12 | _UpdateNotes |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,13 | Sect1.L11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,14 | Sect1.M11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,15 | Sect1.N11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,16 | Sect1.O11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,17 | Sect1.P11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,18 | Sect1.Q11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,19 | Sect1.R11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,20 | Sect1.S11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,21 | Sect1.T11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,22 | Sect1.U11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,23 | Sect1.V11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,24 | Sect1.W11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,25 | Sect1.Z11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,26 | InhibitSnap |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,27 | PageLockReplace |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,28 | PageLockDuplicate |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,29 | Sect1.AD11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,30 | Sect1.AE11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,31 | Sect1.AF11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,32 | Sect1.AG11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,33 | Sect1.AH11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,34 | UIVisibility |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,35 | ShdwType |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,36 | ShdwObliqueAngle |  | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,10,37 | ShdwScaleFactor |  | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,10,38 | DrawingResizeType |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,10,39 | Sect1.AN11 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,0 | LeftMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,11,1 | RightMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,11,2 | TopMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,11,3 | BottomMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,11,4 | VerticalAlign | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,5 | TextBkgnd | 0 | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,11,6 | DefaultTabStop | 0.5 in | 0.5 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,11,7 | TextMaxDepth | 4294967295 | 4294967295 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,8 | TextGeometry | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,9 | TextContainer | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,10 | TextDirection | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,11 | TextBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,11,12 | _UnusedE5 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,11,13 | _UnusedE6 |  | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,11,14 | _UnusedE7 |  | 0 | 141 |
| page:0:Page-1/Switch.3 | 3 | 1,15,0 | LockWidth | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,1 | LockHeight | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,2 | LockMoveX | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,3 | LockMoveY | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,4 | LockAspect | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,5 | LockDelete | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,6 | LockBegin | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,7 | LockEnd | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,8 | LockRotate | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,9 | LockCrop | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,10 | LockVtxEdit | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,11 | LockTextEdit | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,12 | LockFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,13 | LockGroup | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,14 | LockCalcWH | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,15 | LockSelect | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,16 | LockCustProp | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,17 | LockFromGroupFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,18 | LockThemeColors | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,19 | LockThemeEffects | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,20 | LockThemeConnectors | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,21 | LockThemeFonts | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,22 | LockThemeIndex | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,23 | LockReplace | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,15,24 | LockVariation | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,16,0 | HelpTopic |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,16,1 | Copyright |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,16,2 | Sect1.A17 |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,16,3 | Sect1.B17 |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,17,0 | NoObjHandles | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,1 | NonPrinting | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,2 | NoCtlHandles | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,3 | NoAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,4 | UpdateAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,5 | HideText | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,6 | _Unused83 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,7 | ShapeTabStop | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,8 | DynFeedback | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,9 | GlueType | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,11 | BegTrigger |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,12 | EndTrigger |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,14 | _Unused78 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,15 | _Unused79 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,16 | Comment |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,17,17 | IsDropSource | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,18 | NoLiveDynamics | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,19 | LocalizeMerge | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,20 | NoProofing | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,21 | Sect1.T18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,22 | Sect1.U18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,23 | Sect1.V18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,24 | Sect1.W18 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,25 | Calendar | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,26 | LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,27 | ShapeKeywords |  | 0 | 231 |
| page:0:Page-1/Switch.3 | 3 | 1,17,28 | DropOnPageScale | 100% | 1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,17,29 | Sect1.AD18 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,17,30 | Sect1.AE18 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,18,0 | XRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,18,1 | YRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,18,2 | XRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,3 | YRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,4 | XRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,5 | YRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,6 | XGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,18,7 | YGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,18,8 | XGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,9 | YGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,10 | XGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,18,11 | YGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,21,0 | Gamma | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,21,1 | Contrast | 50% | 0.5 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,2 | Brightness | 50% | 0.5 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,3 | Sharpen | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,4 | Blur | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,5 | Denoise | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,6 | Transparency | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,7 | _Unused8F | 0% | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,21,8 | _Unused90 | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,0 | SelectMode | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,1 | DisplayMode | 2 | 2 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,2 | IsDropTarget | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,3 | IsSnapTarget | TRUE | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,4 | IsTextEditTarget | TRUE | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,5 | DontMoveChildren | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,6 | Sect1.E23 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,7 | Sect1.F23 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,8 | Sect1.G23 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,22,9 | Sect1.H23 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,0 | PageLeftMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,25,1 | PageRightMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,25,2 | PageTopMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,25,3 | PageBottomMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Switch.3 | 3 | 1,25,4 | ScaleX | 100% | 1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,25,5 | ScaleY | 100% | 1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,25,6 | PagesX | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,7 | PagesY | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,8 | CenterX | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,9 | CenterY | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,10 | OnPage | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,11 | PrintGrid | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,12 | Sect1.K26 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,13 | Sect1.L26 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,14 | Sect1.M26 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,15 | Sect1.N26 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,16 | PrintPageOrientation | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,17 | PaperKind | 1 | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,25,18 | PaperSource | 7 | 7 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,0 | LineGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,1 | LineGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,26,2 | FillGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,3 | FillGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,26,4 | LineGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,5 | FillGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,6 | RotateGradientWithShape | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,7 | UseGroupGradient | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,8 | Sect1.G27 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,9 | Sect1.H27 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,10 | Sect1.I27 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,26,11 | Sect1.J27 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,0 | QuickStyleLineColor | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,1 | QuickStyleFillColor | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,2 | QuickStyleShadowColor | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,3 | QuickStyleFontColor | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,4 | QuickStyleLineMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,5 | QuickStyleFillMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,6 | QuickStyleEffectsMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,7 | QuickStyleFontMatrix | 100 | 100 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,8 | QuickStyleType | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,27,9 | QuickStyleVariation | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,0 | ReflectionTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,28,1 | ReflectionSize | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,28,2 | ReflectionDist | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,28,3 | ReflectionBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,28,4 | GlowColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,28,5 | GlowColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,28,6 | GlowSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,28,7 | SoftEdgesSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,28,8 | SketchSeed | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,9 | SketchEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,10 | SketchAmount | THEMEVAL() | 5 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,11 | SketchLineWeight | THEMEVAL() | 0.041666666666666664 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,28,12 | SketchLineChange | THEMEVAL() | 0.14 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,28,13 | SketchFillChange | THEMEVAL() | 0.1 | 33 |
| page:0:Page-1/Switch.3 | 3 | 1,28,14 | Sect1.M29 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,15 | Sect1.N29 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,28,16 | Sect1.O29 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,29,0 | BevelTopType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,29,1 | BevelTopWidth | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,2 | BevelTopHeight | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,3 | BevelBottomType | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,29,4 | BevelBottomWidth | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,5 | BevelBottomHeight | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,6 | BevelDepthColor | 1 | 1 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,29,7 | BevelDepthSize | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,8 | BevelContourColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Switch.3 | 3 | 1,29,9 | BevelContourSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,29,10 | BevelMaterialType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,29,11 | BevelLightingType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,29,12 | BevelLightingAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,30,3 | RotationType | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| page:0:Page-1/Switch.3 | 3 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| page:0:Page-1/Switch.3 | 3 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,32,0 | ReplaceLockShapeData | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,32,1 | ReplaceLockText | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,32,2 | ReplaceLockFormat | 0 | 0 | 32 |
| page:0:Page-1/Switch.3 | 3 | 1,32,3 | ReplaceCopyCells |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,0 | PinX | 2.5 in | 2.5 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,1 | PinY | 8.3 in | 8.3 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,2 | Width | 3 in | 3 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,3 | Height | 0.6 in | 0.59999999999999964 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,4 | LocPinX | Width*0.5 | 1.5 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,5 | LocPinY | Height*0.5 | 0.29999999999999982 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,6 | Angle | 0 deg | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,7 | FlipX | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,8 | FlipY | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,1,9 | ResizeMode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,0 | LineWeight | THEMEVAL() | 0.010416666666666666 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,1 | LineColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,2 | LinePattern | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,3 | Rounding | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,4 | EndArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,5 | BeginArrow | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,6 | EndArrow | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,7 | LineCap | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,8 | BeginArrowSize | 2 | 2 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,9 | LineColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,2,10 | CompoundType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,0 | Char.Font | THEMEVAL() | 4 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,1 | Char.Color | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,2 | Char.Style | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,3 | Char.Case | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,4 | Char.Pos | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,5 | Char.FontScale | 100% | 1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,6 | Char._Unused6 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,7 | Char.Size | 12 pt | 0.16666666666666666 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,8 | Char.DblUnderline | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,9 | Char.Overline | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,10 | Char.Strikethru | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,11 | Char._Unused11 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,12 | Char._Unused12 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,13 | Char.DoubleStrikethrough | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,14 | Char._Unused14 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,15 | Char._Unused15 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,16 | Char.Letterspace | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,17 | Char.ColorTrans | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,18 | Char.Q1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,19 | Char.R1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,20 | Char.S1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,21 | Char.T1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,22 | Char.U1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,23 | Char.V1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,24 | Char.W1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,25 | Char.Z1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,26 | Char.AA1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,27 | Char.AB1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,28 | Char.AC1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,29 | Char.AD1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,30 | Char.AE1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,31 | Char.AF1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,32 | Char.AG1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,33 | Char.AH1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,34 | Char.AI1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,35 | Char.AJ1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,36 | Char.AK1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,37 | Char.AL1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,38 | Char.AM1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,39 | Char.AN1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,40 | Char.AO1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,41 | Char.AP1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,42 | Char.AQ1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,43 | Char.AR1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,44 | Char.AS1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,45 | Char.AT1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,46 | Char.AU1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,47 | Char.AV1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,48 | Char.AW1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,49 | Char.AX1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,50 | Char.AY1 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,51 | Char.AsianFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,52 | Char.ComplexScriptFont | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,53 | Char._Unused53 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,54 | Char.ComplexScriptSize | -100% | -1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,55 | Char.BD1 |  | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,56 | Char.BE1 |  | 0 | 141 |
| page:0:Page-1/Sheet.4 | 4 | 3,0,57 | Char.LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,0 | Para.IndFirst | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,1 | Para.IndLeft | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,2 | Para.IndRight | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,3 | Para.SpLine | -120% | -1.2 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,4 | Para.SpBefore | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,5 | Para.SpAfter | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,6 | Para.HorzAlign | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,7 | Para.Bullet | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,8 | Para.BulletStr |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,9 | Para.BulletFont | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,10 | Para._Unused10 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,11 | Para.BulletFontSize | -100% | -1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,12 | Para.TextPosAfterBullet | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,13 | Para.Flags | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 4,0,14 | Para.M1 |  | 0 | 141 |
| page:0:Page-1/Sheet.4 | 4 | 5,0,0 | Tabs.X1 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,0 | Geometry1.NoFill | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,1 | Geometry1.NoLine | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,2 | Geometry1.NoShow | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,3 | Geometry1.NoSnap | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,4 | Geometry1.Path |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,0,5 | Geometry1.NoQuickDrag | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 10,1,0 | Geometry1.X1 | Width*0 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,1,1 | Geometry1.Y1 | Height*0 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,2,0 | Geometry1.X2 | Width*1 | 3 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,2,1 | Geometry1.Y2 | Height*0 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,3,0 | Geometry1.X3 | Width*1 | 3 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,3,1 | Geometry1.Y3 | Height*1 | 0.59999999999999964 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,4,0 | Geometry1.X4 | Width*0 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,4,1 | Geometry1.Y4 | Height*1 | 0.59999999999999964 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,5,0 | Geometry1.X5 | Geometry1.X1 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 10,5,1 | Geometry1.Y5 | Geometry1.Y1 | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 248,0,0 | LineGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,0,1 | LineGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,0,2 | LineGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,1,0 | LineGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,1,1 | LineGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,1,2 | LineGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,2,0 | LineGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,2,1 | LineGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,2,2 | LineGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,3,0 | LineGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,3,1 | LineGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,3,2 | LineGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,4,0 | LineGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,4,1 | LineGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,4,2 | LineGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,5,0 | LineGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,5,1 | LineGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,5,2 | LineGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,6,0 | LineGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,6,1 | LineGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,6,2 | LineGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,7,0 | LineGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,7,1 | LineGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,7,2 | LineGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,8,0 | LineGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,8,1 | LineGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,8,2 | LineGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,9,0 | LineGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 248,9,1 | LineGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 248,9,2 | LineGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,0,0 | FillGradientStops.GradientStopColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,0,1 | FillGradientStops.GradientStopColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,0,2 | FillGradientStops.GradientStopPosition | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,1,0 | FillGradientStops.GradientStopColor[2] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,1,1 | FillGradientStops.GradientStopColorTrans[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,1,2 | FillGradientStops.GradientStopPosition[2] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,2,0 | FillGradientStops.GradientStopColor[3] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,2,1 | FillGradientStops.GradientStopColorTrans[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,2,2 | FillGradientStops.GradientStopPosition[3] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,3,0 | FillGradientStops.GradientStopColor[4] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,3,1 | FillGradientStops.GradientStopColorTrans[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,3,2 | FillGradientStops.GradientStopPosition[4] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,4,0 | FillGradientStops.GradientStopColor[5] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,4,1 | FillGradientStops.GradientStopColorTrans[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,4,2 | FillGradientStops.GradientStopPosition[5] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,5,0 | FillGradientStops.GradientStopColor[6] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,5,1 | FillGradientStops.GradientStopColorTrans[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,5,2 | FillGradientStops.GradientStopPosition[6] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,6,0 | FillGradientStops.GradientStopColor[7] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,6,1 | FillGradientStops.GradientStopColorTrans[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,6,2 | FillGradientStops.GradientStopPosition[7] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,7,0 | FillGradientStops.GradientStopColor[8] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,7,1 | FillGradientStops.GradientStopColorTrans[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,7,2 | FillGradientStops.GradientStopPosition[8] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,8,0 | FillGradientStops.GradientStopColor[9] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,8,1 | FillGradientStops.GradientStopColorTrans[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,8,2 | FillGradientStops.GradientStopPosition[9] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,9,0 | FillGradientStops.GradientStopColor[10] | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 249,9,1 | FillGradientStops.GradientStopColorTrans[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 249,9,2 | FillGradientStops.GradientStopPosition[10] | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,0 | FillForegnd | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,1 | FillBkgnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,2 | FillPattern | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,3 | ShdwForegnd | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,4 | ShdwBkgnd | 1 | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,5 | ShdwPattern | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,6 | FillForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,7 | FillBkgndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,8 | ShdwForegndTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,9 | ShdwBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,10 | ShapeShdwType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,11 | ShapeShdwOffsetX | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,12 | ShapeShdwOffsetY | THEMEVAL() | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,13 | ShapeShdwObliqueAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,14 | ShapeShdwScaleFactor | THEMEVAL() | 1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,15 | ShapeShdwBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,3,16 | ShapeShdwShow | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,0 | TheData |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,1 | TheText |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,2 | EventDblClick |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,3 | EventXFMod |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,4 | EventDrop |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,5 | _EventPatMas |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,6 | EventTextOverflow |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,7 | Sect1.F6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,8 | Sect1.G6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,9 | Sect1.H6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,10 | Sect1.I6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,11 | Sect1.J6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,12 | Sect1.K6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,13 | Sect1.L6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,14 | Sect1.M6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,15 | Sect1.N6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,16 | Sect1.O6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,17 | Sect1.P6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,18 | Sect1.Q6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,19 | Sect1.R6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,20 | Sect1.S6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,21 | Sect1.T6 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,5,22 | EventMultiDrop |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,0 | EnableLineProps | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,1 | EnableFillProps | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,2 | EnableTextProps | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,3 | HideForApply | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,4 | Sect1.C9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,5 | Sect1.D9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,6 | Sect1.E9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,7 | Sect1.F9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,8 | Sect1.G9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,9 | Sect1.H9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,8,10 | Sect1.I9 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,0 | PageWidth |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,1 | PageHeight |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,2 | ShdwOffsetX |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,3 | ShdwOffsetY |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,4 | PageScale |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,5 | DrawingScale |  | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,6 | DrawingSizeType |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,7 | DrawingScaleType |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,8 | _Unused5F |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,9 | _Unused60 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,10 | Sect1.I11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,11 | Sect1.J11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,12 | _UpdateNotes |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,13 | Sect1.L11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,14 | Sect1.M11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,15 | Sect1.N11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,16 | Sect1.O11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,17 | Sect1.P11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,18 | Sect1.Q11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,19 | Sect1.R11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,20 | Sect1.S11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,21 | Sect1.T11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,22 | Sect1.U11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,23 | Sect1.V11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,24 | Sect1.W11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,25 | Sect1.Z11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,26 | InhibitSnap |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,27 | PageLockReplace |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,28 | PageLockDuplicate |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,29 | Sect1.AD11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,30 | Sect1.AE11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,31 | Sect1.AF11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,32 | Sect1.AG11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,33 | Sect1.AH11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,34 | UIVisibility |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,35 | ShdwType |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,36 | ShdwObliqueAngle |  | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,37 | ShdwScaleFactor |  | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,38 | DrawingResizeType |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,10,39 | Sect1.AN11 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,0 | LeftMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,1 | RightMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,2 | TopMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,3 | BottomMargin | 4 pt | 0.055555555555555552 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,4 | VerticalAlign | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,5 | TextBkgnd | 0 | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,6 | DefaultTabStop | 0.5 in | 0.5 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,7 | TextMaxDepth | 4294967295 | 4294967295 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,8 | TextGeometry | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,9 | TextContainer | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,10 | TextDirection | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,11 | TextBkgndTrans | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,12 | _UnusedE5 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,13 | _UnusedE6 |  | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,11,14 | _UnusedE7 |  | 0 | 141 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,0 | LockWidth | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,1 | LockHeight | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,2 | LockMoveX | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,3 | LockMoveY | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,4 | LockAspect | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,5 | LockDelete | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,6 | LockBegin | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,7 | LockEnd | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,8 | LockRotate | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,9 | LockCrop | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,10 | LockVtxEdit | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,11 | LockTextEdit | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,12 | LockFormat | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,13 | LockGroup | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,14 | LockCalcWH | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,15 | LockSelect | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,16 | LockCustProp | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,17 | LockFromGroupFormat | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,18 | LockThemeColors | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,19 | LockThemeEffects | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,20 | LockThemeConnectors | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,21 | LockThemeFonts | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,22 | LockThemeIndex | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,23 | LockReplace | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,15,24 | LockVariation | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,16,0 | HelpTopic |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,16,1 | Copyright |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,16,2 | Sect1.A17 |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,16,3 | Sect1.B17 |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,0 | NoObjHandles | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,1 | NonPrinting | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,2 | NoCtlHandles | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,3 | NoAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,4 | UpdateAlignBox | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,5 | HideText | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,6 | _Unused83 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,7 | ShapeTabStop | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,8 | DynFeedback | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,9 | GlueType | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,10 | WalkPreference | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,11 | BegTrigger |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,12 | EndTrigger |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,13 | ObjType | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,14 | _Unused78 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,15 | _Unused79 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,16 | Comment |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,17 | IsDropSource | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,18 | NoLiveDynamics | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,19 | LocalizeMerge | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,20 | NoProofing | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,21 | Sect1.T18 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,22 | Sect1.U18 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,23 | Sect1.V18 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,24 | Sect1.W18 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,25 | Calendar | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,26 | LangID | 1033 | 1033 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,27 | ShapeKeywords |  | 0 | 231 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,28 | DropOnPageScale | 100% | 1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,29 | Sect1.AD18 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,17,30 | Sect1.AE18 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,0 | XRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,1 | YRulerDensity | 32 | 32 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,2 | XRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,3 | YRulerSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,4 | XRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,5 | YRulerOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,6 | XGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,7 | YGridDensity | 8 | 8 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,8 | XGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,9 | YGridSpacing | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,10 | XGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,18,11 | YGridOrigin | 0 in | 0 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,0 | Gamma | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,1 | Contrast | 50% | 0.5 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,2 | Brightness | 50% | 0.5 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,3 | Sharpen | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,4 | Blur | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,5 | Denoise | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,6 | Transparency | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,7 | _Unused8F | 0% | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,21,8 | _Unused90 | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,0 | SelectMode | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,1 | DisplayMode | 2 | 2 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,2 | IsDropTarget | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,3 | IsSnapTarget | TRUE | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,4 | IsTextEditTarget | TRUE | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,5 | DontMoveChildren | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,6 | Sect1.E23 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,7 | Sect1.F23 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,8 | Sect1.G23 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,22,9 | Sect1.H23 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,0 | ShapePermeableX | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,1 | ShapePermeableY | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,2 | ShapePermeablePlace | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,3 | Relationships | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,4 | Sect1.C24 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,5 | Sect1.D24 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,6 | Sect1.E24 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,7 | Sect1.F24 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,8 | ShapeFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,9 | ShapePlowCode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,10 | ShapeRouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,11 | ShapePlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,12 | ConFixedCode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,13 | ConLineJumpCode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,14 | ConLineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,15 | ShapePlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,16 | ConLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,17 | ConLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,18 | ShapePlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,19 | ConLineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,20 | ShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,21 | ShapeSplittable | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,22 | DisplayLevel | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,23 | Sect1.V24 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,23,24 | Sect1.W24 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,0 | ResizePage | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,1 | EnableGrid | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,2 | DynamicsOff | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,3 | CtrlAsInput | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,4 | AvoidPageBreaks | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,5 | Sect1.D25 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,6 | Sect1.E25 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,7 | Sect1.F25 | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,8 | PlaceStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,9 | RouteStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,10 | PlaceDepth | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,11 | PlowCode | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,12 | LineJumpCode | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,13 | LineJumpStyle | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,14 | PageLineJumpDirX | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,15 | PageLineJumpDirY | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,16 | LineToNodeX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,17 | LineToNodeY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,18 | BlockSizeX | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,19 | BlockSizeY | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,20 | AvenueSizeX | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,21 | AvenueSizeY | 0.375 in | 0.375 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,22 | LineToLineX | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,23 | LineToLineY | 0.125 in | 0.125 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,24 | LineJumpFactorX | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,25 | LineJumpFactorY | 0.66666666666667 | 0.66666666666667 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,26 | LineAdjustFrom | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,27 | LineAdjustTo | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,28 | PlaceFlip | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,29 | LineRouteExt | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,24,30 | PageShapeSplit | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,0 | PageLeftMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,1 | PageRightMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,2 | PageTopMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,3 | PageBottomMargin | 0.25 in | 0.25 | 73 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,4 | ScaleX | 100% | 1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,5 | ScaleY | 100% | 1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,6 | PagesX | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,7 | PagesY | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,8 | CenterX | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,9 | CenterY | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,10 | OnPage | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,11 | PrintGrid | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,12 | Sect1.K26 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,13 | Sect1.L26 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,14 | Sect1.M26 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,15 | Sect1.N26 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,16 | PrintPageOrientation | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,17 | PaperKind | 1 | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,25,18 | PaperSource | 7 | 7 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,0 | LineGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,1 | LineGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,2 | FillGradientDir | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,3 | FillGradientAngle | THEMEVAL() | 1.5707963267949 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,4 | LineGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,5 | FillGradientEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,6 | RotateGradientWithShape | THEMEVAL() | 1 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,7 | UseGroupGradient | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,8 | Sect1.G27 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,9 | Sect1.H27 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,10 | Sect1.I27 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,26,11 | Sect1.J27 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,0 | QuickStyleLineColor | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,1 | QuickStyleFillColor | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,2 | QuickStyleShadowColor | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,3 | QuickStyleFontColor | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,4 | QuickStyleLineMatrix | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,5 | QuickStyleFillMatrix | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,6 | QuickStyleEffectsMatrix | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,7 | QuickStyleFontMatrix | 100 | 100 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,8 | QuickStyleType | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,27,9 | QuickStyleVariation | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,0 | ReflectionTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,1 | ReflectionSize | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,2 | ReflectionDist | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,3 | ReflectionBlur | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,4 | GlowColor | THEMEVAL() | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,5 | GlowColorTrans | THEMEVAL() | 0 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,6 | GlowSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,7 | SoftEdgesSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,8 | SketchSeed | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,9 | SketchEnabled | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,10 | SketchAmount | THEMEVAL() | 5 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,11 | SketchLineWeight | THEMEVAL() | 0.041666666666666664 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,12 | SketchLineChange | THEMEVAL() | 0.14 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,13 | SketchFillChange | THEMEVAL() | 0.1 | 33 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,14 | Sect1.M29 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,15 | Sect1.N29 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,28,16 | Sect1.O29 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,0 | BevelTopType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,1 | BevelTopWidth | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,2 | BevelTopHeight | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,3 | BevelBottomType | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,4 | BevelBottomWidth | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,5 | BevelBottomHeight | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,6 | BevelDepthColor | 1 | 1 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,7 | BevelDepthSize | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,8 | BevelContourColor | THEMEVAL() | 0 | 251 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,9 | BevelContourSize | THEMEVAL() | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,10 | BevelMaterialType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,11 | BevelLightingType | THEMEVAL() | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,29,12 | BevelLightingAngle | THEMEVAL() | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,0 | RotationXAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,1 | RotationYAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,2 | RotationZAngle | 0 deg | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,3 | RotationType | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,4 | Perspective | 0 deg | 0 | 81 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,5 | DistanceFromGround | 0 pt | 0 | 50 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,6 | KeepTextFlat | FALSE | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,7 | Sect1.F31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,8 | Sect1.G31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,9 | Sect1.H31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,10 | Sect1.I31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,11 | Sect1.J31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,12 | Sect1.K31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,30,13 | Sect1.L31 |  | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,0 | ColorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,1 | EffectSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,2 | ConnectorSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,3 | FontSchemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,4 | ThemeIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,5 | VariationColorIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,6 | VariationStyleIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,31,7 | EmbellishmentIndex | 65534 | 65534 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,32,0 | ReplaceLockShapeData | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,32,1 | ReplaceLockText | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,32,2 | ReplaceLockFormat | 0 | 0 | 32 |
| page:0:Page-1/Sheet.4 | 4 | 1,32,3 | ReplaceCopyCells |  | 0 | 32 |

Master attributes: `{"AlignName": "2", "BaseID": "{052EB6F2-ABA5-46DF-B488-2E31CD4C1717}", "Hidden": "0", "ID": "2", "IconSize": "1", "IconUpdate": "1", "IsCustomName": "1", "IsCustomNameU": "1", "MasterType": "2", "MatchByName": "0", "Name": "Switch", "NameU": "Switch", "PatternFlags": "0", "Prompt": "", "UniqueID": "{0211518D-0002-0000-8E40-00608CF305B2}"}`.
Icon: present; MasterShortcut: absent.

Text visio/pages/page1.xml shape 4: markers `[]`; content `'Single run\n'`.
Text visio/masters/master1.xml shape 5: markers `[["pp", {"IX": "0"}]]`; content `'Switch\nOriginal\n'`.

M4.20: the original Property COM dump exposes five reserved columns9..13,
named Prop.NameH7 through Prop.NameL7 in row0, then H8..L8 in row1 and so on.
They return0 with unit32 and have no serialized cells. The dump adapter exposes
these live slots, just as it already does for Connection and Actions slots.
No oracle row or comparison denominator was removed. Full Property local-cell
comparison checks the eleven XML cells separately, including LangID.

M4.21: Masters.Drop reserves master ID1 and creates the first master as2.
The original rectangle master page is1.510416666666667 by0.7604166666666661in:
shape dimensions1.5 by0.75 plus its0.0104166666666667in stroke. Its pin is half
that complete extent. DrawingResizeType=1, no inherited Connector layer, empty
Prompt and MatchByName=0 distinguish it from the Dynamic connector template.
The engine retains fresh identities and omits Icon for Visio to generate.
The original's default centered alignment produces a pp marker but no redundant
Paragraph section. The admissible SAN re-save gives the character-format case:
cp then pp, and a literal Size without its former constant formula. Imported
text and cells remain preserved by the writer's existing source-state contract.

M4.22: the SAN recipe had explicitly named Liberation Sans, a renderer fallback,
creating a FaceNames entry that Visio coalesced on save. The recipe now inherits
the blank document's face identity and only sets size/style/alignment. Rendering
resolves the document name to an open font at draw time. Explicit user font
choices still add their named document face. Regression checks both paths and
asserts that text authoring, rendering and saving add no fallback face.

M4.23 retained PNG row analysis: SAN labels begin at our row1 and Visio row12;
the bottom switch edge is our746 and Visio755/756. Aligning our image ten rows
down reduces MAD from7.086011 to1.288038. The missing extent is at the top,
not extra bottom canvas padding. The text-only labels were bounded using a
0.75em typographic ascent. Visio's observed export uses the full line advance
around the baseline:12pt label advance14.4pt,16pt fabric label advance19.2pt.
Using the existing layout's line advance gives master/from-shape433x272 and
SAN1612x757, exactly the observed sizes. This is a logical text extent rule,
not a per-file pixel offset. The retained SAN pair with the existing canvas
mapping measures MAD0.897567, over16 1.282161%, within frozen non-text ceilings.
The new data Pair separately exposes the existing small-image mapping gap.

### M4.21 closure re-save disagreement: redundant paragraph markers

The original `generated/master/from-shape.vsdx` master shape5 has `pp IX=0`
with no local Paragraph section. The matching rebuild has the same marker and
local cell set. Visio opens the rebuild and removes that marker on save:
`p4bc__from-shape/saved.vsdx`, master1 shape5. It likewise removes pp from the
SAN Switch master and fabric labels when their Paragraph format is inherited,
leaving the local cp marker. The earlier SAN save retained pp while its local
Paragraph rows were present. Thus pp is not an unconditional consequence of
requesting alignment equal to an inherited default.

The original takes precedence under Charles's ruling. This closure retains its
marker form and reports the re-save differences instead of silently changing
the original comparison or adding a repair allowance. The three SAN Text
residuals and one master Text residual remain. The master rebuild additionally
has one opaque docProps/app.xml byte residual: Visio expands empty elements
such as Template and Manager. None is a COM value difference. These are named
serialization gaps, with exact input, saved package and diff retained privately.

Fresh closure acceptance: all three files open, COM residual0. Property has0
residual repairs; SAN3 and master rebuild2. SAN re-save preserves40 connectors
and1344 Geometry/XForm cells within1e-6, with80 Connects. Fresh SAN PNG sizes
are1612x757 on both sides, MAD0.897574663, over16 1.282242%; required non-text
ceilings pass. Property remains529x145, MAD5.592103948, over16 3.929340%, failing
the unchanged ceiling. Master page433x272 passes at1.297346941/1.932482%.


### M4.26 and M4.27: authored canonical form (2026-10-15)

Observed, undocumented: the master/from-shape original contains a pp IX=0
marker with inherited Paragraph formatting. Visio removes the same redundant
marker from our from-shape master and the SAN master and fabric labels on save.
Charles's dated ruling adopts removal for newly authored content, while the
original remains authoritative for cells and rows. Imported Text trees remain
unchanged. Local paragraph formatting still writes its run marker.
Reference: https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
Witnesses: private corpus/_vm/out/accept/p4bc__from-shape/saved.vsdx and
p4bc__san-topology/saved.vsdx, retained in the P4b-closure private checkpoint.

The from-shape save spells empty Template, Manager, Company and HyperlinkBase
in docProps/app.xml with explicit closing tags. Authored metadata now uses that
spelling, keeping the observed CRLF declaration and vector attribute order.
Read metadata remains byte-preserved under M3.12. This is an authoring fix, not
an allowed repair class.
Reference: https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx

Closure-2 fresh observations: SAN and from-shape both open with zero residual
repairs and zero COM differences. The unchanged Property observation remains
zero/zero. SAN keeps all 40 routes across 1,344 Geometry/XForm cells, with 80
Connects rows. Both renders are 1612x757, MAD 0.897574663 and over16 1.282242494%,
inside the frozen required ceilings. From-shape app.xml is byte-equal to its
fresh Visio save. No new repair class or per-file extent adjustment is used.


## Observed, undocumented: Brief A witnesses (2026-10-15)

The initial authorized make-corpus VM sweep ran through the interactive scheduled-task
runner on Visio 16.0 build 20430 (file version 16.0.20430.20146). The executed
addition is retained in `plan/brief-a/make-corpus.ps1`, with the immutable run
log under `corpus/_vm/history/brief-a-first-run/probe-log.txt`. The private corpus retains
the script, shared dump helper, log and SHA256 receipt under
`_state/brief-a-partial/`. There are seven complete drawings (42 artifacts),
34,617 dumped cells, zero cell-read errors and two drawing failures. Runner exit
0 means script completion; it does not make those two failed drawings successful.
No existing witness, threshold, population or manifest pin was changed.

### W1: move and PinX assignment

Files: `generated/review-a/w1-before.vsdx` and
`generated/review-a/w1-after-move.vsdx`, with `.sheet.tsv` and `.connects.tsv`.
The Dynamic connector was statically glued at each end to a rectangle connection
point. Before the move, BeginX was
`PAR(PNT(Sheet.1!Connections.X1,Sheet.1!Connections.Y1))`.
COM refused `Shape.Move` because that method is not exposed. The documented
single-shape `Selection.Move(0,1)` succeeded: `Shape.Connects.Count=0`, BeginX
FormulaU `1.75 in`, and BeginY changed from 6.75 to 7.75 internal inches. Both
endpoint formulas became literals and the saved Connects table is empty.

After the script restored both GlueTo attachments, assigning PinX FormulaU `4`
raised `Cell is guarded.` No after-PinX drawing or post-refusal count was captured;
these are unmeasured. This is not evidence that an accepted PinX edit drops glue.
The movement result supports detaching both endpoints and removing Connect rows
for shape.move and equivalent arrangement operations.
References: [Selection.Move](https://learn.microsoft.com/en-us/office/vba/api/visio.selection.move),
[Cell.FormulaU](https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau).

### W2: master from a group member

File: `generated/review-a/w2-member-master.vsdx` and its sheet dump.
The right member had Width `Sheet.3!Width*0.5` and Height `Sheet.3!Height*1`.
`Document.Masters.Drop(member,0,0)` succeeded. The new master's shape has
Width FormulaU `2 in` and Height FormulaU `1 in`. External group references were
materialized as literal dimensions.
Reference: [Masters.Drop](https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop).

### W3: jumps exclude the plain glued line

The authorized W3-only recovery selected only the Dynamic connector and called
`Selection.Layout`, without Shape.Reroute. Files:
`generated/review-a/w3-horizontal-jumps.vsdx` (LineJumpCode 1) and
`generated/review-a/w3-plain-line-crossing.vsdx` (LineJumpCode 2).
Both were saved, exported and dumped by the existing path: 13,824 cells,
zero cell-read errors, zero failed drawings, runner exit 0. The exact recipe
and log are preserved privately under `_state/brief-a-w3/`.

In both drawings, shape 3 is the static-glued plain line (ObjType 0):
Geometry1 row 1 MoveTo `(0,0)`, row 2 LineTo `(5,0)`, with X formulas
`Width*0` and `Width*1`. It receives no jump. Shape 6 is the crossing Dynamic
connector: row 1 MoveTo `(0.125,0)`, row 2 LineTo `(0.125,6.5)`, deleted
LineTo row 3. It also receives no jump across the plain line. Begin/End are
`(4,1.5)` and `(4,8)`, crossing the plain line at `(4,4.5)`.
Thus the plain line neither receives a jump nor participates as a connector
crossing candidate. This agrees with the connector scope of the references.
References: [Selection.Layout](https://learn.microsoft.com/en-us/office/vba/api/visio.selection.layout),
[LineJumpCode](https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpcode-cell-page-layout-section),
[ConLineJumpCode](https://learn.microsoft.com/en-us/office/client-developer/visio/conlinejumpcode-cell-shape-layout-section).

### W4: ConFixedCode 2 after moving the target

Files: `generated/review-a/w4-never-reroute-before.vsdx` and
`generated/review-a/w4-never-reroute-after.vsdx`, with full sheet dumps.
The right target moved by (1,0). ConFixedCode FormulaU and ResultIU remain `2`.
Geometry1 has MoveTo row 1 `(0,0.125)` and LineTo row 2; row 2 changes from
`(2.5,0.125)` to `(3.5,0.125)`. Deleted LineTo row 3 remains deleted in XML.
The coordinate formulas are literals with `in` units. NoFill remains TRUE;
NoLine, NoShow, NoSnap and NoQuickDrag remain FALSE. Width changes 2.5 to 3.5;
BeginX remains 2.5 and EndX changes 5 to 6. Thus never-reroute does not mean
freezing the last geometry endpoint while its glued target moves.
Reference: [ConFixedCode](https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section).

### W5: all flip combinations keep centred text readable

Files: `generated/review-a/w5-text-flips.vsdx`, `.png`, `.svg` and `.sheet.tsv`.
Shapes 1 through 4 are unflipped, FlipX, FlipY and both, respectively. The PNG
was visually inspected: every label reads `Readable` upright and left-to-right.
SVG shape 2 has scale(-1,1) and its text applies scale(-1,1); shape 3 and its
text similarly apply scale(1,-1). Shape 4 and its text both apply rotate(180).
The text counter-transform cancels every observed shape flip.
References: [FlipX](https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section),
[FlipY](https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section).

### W6: bare FormulaU numbers, record only

File: `generated/review-a/w6-default-units.vsdx` and its sheet dump.
The document was created with Documents.AddEx('',visMSMetric=1,0), and its
DrawingScale is `1 mm`. Setting Angle FormulaU to bare `30` yields ResultIU
`4.8672587712816`, FormulaU `278.87338539247 deg`, Units 81. This is 30 radians
normalized modulo 2*pi. Setting Width FormulaU to bare `20` yields ResultIU
`20`, FormulaU `508 mm`, Units 70. These observations use internal radians
and inches rather than the displayed cell defaults of degrees and millimetres.
These results match Diagramcraft. No unit behavior change or new issue is required.
References: [Documents.AddEx](https://learn.microsoft.com/en-us/office/vba/api/visio.documents.addex),
[Cell.FormulaU](https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau).

The follow-up rulings use W1 to detach moved connector ends and reset triggers,
W2 to literalize references outside a copied master, and W4 to preserve code 2
without rerouting while updating glued ends. W5 requires readable text for all
three flip combinations. The two W3 recovery originals add 12 artifacts to the
seven first-run originals. They are observations, not additions to the frozen
regression cohort. Private paths above remain `generated/review-a`; the local
measurement copy is `corpus/observations/review-a` during frozen gate runs.
