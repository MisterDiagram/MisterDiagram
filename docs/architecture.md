# Architecture

Page coordinates are inches, y-up, origin bottom-left. Dependencies go to strictly lower layers. Apps are exempt. UI crates are forbidden in L0 to L6.

| Layer | Crates | Responsibility |
|---|---|---|
| L0 | geom, color, opc | Geometry rows to kurbo paths in y-up inches, colour and package primitives. |
| L1 | sheet, model, fonts | sheet: formula syntax, values, units, functions and host traits; model: document structures; fonts: shaping database. |
| L2 | text, theme, vsdx | vsdx: package reading/writing, document evaluator, inheritance, dependency tracking and dirty recalc; text/theme: model-based host implementations. |
| L3 | route, render | Routing and rendered outputs. |
| L4 | export | External format export. |
| L5 | engine | Commands and session state. |
| L6 | mcp | Protocol adapter. |

The L2 document evaluator supplies adapters to theme and text hosts using L1 traits and
model inputs. Theme and text do not depend on vsdx; production L2 sibling dependencies
remain forbidden. The named P3 `vsdx` integration test injects real theme/text hosts
through explicit dev-only dependency exceptions; production libraries remain independent.

The writer receives optional theme/text services through those same traits. It serializes
local overrides and evaluates caches through an isolated snapshot, retaining unsupported
cached values with warnings. It validates caller-built models before cloning, applies
reader sheet limits after XML escaping, and preserves opaque part payloads exactly.

## Copied from siblings

