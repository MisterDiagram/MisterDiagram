# DiagramCraft

A clean-room, headless Rust core of Microsoft Visio. It opens .vsdx, .vssx and .vstx files, and their macro-enabled .vsdm, .vssm and .vstm variants with the VBA payload dropped,
evaluates the ShapeSheet the way Visio does, renders pages and stencils to PNG, SVG and
PDF, writes files that Visio opens without repair, and authors drawings from commands: shapes,
masters, shape data, glued connectors with orthogonal routing and line jumps. An MCP server for
AI agents and pptx export are next. It is built in the mould of the Craft apps (deckcraft,
vectorcraft and their siblings) and is offered to that project.

![Four drawings made by this repo's scripts, rendered by DiagramCraft](docs/images/diagramcraft-renders.png)

*DiagramCraft's own renders. Each label gives the mean per-pixel distance from Visio 2021's PNG
export of the same file, measured by the test harness; Visio's exports stay out of the repository.*

## What works today (2026-10-15)

- **Open** every one of the 1,181 stencils and templates that ship with Visio 2021, plus a
  204-drawing test corpus, with zero errors and zero panics under mutation fuzzing.
- **Evaluate** the ShapeSheet: 104 functions, inheritance from masters, style sheets and
  document defaults, dirty-set recalculation. Measured against Visio itself: 97.8% of 4.8
  million formula cells match the value Visio cached in the file, and 98.4% of 76.2 million
  cells match Visio's live COM dumps, theme and text-metric cells included.
- **Render** PNG, SVG and vector PDF. All aggregate feature groups of the test corpus pass their fixed
  pixel ceilings against Visio's own 96 dpi export. Individual connector comparisons
  remain above their ceilings and are tracked as a renderer gap. All 1,108 masters in the
  review stencil set render. PDF text extracts with ligatures and combining marks intact.
- **Write** .vsdx, .vssx and .vstx, and build documents from scratch. Visio 2021 opened all
  230 files of the acceptance set, five of them built from scratch, with no repair beyond a
  few disclosed metadata rewrites, and its COM dumps of the reopened files show zero
  differences. Read, write, read preserves all 15.1 million cells across 1,413 packages.
- **Author** through 52 schema-described commands in a transactional session: pages, layers,
  shapes, text, shape data, hyperlinks, formatting, stencils and master drops, connection
  points, static and dynamic glue, and connector routing. Routes match Visio's own on 28 of
  29 witness drawings (obstacles, route styles, page preferences and line jumps) within one
  millionth of an inch; the one gap is a 1/64 in packing shift where three connectors cross.
  A 24-switch, 40-connector topology built by a script opens in Visio 2021 unrepaired, keeps
  every route when Visio re-saves it, and Visio's export of it matches ours inside the pixel
  ceilings. CI runs the full gate set on Linux, Windows and macOS for every push.

Known gaps are listed in [docs/gaps.md](docs/gaps.md). Text wrapping is the weakest area,
EMF+ images are not drawn, and compound line patterns are approximate.

## Try it

```sh
cargo build --release
cargo run --release -p diagramcraft-cli -- info drawing.vsdx
cargo run --release -p diagramcraft-cli -- sheet drawing.vsdx --page 1
cargo run --release -p diagramcraft-cli -- render drawing.vsdx --page 1 out.png
```

`info` lists the package parts. `sheet` prints every evaluated cell in the same columns as a
Visio COM dump, so the two diff directly. `render` also takes `.svg` and `.pdf` output names,
and `--masters stencil.vssx` draws a contact sheet of a stencil. No binaries are published yet;
Rust stable with edition 2024 is required. There is no GUI and none is planned in this repo.

### Generate a drawing

```sh
cargo run --release -p diagramcraft-cli -- commands
cargo run --release -p diagramcraft-cli -- describe shape.draw.rect
cargo run --release -p diagramcraft-cli -- run \
  --cmd 'shape.draw.rect={"x":1,"y":1,"width":2,"height":1}' \
  --cmd 'shape.text={"id":1,"text":"Example"}' --save example.vsdx
cargo run --release -p diagramcraft-cli -- convert example.vsdx example.svg
bash tools/examples/two-boxes-glued.sh corpus/_state/example
```

`run` executes commands in one transactional session. Page numbers are one-based;
shape IDs are the document's IDs. Lengths use internal inches and angles use radians.
`commands` and `describe` print the parameter schemas. The two-boxes script saves
VSDX and PNG using the document's own connector master. The private probe rebuild
script also checks its result against the local Visio corpus.

Individual connector pixel comparisons are measured and reported as informational
until the connector stroke rendering gap closes; route acceptance is cell and geometry
equality with Visio plus the unchanged group ceilings. The rulings behind each gate are
dated in [ROADMAP.md](ROADMAP.md).

## What is coming

- Phase 5: an MCP server and pptx export, so an AI agent can build a diagram from a
  description and hand over a file Visio opens, with a layout layer so the result reads
  like a drawing a person made (side connection points, spacing, fan-out).
- Carried gaps, named in [docs/gaps.md](docs/gaps.md): the three-crossing packing shift,
  unwitnessed WalkPreference values, theme.apply (Visio refused the variant call under
  COM), connector stroke rendering, and export mapping for small images.

See [ROADMAP.md](ROADMAP.md) for the milestones and the numbers each one has to reach.

## How it is built

Clean room. Visio is observed only as a black box, on drawings this repo's own scripts made,
in a licensed copy of Visio Standard 2021. File formats come from Microsoft's published
references, cited in doc comments and collected in [docs/sources.md](docs/sources.md). No
Microsoft stencils, templates, icons or fonts are in this repository; the reference corpus is
private for that reason, and the scripts to regenerate it with your own Visio are in
[tools/oracle](tools/oracle/). Every milestone is a number produced by the test harness against
Visio's output, with floors that only rise. Details are in
[docs/provenance.md](docs/provenance.md).

Code reused from the Craft apps is MIT OR Apache-2.0 and is listed per file in
[ATTRIBUTION.md](ATTRIBUTION.md).

## Status and handover

Pre-alpha. Milestones M0 to M4 (reading, evaluation, rendering, writing, authoring and
routing) are met and independently reviewed, each under dated gate rulings recorded in the
roadmap; Phase 5 has not started. The name follows the Craft apps' convention and belongs to that project: if they
adopt this work, it transfers with the name; if not, the name changes. The project lives at
[github.com/charles-hood/DiagramCraft](https://github.com/charles-hood/DiagramCraft) until
that transfer happens; any other copy, including forks in community or incubator
organisations, is a mirror. Issues, discussion and pull requests go to this repository.

## Development

Build gates, the oracle VM workflow, corpus pinning and contact sheets are in
[docs/development.md](docs/development.md). Operating rules for agents are in
[AGENTS.md](AGENTS.md); layers and architecture in [docs/architecture.md](docs/architecture.md).

## Licence

MIT OR Apache-2.0; see LICENSE-MIT, LICENSE-APACHE and NOTICE. The embedded fonts Carlito,
Caladea, Arimo, Tinos and Cousine are under the SIL Open Font License 1.1.
