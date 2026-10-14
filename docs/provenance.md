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
property trees are authored, with application name Diagramcraft, no copied
personal metadata or timestamps, and the observed BuildNumberCreated /
BuildNumberEdited value 1074417614 plus the requested IsMetric flag. The
numeric build records the format-compatibility witness, not a claim that
Diagramcraft is Microsoft Visio. No thumbnail is included at this step;
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
CharSets, Panose and Flags. Application remains
Diagramcraft; dates are newly generated. No font binary, icon or vendor artwork
is copied. Foreground page names precede background names in the summary witness.
The authored connector MasterType 541 and authored rectangle MasterType 2 follow
the same saved witnesses, avoiding a needless rewrite. Master Comment and
ShapeKeywords are omitted because Visio removes those redundant local cells;
style sheet 0 retains the observed empty Copyright and HelpTopic cells.


## 6. Private corpus

The corpus is generated by our scripts in licensed Visio, embeds Microsoft's connector
master, stays private and is never published. Generator scripts are public in this repository;
the generated files live in the permanently private diagramcraft-corpus repository.

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
