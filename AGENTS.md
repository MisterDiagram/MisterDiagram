# AGENTS.md: rules for every agent and contributor

Diagramcraft is a clean-room, headless Rust Visio core. The user brief takes precedence.

## 1. Asset policy (absolute: no exceptions)

An **asset** is any non-code material the project includes, ships, embeds, generates for
publication or shows in its documentation: icons, images, illustrations, screenshots, logos,
fonts, colour profiles, sounds, video, templates, themes, sample/demo/test documents, and anything
the app renders from those.

### 1.1 Nothing from Microsoft, Adobe, Avid or Autodesk products

- **Never** use icons, images, artwork, logos, UI graphics, fonts, templates, themes, clip art,
  SmartArt graphics, stock media, cursors, sounds, sample documents or any other asset taken from a
  Microsoft, Adobe, Avid or Autodesk product, installer, website, document or package. "Microsoft
  product" includes Visio, Office, Windows and the fonts they install (Aptos, Calibri,
  Cambria, Segoe, Consolas, Wingdings…).
- This holds even when the vendor publishes the material under an open licence, if it is **visual
  design**: Diagramcraft does not embed or reference Adobe's Source Sans/Serif/Han families or Noto
  CJK (Source Han rebranded); `crates/fonts/build.rs` excludes them.
- **Never** recreate, trace, redraw or closely imitate a vendor icon, theme or graphic. Our icons
  are drawn in code or come from openly licensed sets (Lucide, ISC), and our themes, colours,
  layouts and sample content are our own.
- Screenshots of Visio exist only as private, local observation notes under
  `plan/visio/screenshots/` (gitignored). They must **never** be committed, published,
  linked, shown in docs, or traced.
- Never commit Visio-produced files to this repository. Script-generated drawings live only
  in the permanently private diagramcraft-corpus repository. They may embed Microsoft masters.
- Microsoft shipped stencils and templates are observed locally, never shipped or committed.
  Their files and COM dumps stay under gitignored `corpus/ms-shipped*/`.

### 1.2 Only open, provable licences

Every asset must be one of: open source (ISC, MIT, BSD-2/3-Clause, Apache-2.0, OFL-1.1,
Ubuntu-Font-1.0, Bitstream-Vera); Creative Commons (CC0-1.0, CC-BY-4.0, CC-BY-SA-4.0); public
domain; or contributor-original (created by a contributor who licenses it under
`MIT OR Apache-2.0`).

Forbidden: system fonts that are not openly licensed, vendor emoji artwork, stock photos, "free
for personal use" material, screenshots of other people's software, anything whose licence you
cannot prove from its original source.

**Screenshots of Diagramcraft** are contributor-original. Everything visible in them must itself be
allowed: our UI, our/Lucide icons, OFL fonts, and drawings built only from allowed assets.

### 1.3 Every asset is attributed

- **`ATTRIBUTION.md`** (repo root) lists every asset file: committed, fetched at build time
  (craft-fonts) or bundled through a Cargo dependency: with author, source (URL + version or
  commit), licence and use. `cargo xtask assets` fails if a file under `assets/`, `docs/images/`
  or `docs/brand/` has no row.
- **`NOTICE`** keeps the notices the licences require.
- Fonts live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts), never in this
  repo (craftrules `standards/fonts.md`). Diagramcraft embeds its `fonts/latin-manifest.txt` and
  `fonts/manifest.txt` when built with `CRAFT_FONTS_DIR` (local dev: `.cargo/config.toml` points at
  `/Users/charles/Projects/craft-apps/craft-fonts/src`).

### 1.4 Adding or changing an asset

1. Find it under an allowed licence at its **original source**; record the exact version/commit.
2. Confirm the author is not Microsoft, Adobe, Avid or Autodesk (§1.1). Keep the licence text.
3. Add the file, its licence file and an `ATTRIBUTION.md` row; run `cargo xtask assets`.
4. If origin or licence is unclear, **do not add it**.

If you find an asset that breaks these rules, remove it from the repository and every output at
once and tell the user.

---

## 2. Clean-room rules

- Visio is installed on the dev machine and may be **observed** black-box: run it, use its UI
  on synthetic drawings, screenshot windows by id into `plan/visio/screenshots/` (never
  committed). Never disassemble Office or read its implementation. The brief permits copying shipped
  stencils/templates only into local gitignored corpus storage for black-box tests. Never copy
  Microsoft wording beyond feature and menu names.
- File formats come from public standards: Microsoft's public .vsdx reference and ShapeSheet reference, ISO 32000
  (PDF). Geometry is derived from published references and observed synthetic drawings.
- Never copy GPL/AGPL/LGPL/MPL code (LibreOffice, Calligra…). Permissive crates are fine as
  dependencies. Code may be copied (not depended on) from sibling Craft apps.

## 3. Never crash

