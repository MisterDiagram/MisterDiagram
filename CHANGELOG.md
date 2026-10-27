# Changelog

## Unreleased

- **Symbol characters draw.** Check marks, crosses, dingbats, geometric shapes and the other
  symbols that Windows documents set in Segoe UI Symbol rendered as the missing-glyph box,
  because that font is Microsoft's and system fonts are never scanned
  ([#106](https://github.com/MisterDiagram/MisterDiagram/issues/106)). Noto Sans Symbols 2
  (OFL 1.1, symbols only) is now bundled and draws such a character when the text face
  cannot; the line keeps the text face's metrics, so nothing else moves. Each binary grows
  by about 655 KiB.
- **A lone symbol with a trailing no-break space stays on one line.** When the pair was
  wider than its block and the line had no earlier break opportunity, the layout forced a
  break before the space, opened an empty extra line and recentred the text; Visio lets the
  space hang past the margin, and so does the layout now. Wrapping at an earlier opportunity
  is unchanged.

## 0.1.0-alpha.11

One authoring fix from the first batch of real company files
([#87](https://github.com/MisterDiagram/MisterDiagram/issues/87)), with the rule taken from
Visio 2021 on five witnessed edits. Still open from that batch: symbol characters set in Segoe
UI Symbol draw as the missing-glyph box
([#106](https://github.com/MisterDiagram/MisterDiagram/issues/106)), SVG text is exported as
outlines ([#102](https://github.com/MisterDiagram/MisterDiagram/issues/102)), SVG import
([#103](https://github.com/MisterDiagram/MisterDiagram/issues/103)), and
`INTERSECTX`/`INTERSECTY` on guide-aligned shapes take the saved value with a warning.

- **An edit reroutes only the connectors with a reason to move**
  ([#104](https://github.com/MisterDiagram/MisterDiagram/issues/104)). Moving a label or
  any unrelated shape used to reroute every dynamic connector on the page, unglued ones
  included, and drop their line jumps. A connector now reroutes when its glue or endpoints
  change, when a placeable shape moves onto its route (ConFixedCode 0 only, as Visio does),
  or on an explicit request: `route.reroute`, a layout command, a page routing setting, or
  setting that connector's `ShapeRouteStyle`. On the five witnessed edits the changed sets
  equal Visio's exactly. Every other connector keeps its Shape XML byte for byte, bends and
  jumps included; line jumps are refreshed only where a rerouted connector changed a crossing.
- **Shape-local routing cells no longer reroute by themselves.** Changing `ShapePermeableX`,
  `ShapePermeableY` or `ShapePlowCode` on a shape takes effect on the next explicit reroute,
  consistent with the rule above. A group flagged ObjType 8 is not an obstacle; its placeable
  members are.
- Where a connector does reroute, its path is still this router's (28 of 29 geometry witnesses):
  all three connectors that rerouted on the witness drawings differ from Visio's path, two on
  the other side of the obstacle and one with its bend elsewhere.

## 0.1.0-alpha.10

Three render fixes from the first batch of real company files
([#87](https://github.com/MisterDiagram/MisterDiagram/issues/87)), each checked against Visio.
Still open from that batch: an edit reroutes every connector on its page
([#104](https://github.com/MisterDiagram/MisterDiagram/issues/104)), symbol characters set in
Segoe UI Symbol draw as the missing-glyph box
([#106](https://github.com/MisterDiagram/MisterDiagram/issues/106)), SVG text is exported as
outlines ([#102](https://github.com/MisterDiagram/MisterDiagram/issues/102)), and
`INTERSECTX`/`INTERSECTY` on guide-aligned shapes take the saved value with a warning.

- **Guides are no longer drawn.** A page with a guide (a view-only `Guide` shape with an
  infinite dotted line) failed `render` and `convert` outright with `scene path limit (dash
  expansion)`, because the line was expanded over a 20,000 in extent. Visio never prints or
  exports guides, so the scene skips them; an infinite line on an ordinary shape is now
  clipped to the page before its dashes are counted
  ([#107](https://github.com/MisterDiagram/MisterDiagram/issues/107)).
- **No line ends on closed paths.** A rectangle whose sheet carries `EndArrow` or
  `BeginArrow` drew an arrowhead at its corner. Visio draws line ends on open paths only
  ([#105](https://github.com/MisterDiagram/MisterDiagram/issues/105)).
- **Transparent SVG page.** `convert` and the MCP `export` tool wrote a page-sized white
  rectangle behind the drawing; Visio's own SVG export has none, so it is gone. PNG output
  keeps its white page ([#101](https://github.com/MisterDiagram/MisterDiagram/issues/101)).

## 0.1.0-alpha.9

Very large templates no longer report `#LIMIT!` just because dependency tracking is full
([#92](https://github.com/MisterDiagram/MisterDiagram/issues/92)).

- **Formula evaluation continues when the dependency graph fills.** The evaluator records at
  most 2 million dependency edges (and 256 MiB of cell names). Before, reaching that cap made
  every later cell report `#LIMIT!`: on Microsoft's social-distancing template `sheet` printed
  it for 336,253 cells, on the Floor plans template for 1,068,519, every cell from one shape
  onward. Now the graph is marked incomplete and evaluation goes on; both dumps come out with
  no `#LIMIT!` at all and every earlier row unchanged. After an edit under an incomplete graph
  the whole value cache is discarded rather than trusting a partial dependency record, and
  saving recomputes the caches it queries. Per-formula work and depth limits are unchanged, so
  a cell whose own formula hits a limit still reports `#LIMIT!`.
- **Measured, not asserted.** On Apple Silicon `sheet` on the two templates takes 20 and 25
  seconds and peaks at 1,358 and 1,381 MiB; `render --page 1` takes about 2 seconds; a
  1,949,602-cell synthetic dump peaks at 1,685 MiB. These are measured workloads, not a
  guarantee. Every corpus number, the 85 of 85 field renders and the hostile set's 27 exits are
  unchanged from alpha.8.

Known gaps: `sheet` still exits 1 after writing a complete dump when an embedded picture's
format cannot be determined, where render recovers with a warning
([#100](https://github.com/MisterDiagram/MisterDiagram/issues/100)). The feet-and-inches
picture family is still unimplemented (#95 stays open for it); `THEME()` with a colour argument
(#88), the Text Callout cycle (#91) and thin-line arrowheads (#93) remain open. The alpha.5
gaps still stand; locale-specific number and date pictures remain an en-US approximation.

## 0.1.0-alpha.8

Larger inherited drawings render, and evaluator construction failures explain the limit
([#97](https://github.com/MisterDiagram/MisterDiagram/issues/97)).

- **The evaluator admits up to 2 million effective cells and 2 million indexed cells.**
  A contributor-original drawing with 1,949,602 effective cells rendered successfully;
  its largest measured process peak was 1,525.19 MiB during `sheet`, below the 2,048 MiB
  default session ceiling. This is a measured workload, not a universal process-memory
  guarantee; evaluator work is separate from the session's retention budget.
- **A larger inheritance product still fails with a useful message.** Render, convert,
  export, sheet and MCP name the construction limit, attempted count, cap and stage,
  so the count shows how far the request exceeded the cap. Individual evaluated cells
  still report the ShapeSheet value `#LIMIT!` when their own evaluation hits a limit.

Known gaps: the feet-and-inches picture family is still unimplemented (#95 stays open for it);
`THEME()` with a colour argument (#88), the Text Callout cycle (#91), the `sheet` budget on very
large templates (#92) and thin-line arrowheads (#93) remain open. The alpha.5 gaps still stand;
locale-specific number and date pictures remain an en-US approximation.

## 0.1.0-alpha.7

Two more recoveries from the template catalogue, so a page draws what Visio drew instead of
stopping on a cell it cannot compute.

- **A shape whose position or size cannot be evaluated keeps the one saved in the file.**
  The transform cells (Width, Height, pins, angle, flips, 1-D endpoints and the text block's)
  now take the saved value when their formula errors, even where Visio itself recorded the
  error, because Visio draws the retained value in that case: the database-modelling masters'
  attribute rows (`Height=GUARD(REF())`) render instead of stopping the page
  ([#96](https://github.com/MisterDiagram/MisterDiagram/issues/96)).
- **A text field that cannot be formatted draws the text saved in the file.** Render, convert
  and export show what Visio last displayed, with a warning naming the field, instead of
  stopping the page; strict evaluation still reports the error. The feet-and-inches picture
  itself is still unimplemented
  ([#95](https://github.com/MisterDiagram/MisterDiagram/issues/95)).

Known gaps: the feet-and-inches picture family is still unimplemented (#95 stays open for it);
the construction budget on a 2,780-shape template ([#97](https://github.com/MisterDiagram/MisterDiagram/issues/97)),
`THEME()` with a colour argument (#88), the Text Callout cycle (#91), the `sheet` budget on very
large templates (#92) and thin-line arrowheads (#93) are as listed under alpha.6; the alpha.5
gaps still stand.

## 0.1.0-alpha.6

Four small fixes from the first week of tester files, each verified against the corpus and
the field gate.

- **CHAR() is implemented.** A Unicode code point becomes its character, so shape labels
  built with `CHAR(10)` line breaks evaluate instead of reporting `#NAME?`
  ([#89](https://github.com/MisterDiagram/MisterDiagram/issues/89)).
- **FORMAT and FORMATEX accept every language ID.** Files saved by a non-US Visio carry
  their LangID (2057 for en-GB) in every formatted field; those cells now format with the
  en-US picture rules instead of failing. Locale-specific number and date pictures remain an
  approximation listed in the gaps ([#90](https://github.com/MisterDiagram/MisterDiagram/issues/90)).
- **Corpus ceilings fall.** The errors-where-the-witness-has-a-number count drops from
  31,386 to 29,115 cached and from 1,095,070 to 1,094,960 COM; the pinned ceilings follow it down.
- **A file that is not a ZIP is named for what it is.** An HTML page saved as .vsdx, a legacy
  binary .vsd, an empty file or a PDF now says so, quoting the first bytes, instead of the ZIP
  reader's "Could not find EOCD"; render, convert, info and the MCP server print the same line
  ([#94](https://github.com/MisterDiagram/MisterDiagram/issues/94)).

Known gaps, new since alpha.5 and filed from Microsoft's template catalogue: a text field
whose picture is feet-and-inches (`<,FEET/INCH>0 u`) fails and a text-field error still stops
the render ([#95](https://github.com/MisterDiagram/MisterDiagram/issues/95)); a master
shape's Height saved as `GUARD(REF())` stops the page shape that inherits it
([#96](https://github.com/MisterDiagram/MisterDiagram/issues/96)); a 2,780-shape template
trips the evaluator's construction budget before any page draws
([#97](https://github.com/MisterDiagram/MisterDiagram/issues/97)); `THEME()` with a colour
argument ([#88](https://github.com/MisterDiagram/MisterDiagram/issues/88)), the Text Callout
cycle ([#91](https://github.com/MisterDiagram/MisterDiagram/issues/91)), the `sheet` command's
budget on very large templates ([#92](https://github.com/MisterDiagram/MisterDiagram/issues/92))
and arrowheads on thin lines drawn about half Visio's size
([#93](https://github.com/MisterDiagram/MisterDiagram/issues/93)). The alpha.5 gaps below
still stand; locale-specific number and date pictures are an en-US approximation.

## 0.1.0-alpha.5

More real-world drawings render, with useful errors when evaluation fails.

- **Imported diagrams survive duplicate shape identities and detached cells.**
  Seven formerly rejected packages now render every source page. A stopping
  evaluation error identifies its page, shape and cell, including dependencies
  in document, master, style and layer sheets
  ([#84](https://github.com/MisterDiagram/MisterDiagram/issues/84)).
- **One bad geometry row no longer stops the drawing.** Render, convert and
  export use saved geometry and connection values when available, or skip the
  affected row with a named warning. REF() reports the native reference error;
  named User cells resolve page-scale references. Currency amounts, imported
  numeric cell text, palette identities and sparse inspection columns are handled
  more accurately. Colour shading near black and white follows Visio more closely.
  Strict inspection keeps errors visible ([#85](https://github.com/MisterDiagram/MisterDiagram/issues/85)).
- **Unsupported embedded objects keep a visible representation.** Their cached
  picture or icon is drawn when available; otherwise a sized rectangle marks
  the object. A warning identifies the shape and object type. Primary media
  byte, pixel, allocation and operation limits still return errors
  ([#86](https://github.com/MisterDiagram/MisterDiagram/issues/86)).
- **Numerical evaluation failures have their own regression count.** Cached and
  Visio numeric witnesses that evaluate to errors are counted separately from
  text-metric differences, with ceilings that may only fall
  ([#81](https://github.com/MisterDiagram/MisterDiagram/issues/81)).

Known gaps: rendering succeeds on all 85 native-opened field files, but complete
appearance and cell parity remain open. Imported timeline date labels and two
caption colours differ; imported successful transform results can retain positions
that differ from formula recalculation. Stored ActiveX pictures differ from live controls;
3D rotation, fine equipment detail, repeated-master strict evaluation and some
master cells still differ. The theme sweep retains
2,550 unequal secondary backgrounds and dormant gradient angles. Container/list
formulas, text wrapping, larger-page routing, legacy formats, EMF+, compound line
patterns, shadows, PowerPoint export, large-drawing authoring speed and masters
with several top-level shapes remain limited. No third-party specimen is shipped.

## 0.1.0-alpha.4

Clearer themed fills, readable connector labels and working colour variants.

- **Themed gradients keep their actual colours.** Unused stop slots no longer
  wash the fill toward white. Colour transforms retain the precision Visio
  uses, including saturation above 100%
  ([#79](https://github.com/MisterDiagram/MisterDiagram/issues/79)).
- **Connector labels use connector text styles.** Labels that were white on
  white backgrounds are readable, and connector line colours and weights
  follow their own style entries
  ([#80](https://github.com/MisterDiagram/MisterDiagram/issues/80)).
- **Colour and style variants resolve without reference errors in the theme
  sweeps.** The 200-range colours retain their distinct behaviour in monotone
  variants, and theme font names survive installation-specific font IDs
  ([#82](https://github.com/MisterDiagram/MisterDiagram/issues/82)).

Known gaps: some themes still produce different secondary fill backgrounds
and dormant gradient angles. The strict theme sweep has 261,855 of 264,405
rows equal; the remaining 2,550 are visible failures. This is not complete
THEMEVAL parity. Existing limitations remain in container/list formulas,
text wrapping, larger-page routing, legacy formats, EMF+, compound line patterns,
shadows, PowerPoint export, large-drawing authoring speed and masters with several
top-level shapes.


## 0.1.0-alpha.3

Scaled stencil drops, complete dotted lines and reusable stencil folders.

- **Rack and other scaled masters land at the size Visio uses.** Dropping a 19-inch
  1:10 faceplate onto a normal 1:1 page produces a 1.9-inch shape. Drawing-scale
  types and the observed eightfold threshold control conversion; explicit width
  and height still win. Master discovery reports scale and the size on the current
  page before a drop ([#68](https://github.com/MisterDiagram/MisterDiagram/issues/68)).
- **Newly dropped shapes save without redundant inherited values.** Matching
  literals use the native inherited representation, while explicit units and
  destination-page layer membership remain intact. Scaled drops use the native
  numeric cache representation and ordinary line-master classification
  ([#68](https://github.com/MisterDiagram/MisterDiagram/issues/68)).
- **Dotted lines keep their marks in PNG, SVG and PDF.** Dots no longer disappear,
  and thin patterned strokes keep the correct spacing across export scales
  ([#70](https://github.com/MisterDiagram/MisterDiagram/issues/70)).
- **Open stencils by name from your own folders.** Set `MISTERDIAGRAM_STENCIL_PATH`
  or `--stencil-path`, then use `stencil.open` with a filename, optionally without
  `.vssx`. `stencil.available` and the MCP `stencil_available` tool list filenames,
  folders and sizes without opening files. `stencil.list` reports resolved paths
  ([#71](https://github.com/MisterDiagram/MisterDiagram/issues/71)).

These remove the scale-drop, missing-dot and missing-search-path limitations
listed for alpha.1. Masters containing several top-level shapes remain unsupported.
All 13 synthetic scale and dotted-line drawings now open and save in Visio
with zero strict representation differences. Two faint-dot differences in
Visio's own 96 dpi export are documented in
[docs/file-format-parity.md](docs/file-format-parity.md). Other previously listed
gaps remain, including container/list formulas, text wrapping, legacy formats,
EMF+, shadows and PowerPoint export.

## 0.1.0-alpha.2

The first fix from a tester report.

- **A file that uses Visio's container and list functions renders.** A BPMN diagram or
  cross-functional flowchart stopped the whole render with `shape 7 transform: #VALUE!`
  ([#78](https://github.com/MisterDiagram/MisterDiagram/issues/78)). Two changes. The
  "never wrap" text measure a container heading uses, `TEXTHEIGHT(TheText,65535)`, is now
  accepted. And when a cell cannot be computed (the container and list functions
  `CONTAINERSHEETREF`, `LISTORDER` and `LISTMEMBERCOUNT` are still missing), `render`,
  `convert` and the MCP `export` take the value Visio saved in the file and print a warning
  naming the cell, so an unedited file draws the way Visio last drew it. Cells you changed
  have no saved value and still report their error; `sheet` and the MCP inspection tools keep
  showing the error.

## 0.1.0-alpha.1

The first binaries. This is an alpha for testers: it is checked against Visio 2021 on drawings
our own scripts made, and real drawings are the next test. Install notes are in
[docs/install.md](docs/install.md). There is no GUI; MisterDiagram is a command-line tool and an
MCP server for AI agents.

Builds: macOS (one universal binary for Apple silicon and Intel), Windows x64 and arm64, Linux
x64 and arm64 (static, any distribution). The binaries are not signed; docs/install.md explains
the macOS and Windows warnings and how to check the download against SHA256SUMS. Each archive
carries THIRD-PARTY-LICENSES.txt, the licences of the Rust crates linked into the binary.

### What works

- **Open** .vsdx, .vssx and .vstx files and their macro-enabled variants (the VBA is dropped).
  All 1,181 stencils and templates that ship with Visio 2021 open with zero errors.
- **Read** every cell the way Visio evaluates it: `sheet` prints the evaluated ShapeSheet in the
  same columns as a Visio COM dump.
- **Render** pages and stencils to PNG, SVG and PDF.
- **Write** drawings that Visio 2021 opens without repair, and keep every cell of a file you open
  and save again (15.1 million cells across 1,413 test packages).
- **Author** drawings from commands: pages, layers, shapes, text, shape data, hyperlinks,
  formatting, stencils and master drops, connection points, glued connectors with orthogonal
  routing and line jumps, and tree, row and fan layouts. `run --batch FILE` rebuilds a drawing
  from a script. As in Visio, connectors route around placeable shapes: a shape a connector is
  glued to, or one whose ObjType cell is 1 (docs/install.md, "Connectors and obstacles").
- **Automate** through the MCP stdio server (`misterdiagram mcp`): sixteen tools for opening,
  building, inspecting, rendering and saving drawings.
- A built-in network stencil with eleven device masters, drawn as cards so connectors meet
  the outline on every side, and vendor stencils you already have.

### Known gaps in alpha.1

The scale-drop, missing-dot and stencil-search-path gaps below were addressed
in alpha.3; this list records the limitations of alpha.1.

- **Large drawings are slow to build.** Every command republishes the whole drawing, so a
  2,500-command drawing of about 380 shapes takes about 13 minutes
  ([#39](https://github.com/MisterDiagram/MisterDiagram/issues/39)).
- **Routing differs from Visio on larger pages.** Routes are valid and Visio keeps them when it
  saves the file again, and they match Visio's on 28 of 29 witness drawings. On larger pages
  Visio would often choose a different path between the same shapes. Parallel connectors get
  separate lanes, spaced by a rule that matches Visio exactly on rows and columns of boxes and
  approximates it elsewhere. docs/gaps.md has the measurements (the paragraph that begins
  "Connector lanes").
- **Vendor masters drawn at another scale land too big.** A 1:10 rack master dropped on a 1:1
  page comes out ten times its size
  ([#68](https://github.com/MisterDiagram/MisterDiagram/issues/68)).
- **Dotted lines** render with dots missing
  ([#70](https://github.com/MisterDiagram/MisterDiagram/issues/70)).
- **No stencil search path yet:** `stencil.open` needs the full path to a stencil file
  ([#71](https://github.com/MisterDiagram/MisterDiagram/issues/71)).
- **Text wrapping** is the weakest part of rendering; **EMF+ images** are not drawn; compound
  line patterns are approximate; 32-bit BMP transparency is ignored
  ([#40](https://github.com/MisterDiagram/MisterDiagram/issues/40)); some image crops, PDF
  glyph positions and text backgrounds differ
  ([#54](https://github.com/MisterDiagram/MisterDiagram/issues/54)).
- **No legacy formats.** Only the XML formats Visio 2013 and later write are read: .vsd, .vss,
  .vst and .vdx files are refused. Open old files in Visio and save them as .vsdx or .vssx
  first ([#19](https://github.com/MisterDiagram/MisterDiagram/issues/19),
  [#20](https://github.com/MisterDiagram/MisterDiagram/issues/20)).
- **Non-Latin text uses your system's fonts.** The binary embeds five Latin font families
  (metric-compatible with Calibri, Cambria, Arial, Times New Roman and Courier New). Japanese,
  Chinese, Korean, Arabic, Thai and other scripts render with whatever fonts the system has; a
  minimal Linux install may have none, and those characters then render as boxes.
- **Shadows** are not drawn ([#23](https://github.com/MisterDiagram/MisterDiagram/issues/23)).
- **No PowerPoint export yet.** It comes in a later release.

The full list is in [docs/gaps.md](docs/gaps.md).

### How to report

Open an [issue](https://github.com/MisterDiagram/MisterDiagram/issues) with the output of
`misterdiagram --version`, the command you ran and what you expected. Please do not attach
drawings that hold customer names, addresses or other private data: a small file that shows the
same problem, or a screenshot and a description, is enough.

### Privacy

MisterDiagram collects nothing and sends nothing: no telemetry, no update check, no network
connections. What was checked for this release: Cargo.lock holds no HTTP, TLS, socket or async
runtime crate (no hyper, reqwest, ureq, curl, tokio, mio, rustls, native-tls or openssl), and the
only socket code in the source is an unused loopback client in crates/mcp/src/backend.rs that
the command-line tool never constructs. The binary writes only the files you ask it to write;
besides your own files it reads the system's font folders, to find fonts a drawing names.
