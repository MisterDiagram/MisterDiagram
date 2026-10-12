# File format parity

Status: pre-alpha, M1 met. All acceptance commands and two independent frozen verification passes are clean.

## Measured

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