People trust Diagramcraft with their diagrams; a crash loses work. **This outranks feature work.**
The rules below are enforced by the local workspace gates.
- No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!` in non-test
  code; no `unsafe` (`unsafe_code = "forbid"`). Every production crate root carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- Errors are `Result<T, E>` + `?`. Input-derived numbers (files, commands, MCP/control params) are
  hostile: `get()` not `[i]`, checked arithmetic, no NaN casts, capped allocations, bounded
  recursion.
- `Session::execute` runs every command inside a panic guard (`crates/engine/src/guard.rs`) that
  keeps the drawing as it was. It is a safety net, not a licence.
- Every crash fix lands with a regression test.

## 4. Architecture

| Layer | Crates |
|---|---|
| L0 | geom, color, opc |
| L1 | sheet, model, fonts |
| L2 | text, theme, vsdx |
| L3 | route, render |
| L4 | export |
| L5 | engine |
| L6 | mcp |


Every user-visible action is an engine command with a spec and tests. Programmatic calls never
open dialogs. A future GUI will be a thin client of the same engine. No UI crate is permitted
in L0 through L6. Never break wasm. Geometry algorithms copied from CADCraft remain dormant
until Phase 2; the active geom API provides Vec2, Affine and Inches.

## 5. Quality gates

`cargo xtask ci` runs formatting, Clippy with `-D warnings`, tests, assets, layers and wasm.
It must pass before every commit touching Rust. Commit with `M<phase>.<n>: what`, push to
`origin master`, never force-push or rewrite history.

## 6. Running

- `cargo build --workspace`
- `cargo run -p diagramcraft-cli -- info FILE.vsdx`
- `cargo xtask ci`
- `cargo xtask parity`

Phase 0 implements only `info` in the CLI. Other subcommands return exit 2. MCP is a library
scaffold with list_commands and run_command; authoring commands arrive in Phase 4.
Fonts are read from CRAFT_FONTS_DIR set in `.cargo/config.toml`. Never write to sibling repos.

## 7. Operating rules for the agents

Copied into the repo's `AGENTS.md` §7 so every session reads them.

- **Start each session** with `plan/STATUS.md`, `plan/execution-plan.md`, `log/devlog.md`, then `cargo xtask ci`. End each session with a devlog entry: what landed, the numbers, what is open.
- **Never ask Charles mid-phase.** Decide, record the decision and reason in `plan/STATUS.md`, continue. Charles is consulted at milestone gates and the two human gates (M2 contact sheets and thresholds, M5 PowerPoint check).
- **One task equals one checklist row, one element-checklist item, or one gaps entry.** Each cites the reference page it was built from and lands with a test. Re-score the row, update `gaps.md`, add a progress-log line.
- **When an oracle disagrees with us, we are wrong until proven otherwise.** Never edit an oracle file, never relax a threshold, floor or denominator to pass. If Visio contradicts the reference page, record it in `file-format-parity.md` under "Observed, undocumented" with the corpus file that shows it, and implement Visio's behaviour. Adding a formula to `volatile.txt` requires a corpus file demonstrating the legitimate difference.
- **Never-crash outranks features.** No unwrap, expect, panic, todo, unreachable outside tests; hostile-input tests for every parser; every crash fix ships with the regression test that panicked.
- **Parallel agents:** own `CARGO_TARGET_DIR=target/agent-<name>`, own crates, surgical edits to shared files after re-reading them, manifests edited via a temp file. Commits `M<phase>.<n>: what`, pushed to `master` on srv.
- **Reuse discipline:** copy a file with its licence header, add an `ATTRIBUTION.md` row (repo, path, commit, licence), list it in `docs/architecture.md`. GPL and MPL projects are never opened in an editor during implementation.
- **Milestone closure** follows Charles's review rule: one open discovery pass by a second agent (fresh context, given the acceptance tests and the code, asked to attack), findings tiered blocker/should-fix/nit and folded into the milestone rubric, then verification passes against the frozen rubric until two in a row are clean. A seeded defect for that review is a change to the engine or a parser, never to an oracle, a threshold or a test harness.

## 8. Planning documents

`plan/` and `log/` are local, gitignored session state. Start with STATUS, the execution plan,
and devlog. Copy STATUS and devlog to `corpus/_state/` at the end of every session.

| File | Purpose |
|---|---|
| ROADMAP.md | Stage, phase, progress and measured numbers |
| docs/architecture.md | Layers and copied code provenance |
| docs/parity-checklist.md | D/P/M feature rows, Core Y/N, regenerated score |
| docs/gaps.md | Ranked work list |
| docs/target-app-parity.md | Measured target and parity limitations |
| docs/file-format-parity.md | Measurements, undocumented observations, missing functions |
| docs/mcp.md | Protocol and tool surface |

After a task, update the relevant row, gaps entry and progress log. Never change oracle files,
thresholds, floors, denominators or volatile lists to conceal a failure.
