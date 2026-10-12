# Diagramcraft

A clean-room, headless Rust implementation of Microsoft Visio's core: ShapeSheet, document
model, file interoperability, rendering and authoring. Stage: pre-alpha, M1 reader and
ShapeSheet milestone met.

The reader loads .vsdx, .vssx and .vstx packages and strips macro payloads from .vsdm.
`info` lists package parts; `sheet` evaluates cells and prints the COM-compatible TSV columns.
`corpus check` compares computed values against package caches and local Visio dumps,
reporting denominators and frozen acceptance floors, plus the generated render comparisons. Unsupported formulas return errors.
Experimental PNG, SVG and vector PDF rendering is available. Pixel parity
has not passed the Phase 2 gate. Writing and authoring remain future phases.

## Try it

```sh
cargo build --workspace
cargo test --workspace
cargo run -p diagramcraft-cli -- info /path/to/your-drawing.vsdx
cargo run -p diagramcraft-cli -- sheet /path/to/your-drawing.vsdx --page 1
cargo run -p diagramcraft-cli -- render /path/to/your-drawing.vsdx --page 1 output.png
```

Use your own `.vsdx` file. Corpus-dependent tests print a one-line skip message when
`corpus/` is absent (use `cargo test --workspace -- --nocapture` to see test output).
When the local corpus exists, cell floors and structural tests remain mandatory.
Until the M2 human gate, `cargo xtask ci` prints render comparisons as advisory;
`corpus check` and direct render comparison tests still fail on the frozen ceilings.

`cargo run --release -p diagramcraft-cli -- corpus check` needs the generated drawings,
blank-document dumps, shipped packages and completed COM dumps under `corpus/`.
SVG/PDF validation also requires a local `.venv` with `pypdf` installed.
The reference corpus is private by design: Visio-generated files can embed Microsoft
masters and artwork. It is not distributed with this public repository. The scripts in
[tools/oracle](tools/oracle/) are the regeneration path for a local Visio installation;
see [the oracle runbook](tools/oracle/README.md) for the VM workflow.

The embedded Latin faces work in a public checkout. Additional craft-fonts faces are optional. `.cargo/config.toml` uses the relative
`../craft-apps/craft-fonts/src` path; set `CRAFT_FONTS_DIR` to your own checkout to override
it. If the directory is absent, the build warning refers to the optional sibling faces;
the vendored Latin faces remain embedded. No fonts are downloaded automatically.

## Build and check

```sh
cargo build --workspace
cargo xtask ci
cargo xtask parity
cargo run -p diagramcraft-cli -- info FILE.vsdx
cargo run -p diagramcraft-cli -- sheet FILE.vsdx --page 1
cargo run --release -p diagramcraft-cli -- corpus check
```

Use Rust edition 2024 with rustfmt, clippy and wasm32-unknown-unknown available. The local
`.cargo/config.toml` points at the optional sibling craft-fonts checkout. Carlito is the last-resort Latin face, ahead of the optional BIZ UDPGothic fallback.

See AGENTS.md for operating rules and docs/architecture.md for layers and provenance.
Generated Visio oracles live in a permanently private companion corpus repo. Microsoft's
shipped stencils and templates stay local under gitignored corpus/. Neither ships here.

Licensed MIT OR Apache-2.0. See ATTRIBUTION.md and NOTICE.

The private reference corpus is pinned by `xtask/src/corpus_pins.rs`. With its clean checkout
at `../diagramcraft-corpus`, run `cargo xtask corpus --local` to copy and SHA256-verify it.
`--update-manifest` is for an intentional new pin, never for accepting a mismatch.
M0 is met: normal-process Visio launch and COM attachment passed the two-file acceptance.
Shipped dump coverage is reported by `corpus check`; the shared inputs can grow between runs.

The Craft name belongs to the upstream project and is theirs to keep or drop if they adopt this work.

The Phase 1 reader library loads all 1,387 local corpus packages; deterministic mutation tests cover each package. Evaluation accuracy is being measured against the Phase 1 floors.

Carlito, Caladea, Arimo, Tinos and Cousine are embedded under the SIL Open Font License 1.1; a separate craft-fonts checkout is optional.

## Private master comparisons

Render a labelled contact sheet with
`cargo run --release -p diagramcraft-cli -- render --masters STENCIL.vssx OUT.png`.
Each master is scaled into its cell before rasterization. The pixel cap applies to the
whole sheet; a failed master is marked and the command returns an error.

For the local human gate, `corpus/_state/m2-contact-inputs.txt` lists the stencil paths.
Generate their `corpus/_state/contact-sheets/<stem>-ours.png` files, then run
`cargo run --release -p diagramcraft-corpus --example contact_pairs`.
This pairs each master with its existing private Visio export from the oracle runner:
ours on the left, Visio on the right. Neither reference images nor paired sheets ship
with the public repository. See the oracle runbook for `export-masters.ps1`.

Run the explicit release performance measurement with
`cargo test --release -p diagramcraft-render --test perf -- --ignored --nocapture`.
The observations in `perf/budgets.toml` are measurements, not acceptance limits.
