# Diagramcraft

A clean-room, headless Rust implementation of Microsoft Visio's core: ShapeSheet, document
model, file interoperability, rendering and authoring. Stage: pre-alpha, Phase 0.

Today the workspace and bounded OPC reader compile. `info` lists package parts, content types,
sizes and root relationships for .vsdx, .vssx, .vstx and .vsdm. It does not execute macros.
There is no document reader, formula evaluator, renderer, writer or authoring engine yet.
Other CLI subcommands print "not implemented in Phase 0" and exit 2.

## Build and check

```sh
cargo build --workspace
cargo xtask ci
cargo xtask parity
cargo run -p diagramcraft-cli -- info FILE.vsdx
```

Use Rust edition 2024 with rustfmt, clippy and wasm32-unknown-unknown available. The local
`.cargo/config.toml` sets CRAFT_FONTS_DIR to the read-only craft-fonts checkout. Override that
environment variable for another checkout; BIZ UDPGothic Regular is required as fallback.

See AGENTS.md for operating rules and docs/architecture.md for layers and provenance.
Generated Visio oracles live in a permanently private companion corpus repo. Microsoft's
shipped stencils and templates stay local under gitignored corpus/. Neither ships here.

Licensed MIT OR Apache-2.0. See ATTRIBUTION.md and NOTICE.

The private reference corpus is pinned by `xtask/src/corpus_pins.rs`. With its clean checkout
at `../diagramcraft-corpus`, run `cargo xtask corpus --local` to copy and SHA256-verify it.
`--update-manifest` is for an intentional new pin, never for accepting a mismatch.
P0 remains partial because the Visio acceptance script stalled in Documents.Open.
Shipped dumps cover 104/1181 files and can resume on later nights; all shipped files are local.

The Craft name belongs to the upstream project and is theirs to keep or drop if they adopt this work.

The Phase 1 reader library loads all 1,387 local corpus packages; deterministic mutation tests cover each package. Evaluation is in progress.
