# Development

The operational notes that used to live in the README. The README is the overview; this
page is for anyone building, testing or regenerating the oracles.

## Build and gates

```sh
cargo build --workspace
cargo test --workspace
cargo xtask ci        # fmt, clippy, test, render, assets, layers, sources, wasm
cargo xtask parity    # Core=Y checklist rows: D, P or M
cargo run --release -p misterdiagram-cli -- corpus check
```

Use Rust 1.92 or newer, edition 2024, with rustfmt, clippy and the `wasm32-unknown-unknown` target
installed. Corpus-dependent tests print a one-line skip message when `corpus/` is absent
(`cargo test --workspace -- --nocapture` shows it). When the local corpus exists, the cell
floors, render ceilings and structural tests are mandatory and `cargo xtask ci` runs them.

`corpus check` compares computed cell values against the caches in every package and against
the local Visio COM dumps, prints each denominator with its frozen floor, and runs the
generated-corpus render comparison. The render ceilings were frozen at the M2 human gate on
2026-10-12: per group, mean absolute pixel distance of at most 4/255 and at most 2% of pixels
differing by more than 16/255 (8/255 and 5% for the text group), a one-pixel export size
difference tolerated and compared over the common area. Floors and ceilings only rise or
tighten; the rules for changing a denominator are in AGENTS.md section 7.

Python 3.11 or newer is required for the full test suite. The Rust package-comparison
tests print a skip reason when no Python 3 executable is available; setting
`MISTERDIAGRAM_REQUIRE_PYTHON=1` makes absence a failure. `cargo xtask ci` and
GitHub Actions set it, so these comparisons cannot silently skip in CI.
SVG and PDF validation needs a local `.venv` with `pypdf` installed.

## Fonts

Carlito, Caladea, Arimo, Tinos and Cousine are embedded (SIL Open Font License 1.1) and are
enough for a public checkout. `.cargo/config.toml` points at the optional sibling checkout
`../craft-apps/craft-fonts/src`; set `CRAFT_FONTS_DIR` to use another path. If the directory
is absent the build prints a warning about the optional faces and the embedded faces are
used. Carlito is the last-resort Latin face, ahead of the optional BIZ UDPGothic fallback.
No fonts are downloaded.

## The reference corpus

The corpus is private by design: files Visio writes embed Microsoft's connector master and
artwork, and Microsoft's shipped stencils and templates are redistributable by nobody. None
of it ships with this repository, and `corpus/` is gitignored.

`MISTERDIAGRAM_CORPUS=/absolute/path/to/corpus` overrides the default repository
`corpus/` root for the route tests and `route_sweep` example. Other corpus tests
and the CLI continue to use the repository's `corpus/` directory; this variable
does not disable gates or fetch inputs.

- `corpus/generated/`: drawings made by `tools/oracle/make-corpus.ps1` in a licensed Visio,
  each with Visio's PNG, SVG, PDF, COM dump and Connects table. Pinned by commit and SHA256
  manifest in `xtask/src/corpus_pins.rs`. With a clean checkout of the private companion repo
  at `../diagramcraft-corpus`, `cargo xtask corpus --local` copies and verifies it.
  `--update-manifest` is for an intentional new pin, never for accepting a mismatch.
- `corpus/blank/`: the blank-document dumps (document sheet, page sheet, style sheets) that
  define the no-theme defaults and the from-scratch skeleton.
- `corpus/ms-shipped/` and `corpus/ms-shipped-dumps/`: Visio's own stencils and templates and
  their COM dumps, local only. Dump coverage is reported by `corpus check` and can grow
  between runs; floors that depend on it are restated only by the rule in AGENTS.md.

The scripts in [tools/oracle](../tools/oracle/) regenerate everything with your own Visio;
[the oracle runbook](../tools/oracle/README.md) describes the VM workflow: a Windows VM
with Visio Standard 2021, driven by `run-in-vm.sh`, which runs each PowerShell oracle as a
one-shot interactive scheduled task with a watchdog and converts the UTF-16 outputs.
Visio is launched as a normal process and attached through `GetActiveObject`; the
COM-launched automation instance hangs on `Documents.Open` in the reference VM.

## Private master comparisons

Render a labelled contact sheet with
`cargo run --release -p misterdiagram-cli -- render --masters STENCIL.vssx OUT.png`.
Each master is scaled into its cell before rasterization. The pixel cap applies to the whole
sheet; a failed master is marked and the command returns an error.

For the human gate, `corpus/_state/m2-contact-inputs.txt` lists the stencil paths. Generate
their `corpus/_state/contact-sheets/<stem>-ours.png` files, then run
`cargo run --release -p misterdiagram-corpus --example contact_pairs`. This pairs each master
with its private Visio export from `export-masters.ps1`: ours on the left, Visio on the
right. Neither the reference images nor the paired sheets ship with the public repository.

## Performance

`cargo test --release -p misterdiagram-render --test perf -- --ignored --nocapture` runs the
release measurement on the largest shipped stencil and a 500-shape page. The observations in
`perf/budgets.toml` are measurements, not acceptance limits.

## Planning files

`plan/STATUS.md` (decisions), `plan/execution-plan.md` and `log/devlog.md` are local and
gitignored; agents read them at the start of every session, per AGENTS.md.
