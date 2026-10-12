# File format parity

Status: pre-alpha, M1 met. Phase 2 is partial and fails its render and text-promotion gates.

## Measured

P2-followup-2 partial worktree: all 204 generated drawings render without command
failures. Size mismatches fall from 35 to 10; fill and line pass their frozen group
ceilings. MAD below is in RGB units from 0 to 255. Mismatched files have no pixel
score and always fail. No oracle image is resized for these comparisons.

| Group | Files | Size mismatches | Mean RGB MAD | Pixels over 16 | Gate |
|---|---:|---:|---:|---:|---|
| arrow | 92 | 0 | 2.928643 | 2.217007% | FAIL |
| basic | 1 | 0 | 2.481191 | 2.008629% | FAIL |
| connector | 6 | 0 | 4.906090 | 2.086032% | FAIL |
| fill | 44 | 0 | 1.881272 | 1.883832% | PASS |
| group | 1 | 0 | 2.874225 | 2.541990% | FAIL |
| image | 3 | 0 | 3.494772 | 3.956284% | FAIL |
| line | 37 | 0 | 2.233283 | 1.518873% | PASS |
| master | 1 | 0 | 3.472529 | 2.609142% | FAIL |
| page | 3 | 1 | 2.551506 | 1.534294% | FAIL |
| text | 16 | 9 | 3.362636 | 2.989645% | FAIL |

The resvg and pypdf tests pass all 204 outputs, with extractable text in all 16 text
drawings. This does not prove complete character preservation: ligature extraction
and field reading order remain gaps. All 24 paired contact sheets were regenerated.
They cover 1108 masters, with one local failure: BASFLO_U Custom1, master 11,
shape 4, has a direct Height/Scratch.Y1 cycle. All 1108 Visio reference exports exist.
These are private observations, not distributable assets. Performance is 24552 ms
for the largest stencil (43 masters, zero errors) and 436 ms for 500 shapes.
No render performance budget is frozen. No new Rust commits landed because the
mandatory render gate still fails CI. The human reviewed the previous sheets and
kept all ceilings unchanged; M2 remains unaccepted.


P2-followup-2 full coverage: 1387 packages, all 1181 shipped packages, 162 completed
shipped dumps and 375 TSVs. Oracle 1 is 4627375/4716091 (98.118866%), above its
97.5951% floor. Oracle 2 is 14252130/14363222 (99.226552%), with non-literal COM
1570871/1586871 (98.991726%). Text cached rises from 34267/67797 (50.543534%) to
54294/67797 (80.083190%); text COM rises from 3583/7724 (46.387882%) to 6192/7724
(80.165717%). Both remain below 90%, so text is deferred. Theme COM is
1053468/1059698 (99.412097%); cached theme is 281901/307601 (91.645021%).
Shared inputs are read without rewriting files.

Remaining cached text failures include 4078 TxtWidth, 2978 TxtHeight, 1879
User.Width and 890 Height cells. Examples: ADO_M master 2 shape 45 TxtWidth is
0.3479410807291667 versus Visio 0.3668267852610678. Fine height rounding remains:
521 TxtHeight and 382 Height cases have approximately -0.000002in error, and 215
TxtHeight cases approximately +0.000002in. Another 212 TxtHeight cases require
unsupported FIND. These are overlapping diagnostic subsets, not new exclusions.
The full local report lists the ten most frequent failing cells before and after,
with signed deltas and corpus witnesses.

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

The P2-followup text-metric promotion threshold is 90% in each cell oracle. Below
that threshold, the main comparison uses M2.3 text-hook dependency semantics and
excludes direct text-family formulas; the separately printed text rates always use
real evaluation. Theme remains included. Both main floors are unchanged. A deferred
text rate is not an M2 acceptance pass: the human gate still requires text inclusion.

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


Current M2.4 text witnesses: generated/basic/0001-probe.vsdx master 2 shape 5
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
77x30; text/align-0-2.vsdx is 387x291 versus 385x289. Arrow stroke extents, square
cap semantics, text logical extents and font metrics require implementation fixes.


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