| Local path | Source repo | Source path | Commit | Licence |
|---|---|---|---|---|
| `crates/vsdx/tests/reader_memory.rs`, `crates/vsdx/tests/support/hostile_package.rs` | craft-apps, Charles Hood | `scope/visio/review/repros/1a/vsdx_review_1a.rs` (selected memory tests and synthetic package recipe) | `107ee4c3423dd4154bd3c59f60362801f638eabf` | MIT OR Apache-2.0, user-authorized port |
| `crates/vsdx/tests/authored_strings.rs` | craft-apps, Charles Hood | `scope/visio/review/repros/1b/vsdx_review_1b.rs` (selected synthetic test) | `3ce1a4740bf3e44463f1e35babc42ff46562f4e5` | MIT OR Apache-2.0, user-authorized port |
| `crates/engine/tests/route_ownership.rs` | craft-apps, Charles Hood | `scope/visio/review/repros/3a/engine_review_3a.rs` (selected synthetic tests) | `3ce1a4740bf3e44463f1e35babc42ff46562f4e5` | MIT OR Apache-2.0, user-authorized port |
| `crates/engine/tests/endpoint_obstacles.rs` | craft-apps, Charles Hood | `scope/visio/review/repros/3a/engine_review_3a.rs` (selected tests), `scope/visio/review/repros/2b/2b-6-connector-endpoint-in-obstacle.sh` (recipes) | `3ce1a4740bf3e44463f1e35babc42ff46562f4e5` | MIT OR Apache-2.0, user-authorized port |
| `crates/render/tests/flipped_text.rs` | craft-apps, Charles Hood | `scope/visio/review/repros/3b/render_review3b.rs` (selected synthetic test and helpers) | `3ce1a4740bf3e44463f1e35babc42ff46562f4e5` | MIT OR Apache-2.0, user-authorized port |
| `clippy.toml` | deckcraft | `clippy.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `rustfmt.toml` | deckcraft | `rustfmt.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `.cargo/config.toml` | deckcraft | `.cargo/config.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `LICENSE-MIT` | deckcraft | `LICENSE-MIT` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `LICENSE-APACHE` | deckcraft | `LICENSE-APACHE` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `Cargo.toml` | deckcraft | `Cargo.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/color/Cargo.toml` | deckcraft | `crates/color/Cargo.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/color/src/lib.rs` | deckcraft | `crates/color/src/lib.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/Cargo.toml` | deckcraft | `crates/fonts/Cargo.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/build.rs` | deckcraft | `crates/fonts/build.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/src/fontdb.rs` | deckcraft | `crates/fonts/src/fontdb.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/src/lib.rs` | deckcraft | `crates/fonts/src/lib.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/src/fallback.rs` | deckcraft | `crates/fonts/src/fallback.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/src/script.rs` | deckcraft | `crates/fonts/src/script.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/fonts/src/shape.rs` | deckcraft | `crates/fonts/src/shape.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/opc/src/opc.rs` | deckcraft | `crates/pptx/src/opc.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/opc/src/xml.rs` | deckcraft | `crates/pptx/src/xml.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/vsdx/tests/malformed.rs` | deckcraft | `crates/pptx/tests/malformed.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/geom/src/curve.rs` | cadcraft | `crates/geom/src/curve.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/geom/src/nurbs.rs` | cadcraft | `crates/geom/src/nurbs.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/geom/src/intersect.rs` | cadcraft | `crates/geom/src/intersect.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/engine/src/cmd/mod.rs` | deckcraft | `crates/engine/src/cmd/mod.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/engine/src/lib.rs` | deckcraft | `crates/engine/src/lib.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/tools.rs` | deckcraft | `crates/mcp/src/tools.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/engine/src/guard.rs` | deckcraft | `crates/engine/src/guard.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/server.rs` | deckcraft | `crates/mcp/src/server.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/backend.rs` | deckcraft | `crates/mcp/src/backend.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/headless.rs` | deckcraft | `crates/mcp/src/headless.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/Cargo.toml` | deckcraft | `xtask/Cargo.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/parity.rs` | deckcraft | `xtask/src/parity.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/main.rs` | deckcraft | `xtask/src/main.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/layers.rs` | deckcraft | `xtask/src/layers.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/assets.rs` | deckcraft | `xtask/src/assets.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/render/src/raster.rs` | deckcraft | `crates/render/src/lib.rs, crates/render/src/paint.rs, crates/render/src/images.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/render/src/pdf.rs` | deckcraft | `crates/pdf/src/lib.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/render/src/pdf.rs` (helper `glyph_text_ranges` and its test) | [storytold/deckcraft](https://github.com/storytold/deckcraft), MarekSpodBiedry | `crates/pdf/src/lib.rs`, `crates/pdf/src/tests.rs` | `dd404af8b53029e80d1ff9a7ede02619fdc7ab8a` | MIT OR Apache-2.0 |
| `crates/render/src/metafile/mod.rs` | vectorcraft | `crates/metafile/src/import.rs` | `806b1a6972288319577d75d097c140f9bf530ce0` | MIT OR Apache-2.0 |
| `crates/render/src/metafile/emf.rs` | vectorcraft | `crates/metafile/src/import/emf.rs` | `806b1a6972288319577d75d097c140f9bf530ce0` | MIT OR Apache-2.0 |
| `crates/render/src/metafile/bytes.rs` | vectorcraft | `crates/metafile/src/bytes.rs` | `806b1a6972288319577d75d097c140f9bf530ce0` | MIT OR Apache-2.0 |
| `crates/render/src/metafile/dib.rs` | vectorcraft | `crates/metafile/src/dib.rs` | `806b1a6972288319577d75d097c140f9bf530ce0` | MIT OR Apache-2.0 |
| `apps/diagramcraft-corpus/tests/render_fuzz.rs` | deckcraft (via diagramcraft vsdx harness) | `crates/pptx/tests/malformed.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |

Adapted `AGENTS.md` from deckcraft `src/AGENTS.md`, commit `be14248577b92059eb2b8c0c9460cceffbc1f01d`, MIT OR Apache-2.0.

Adapted `CLAUDE.md` from deckcraft `src/CLAUDE.md`, commit `be14248577b92059eb2b8c0c9460cceffbc1f01d`, MIT OR Apache-2.0.

