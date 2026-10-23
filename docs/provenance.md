# Provenance

## 1. Target observed

Visio Standard 2021 build 16.0.20430 was driven by scripts on drawings our scripts made.
Visio executable code was not read or disassembled. No screenshots are committed.

## 2. Format sources

Format work uses the [MS-VSDX open specification](https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/),
Microsoft Learn's [.vsdx reference](https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx)
and [ShapeSheet reference](https://learn.microsoft.com/en-us/office/client-developer/visio/visio-shapesheet-reference).
Every implemented element, cell and function cites its reference page in its doc comment.

## 3. Third-party projects consulted

Brief 36 (2026-10-23): the private observation records
`field/themeval-2026-10-22/README.md` and `FINDINGS.md`, plus
[MisterDiagram issue 79](https://github.com/MisterDiagram/MisterDiagram/issues/79),
[issue 80](https://github.com/MisterDiagram/MisterDiagram/issues/80) and
[issue 82](https://github.com/MisterDiagram/MisterDiagram/issues/82), supplied
reported symptoms and experiment leads. Immutable TSV values and fresh
synthetic native controls govern the implementation where the notes disagree.
No sample, theme XML, artwork or third-party implementation was copied into
this repository. Public tests use contributor-written numerical themes.

Brief 34 (2026-10-21): Charles's local observation record
`craft-apps/scope/visio/execute/32-router-capacity.md`, result section, supplied
the native-large-300 lane finding and coordinates that motivated new synthetic
VM witnesses. No third-party implementation or asset was copied. Public
Microsoft ShapeSheet pages define the referenced routing cells; the lane
recurrence and all experimental geometry are contributor-original observations.

Brief 30 (2026-10-20): the outside review recorded in
[MisterDiagram issue 77](https://github.com/MisterDiagram/MisterDiagram/issues/77)
was consulted to locate the 32-bit EMF bitmap-offset overflow and define its
synthetic regression. The fix uses this module's existing checked slice helper.
No third-party implementation or asset was copied.

Brief 26 (2026-10-17): Charles's local scoping records
`craft-apps/scope/visio/stencil-strategy.md` (Root causes found) and
`craft-apps/scope/visio/stencil-survey/INVENTORY.md` were consulted for the
reported failure patterns and original vendor/master counts. The local
`render_all.sh` was read for the acceptance procedure. No vendor package,
artwork, extracted part or render was copied into either repository. The new
Action witness is contributor-original procedural code, observed in Visio.


The [libvisio project README](https://github.com/LibreOffice/libvisio) was consulted
on 2026-10-12 for project scope and licence metadata only. No implementation,
sample documents or assets were used.

The [Apache Tika MIME proposal TIKA-1286](https://issues.apache.org/jira/browse/TIKA-1286)
was consulted for the .vssm and .vstm main-part content type identifiers.
No implementation or attached files were used.

Record each non-Microsoft documentation consultation about Visio or its formats
here in the same commit, with the source and how it was used.
Permissive sibling implementation reuse is recorded separately in section 7.

M3.18: W3C XML 1.0 fifth edition, sections 2.11 and 3.3.3
(https://www.w3.org/TR/xml/#sec-line-ends and https://www.w3.org/TR/xml/#AVNormalize),
consulted 2026-10-13 to correct physical newline and attribute-whitespace normalization.
The 0.38.4 quick-xml API documentation in the installed crate
(https://docs.rs/quick-xml/0.38.4/quick_xml/events/struct.BytesText.html#method.xml10_content)
confirmed the existing decoder's normalization API. No implementation was copied.
The witness is 0001-probe's Text/Icon: our writer had turned physical CRLF into explicit
carriage-return references, changing the parsed text. This is our defect, not a Visio
normalization exception; character references still preserve their specified characters.

Brief26b: [ntwrk-clean-and-flat README and LICENSE](https://github.com/DukeNuke3D/ntwrk-clean-and-flat/tree/211ba3c9e7f1ed895f6ad4d5dbd67e7cb4b731a5), consulted 2026-10-21 for author, redraw provenance and the Unlicense dedication. The pinned stencil is the user-authorized test-only fixture; no implementation was copied.

## 4. Excluded content

This repository never contains Microsoft's shipped stencils, templates, fonts, icons or
copied descriptive wording. Shipped files are observed only in ignored local corpus storage.

## 5. From-scratch connector master

M3.1 reconstructs functional ShapeSheet defaults as typed Rust model records in
`crates/model/src/skeleton_defaults.rs`; it does not embed a Visio XML part or asset.
The blank witness is `corpus/blank/blank.vsdx`, SHA256
`4dc5072775cdd3e2b5ff253e9d38569a685aa94d353facd68d5ab4f3ac006dea`.
Its `document.tsv`, `page.tsv`, six `style-*.tsv` files and `rectangle.tsv` are the
resolved observations; serialization cell sets are checked against the matching
blank packages by `skeleton_cell_set_matches_blank_and_probe_observations`.

The six blank styles are exactly 0, 1, 2, 3, 4 and 6, with respectively 294, 60,
48, 13, 103 and 160 serialized cells. Styles 5 and 7 are absent from the blank.
DocumentSheet has ten local cells and PageSheet seventeen; inherited COM cells
remain inherited. The skeleton also reproduces the blank's functional document
settings, palette entry 24 (neutral grey), and Calibri family name. No font bytes,
Panose table, theme artwork or UI customization is copied. Rectangle geometry
is authored with the observed five-row path contract.

The connector witness is `corpus/generated/basic/0001-probe.vsdx`, SHA256
`edfdc75c58b86ffe7f8b99ed0b18f5ebee44152985de8648474d467232af7ae7`:
master 2, shape 5, its page sheet, and connector style 7 (72 local cells).
Cell names, formulas, units and serialization defaults are reproduced. The
copyright and HelpTopic cells, icon, vendor prompt, and fixed UniqueID/BaseID
are omitted. Both identities are fresh UUID v4 values from the `uuid` crate;
the short prompt is contributor-authored. There is no copied graphic.

Windows metadata uses contributor-chosen portable dimensions. Core/app/custom
property trees are authored, with application name MisterDiagram, no copied
personal metadata or timestamps, and the observed BuildNumberCreated /
BuildNumberEdited value 1074417614 plus the requested IsMetric flag. The
numeric build records the format-compatibility witness, not a claim that
MisterDiagram is Microsoft Visio. No thumbnail is included at this step;
acceptance of missing versus authored PNG thumbnails remains an M3.5 test.

Sources are Microsoft's VisioDocument, DocumentSettings, StyleSheet, Cell,
Master and Windows element references, cited in the constructor doc comments.
Writing and Visio acceptance remain separate steps; model construction alone
is not a claim of an accepted output package.

M3.20 synthetic master UniqueIDs use random first three GUID groups with the
observed suffix `8E40-00608CF305B2`, witnessed by Visio SaveAs of our authored
scratch rectangle-text document. This reproduces serialization form, not a
Microsoft master identity or graphic. The model generates every random prefix
independently with uuid. Charles authorized this in P3-closure-2 M3.20(3).

### M3.26 observed functional defaults

The four authored ColorEntry values (indices 24 to 27), empty style-0
Copyright/HelpTopic, empty inherited connector Comment/ShapeKeywords, full Layer
cell set, authored number precision and redundant connector-cache omission come
from Visio's save of our scratch rectangle, metric/layers and dynamic-glue files
under `corpus/_vm/out/accept/scratch__*/saved.vsdx`. The layer cell names also occur
in `corpus/generated/page/layers.vsdx`. These are functional serialization values,
not copied visual assets. No Icon, Microsoft copyright text or preview is authored.

### M3.32 authored package metadata

Section 5 continuation: the skeleton authors app/core/custom properties, full
Window settings and Calibri FaceName metadata from the structural observations in
`corpus/blank/blank.vsdx` and `corpus/_vm/out/accept/scratch__*/saved.*`.
This includes the app summary vectors, AppVersion 16.0000, UTC creation and
modification fields, DocLangID language, TimeEdited pid 6, window flags and
coordinates, the Windows root xml:space=preserve attribute, UnicodeRanges,
CharSets, Panose and Flags. Application uses the project name, now
MisterDiagram after the 2026-10-16 rename; dates are newly generated. No font binary, icon or vendor artwork
is copied. Foreground page names precede background names in the summary witness.
The authored connector MasterType 541 and authored rectangle MasterType 2 follow
the same saved witnesses, avoiding a needless rewrite. Master Comment and
ShapeKeywords are omitted because Visio removes those redundant local cells;
style sheet 0 retains the observed empty Copyright and HelpTopic cells.


### P4b-closure-2 canonical authored output

Charles's 2026-10-15 ruling admits Visio's retained saves of the contributor-authored
SAN and from-shape rebuild for removal of our redundant pp marker and spelling
of authored app.xml. The implementation uses these XML observations only, with
no artwork copied. Original cells/rows and unchanged imported bytes remain
preserved. Render population membership comes exactly from main commit b3b42ad's
204-file generated manifest, SHA256110f12b555cf9afb250f7deaffbc1df21cea6368b647a974264fceed12958267.

### P4b-closure authored-cell observations

M4.19 (2026-10-15): original synthetic `data/property-rows` and
`master/from-shape` were produced with the public Shape Data/AddNamedRow and
Masters.Drop APIs. Property serialization, MasterType, generated Icon presence
and text markers were observed in their saved XML and COM dumps before writer
changes. All vendor-generated files remain in the permanently private corpus.
The retained SAN re-save is an additional authored-cell witness under Charles's
2026-10-15 ruling; originals take precedence for the same construct.

P4b-closure candidates M4.20 through M4.23 derive the full Property row, authored
MasterType and text markers from those originals. The admitted earlier SAN save
provides inherited-zero and authored-number canonicalization evidence. The text
extent candidate derives from row-aligned retained PNGs and line-layout metrics,
not copied artwork. The fresh closure save confirms size equality and exposes
redundant pp removal despite the original's retained marker. Exact observations,
failed residuals and source-seed receipts are private. No new repair class or
renderer exception was inferred from those observations.

## 6. Private corpus

The corpus is generated by our scripts in licensed Visio, embeds Microsoft's connector
master, stays private and is never published. Generator scripts are public in this repository;
the generated files live in the permanently private corpus repository at `../diagramcraft-corpus`.

## 7. Sibling code

Sibling code is copied under MIT OR Apache-2.0. Each copied or derived file has its source
path, commit and licence recorded in [ATTRIBUTION.md](../ATTRIBUTION.md).

M3.5's five acceptance constructions use the typed skeleton and authored rectangles,
labels, connector instance, layers and master identities. The explicit BASIC_U drop
case reads only the private local stencil and writes only gitignored acceptance
outputs. No stencil, icon or copied drawing enters the public repository. M3.7 removes the experimental authored PNG thumbnail from all scratch outputs
after Charles isolated its open failure. Authored skeletons write no thumbnail;
existing imported EMF thumbnails are retained opaquely.

M3.8 dynamic-glue formulas are observed functional behavior from private
corpus/generated/connector/dynamic-glue.vsdx, not copied graphics. The model helper
writes WALKGLUE endpoints, XFTRIGGER target events, target ObjType1 and both Connects;
cached endpoints come from the caller's page geometry. The scratch reread and hostile
request tests cover this contract. Acceptance requires both live COM Connects rows.

M3.16 authored dynamic connector routing reproduces the serialized state observed in
corpus/generated/connector/dynamic-glue.vsdx, page1 shape3: inherited transform/text
pins, guarded width/height, TextPosition controls, straight Geometry rows with the
unused master row deleted, ConFixedCode6 and membership in a Connector layer. Endpoint
caches come from the caller's geometry. This is observed functional formula/format
behavior, with no copied icon, artwork or Microsoft copyright cell.

### Section 5 addendum: M3.39 authored Page view defaults

Page ViewScale=-1 and centred ViewCenterX/Y are authored from observed
corpus/blank/blank.vsdx and the saved scratch Page witnesses. Metric centres
use the authored A4 page dimensions in internal inches. These scalar format
defaults contain no vendor visual asset. Public Page reference:
https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml

### P4b route and theme observations

The private synthetic sweep at corpus commit e51552a4a1552f608d2fc937988876cde8558ecb
contains 23 route drawings and one theme observation. The recipe, dumps and
exports were generated through the existing scheduled-task oracle runner.
`docs/file-format-parity.md` records the full Geometry, layout, XForm and Connects
tables before router implementation. The route frames, jump row types and
legacy theme slot/shadow interpretation come from those outputs, with the
Microsoft reference pages cited in the recipe and source. No implementation
from Visio or another renderer was read or copied. No theme asset enters this
repository. Existing `line/compound-1` supplies the double-stroke observation.

M4.13 route authoring derives alignment-box attachment, obstacle clearance,
signed frames and Geometry rows from the private P4b sweep table above and
connector/obstacle. The implementation uses original bounded visibility-grid
code, not third-party router code. All comparisons retain1e-6 tolerance.

M4.14 crossing rows, radius, direction and style selection come from the five
matching route/jump witnesses and connector/line-jumps. The sixth sweep case
has the disclosed attachment-packing gap. No oracle or threshold was changed.

M4.15 session invalidation is tested against the original obstacle drawing and
its observed unobstructed pair, with explicit page-style and permeability edits.
No additional Visio behavior is inferred for unwitnessed WalkPreference values.

M4.17 SAN layout, names, Switch master and data values are contributor-original
code. Geometry and glue use the independently observed routing rules above.
Tight clearances never admit an actual obstacle interior. The final Visio
observation is retained privately, including the failed SAN Property/text/master
serialization, font identity and export-height comparisons. No vendor artwork,
font binary or theme asset is committed with the generator. The named packing
gap is checked against only its measured coordinate differences.

Brief 25 uses ecceman/affinity only as a public-domain visual reference for ten
native icon redraws, pinned at48d4a4fa87957cb542678ed5952227028b8657f5. It is
not a Visio implementation or format reference. SVG source files stay local;
ATTRIBUTION.md records each mapping and the original FC-switch contribution.
New engine documents target the observed .20182 Edited compatibility metadata
1074679758; Created retains1074417614 and imported properties remain unchanged.
