# Changelog

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
  from a script.
- **Automate** through the MCP stdio server (`misterdiagram mcp`): sixteen tools for opening,
  building, inspecting, rendering and saving drawings.
- A built-in network stencil with eleven device masters, and vendor stencils you already have.

### Known gaps

- **Large drawings are slow to build.** Every command republishes the whole drawing, so a
  2,500-command drawing of about 380 shapes takes about 13 minutes
  ([#39](https://github.com/MisterDiagram/MisterDiagram/issues/39)).
- **Routing differs from Visio on larger pages.** Routes are valid and Visio keeps them when it
  saves the file again, and they match Visio's on 28 of 29 witness drawings. On larger pages
  Visio would often choose a different path between the same shapes. Parallel connectors get
  separate lanes, spaced by a rule that matches Visio exactly on rows and columns of boxes and
  approximates it elsewhere. docs/gaps.md has the measurements (the paragraph that begins
  "Connector lanes").
- **Connectors go around placeable shapes only.** A shape counts as an obstacle when a
  connector is glued to it or its ObjType cell is 1; a shape with nothing glued to it is not
  avoided. See docs/install.md, "Connectors and obstacles".
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