The native `apps/diagramcraft-corpus` crate shares corpus-oracle logic between the CLI and
integration tests. It is an application harness above the core layers, exempt like the CLI;
no core crate depends on it. This avoids an upward dependency from sheet tests to the reader.

## Phase 2 host composition

`sheet::host::ThemeHost` accepts formula values and a bounded reference context. Applications
construct `theme::Themes` from the model's package parts and inject it into `vsdx::Evaluator`.
The document adapter resolves page sentinels, records selector dependencies and supplies
blank/root-style defaults. Theme never depends on vsdx, and vsdx never depends on theme.
The CLI dump and corpus evaluator use the same injected service. No shipped theme assets
are compiled into the resolver.

### Phase 2 text retarget

| Local path | Source repo | Source path | Commit | Licence |
|---|---|---|---|---|
| `crates/model/src/text.rs` | deckcraft | `crates/model/src/text.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/text/src/lib.rs` | deckcraft | `crates/text/src/lib.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/text/src/bidi.rs` | deckcraft | `crates/text/src/bidi.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |

P2-followup embeds the pinned OFL Latin faces in the fonts crate for both native and wasm builds. Optional craft-fonts adds script coverage; system scanning remains off. Text-metric promotion is permanent. Both text lines remain in the main oracle denominators and must pass 90% plus their recorded ratchets; a regression cannot de-promote them. Rendering uses real text evaluation.

The named P3 `model/tests/from_scratch.rs` integration test has a dev-only
model -> vsdx dependency to assert serialization and re-reading of its five typed
constructions. Production model dependencies remain at L0/L1.

| `crates/engine/tests/session.rs` (hostile parameter loop) | deckcraft | `crates/engine/src/tests.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |

The Phase 4 session owns an Arc<Document>, a page/shape selection, bounded snapshot
history and a bounded JSON command journal. Session::edit restores all session
state on error or panic. Dirty edits serialize through the dependency-aware VSDX
writer and re-import before publication, reusing the established cache contract.
File outputs publish from a sibling temporary file after successful serialization.
Exports share a retained-scene budget across PDF pages; EMF operations are charged
while records are interpreted, before the enclosing scene retains the result.

`stencil.open` retains at most 128 MiB of uncompressed source-part bytes across
all open stencils in a session, with the existing 64-stencil count limit.
Reopening the same canonical path reuses its source. A source-byte refusal names
the stencil, its required MiB, the MiB already open and the 128 MiB limit.

Brief B reader accounting (2026-10-16): scanned all 1,413 strict-corpus packages
and all 844 additional stencil paths under corpus (2,257 paths, 26,497 distinct
XML payloads, zero parse errors). The largest XML part has 467,889 retained
elements, attributes and coalesced text nodes: statistical_infographic_m.vssx,
visio/masters/master10.xml (155,798 elements and 312,091 attributes). The reader's
node budget now counts all three kinds before retaining them, at 5,000,000,
more than ten times that maximum. This replaces the old 4,000,000-element-only
budget; it bounds retained storage, not traversal time. The largest attribute
count on one element is 44, in PRSSTS_M.vstx, visio/data/recordset2.xml.
No per-element attribute cap is introduced.

The XML parser builds the namespace-aware retained tree directly. Small XML
attribute tables use hash maps instead of minimally eleven-slot B-tree leaves;
unqualified attributes need no stored namespace entry. XML emission explicitly
sorts attributes, preserving deterministic output. Typed sheets retain their
ordered maps. Completed part trees and decompressed buffers move into the model,
so the reader retains one source buffer and one XML view per part.
The stencil budget continues to count source bytes at its unchanged 128 MiB:
the six dense-stencil repro opens below 1 GiB after the DOM reduction, so another
cap is unnecessary. Largest uncompressed package input in the scan: 33,815,410
bytes, SOCIAL_DISTANCING_M.vstx. No existing source-byte or sheet cap changes.




Phase 4 glue is a route-owned HostHooks implementation injected into the VSDX
evaluator. The sheet crate stays model-independent. Session edits compare target
transforms, free endpoints, walking preferences and parent frames with their
snapshot, invalidate glue triggers, evaluate endpoints and regenerate straight
Geometry rows before publishing. Existing imported endpoint caches survive
unrelated edits. Grouped connector endpoints are in parent coordinates, as the
BeginX/BeginY contract requires. The CLI command registry and generator scripts
use this same session path.

Attribute parsing disables quick-xml's quadratic duplicate scan and checks raw qualified names in a hash set before retaining values. Duplicates remain errors. No new cap is needed; the corpus maximum remains 44 attributes on one element.

Copied from siblings: `crates/opc/tests/attribute_bounds.rs` adapts the review 1a attribute regression from craft-apps at `107ee4c3423dd4154bd3c59f60362801f638eabf`, MIT OR Apache-2.0.

Brief B relationship accounting: each relationship part is parsed once into an
ID index, then its entries move into the retained part. Package validation uses
the same parsed root relationships and document tree. Repeated pages/masters
collection targets and repeated Page/Master content targets are read once;
the first occurrence supplies the model entry. Empty entries without a Rel
remain distinct because each is represented explicitly in the input. This keeps
work proportional to input parts and elements without a new page/master cap.
The scan of all 1,413 strict packages plus 850 other stencil paths (including six
B1 synthetic probes) found zero duplicate collection or content targets, zero
parse errors, at most 3 pages and 292 masters in a collection.

Copied from siblings: `crates/vsdx/src/relationship_tests.rs` adapts the review 1a
relationship regressions and synthetic package recipe from craft-apps at
`107ee4c3423dd4154bd3c59f60362801f638eabf`, MIT OR Apache-2.0.

Brief B EMF work accounting: the existing 100,000-operation limit now checks
prospective text characters before UTF-16 decoding and shaping, and dash pieces
before kurbo constructs a stroked outline. The dash estimate uses a control-polygon
upper bound on path length, the pattern period and the number of path elements.
No numeric ceiling changes. The scan covered all 1,413 strict packages and all
850 additional stencil paths, finding 2,661 EMF occurrences, 979 unique payloads,
and no import errors. Largest text record: 34 UTF-16 units in generated/text/margins.vsdx,
docProps/thumbnail.emf. Largest prospective dash expansion: 45 pieces in
CISCONETWORKSHAPES_M.VSSX, visio/media/image276.emf. Both are less than one tenth
of the unchanged operation limit. Largest retained EMF output was 19,816 operations;
that existing aggregate ceiling is unchanged. The two hostile repros are refused
with the metafile operation-limit error before expansion. Tests lower the player
budget to32 and assert no outlined operations were retained before refusal.

Copied from siblings: `crates/render/src/metafile/emf_bounds_tests.rs` adapts the
review 3b synthetic EMF records and regressions from craft-apps at
`107ee4c3423dd4154bd3c59f60362801f638eabf`, MIT OR Apache-2.0.

Brief B EVALCELL caching: each cached value records whether it can be shared
with argument frames. A monotonic context counter marks every enclosing
cell evaluation that consulted an argument, including default/missing arguments,
or encountered a transient depth/work refusal or unresolved cycle. Unchanged
counter and revision permit a shared entry. Context-dependent values retain
their historical cache outside frames, but frames ignore those entries and
never write a context-dependent result. An outside cache hit propagates its
context flag to callers. Metadata lives with each value, so invalidation and
cache clearing cannot leave stale flags. Bounded clamped-cycle proof also runs
inside frames; its restricted grammar does not speculatively evaluate ARG or
EVALCELL. Counter saturation disables new shared entries. No cap changes or
additional dependency walk are needed.

Copied from siblings: `crates/vsdx/tests/evalcell_cache.rs` adapts the review 2a
doubling-chain regression from craft-apps at
`107ee4c3423dd4154bd3c59f60362801f638eabf`, MIT OR Apache-2.0.
