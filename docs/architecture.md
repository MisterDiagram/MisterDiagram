# Architecture

Page coordinates are inches, y-up, origin bottom-left. Dependencies go to strictly lower layers. Apps are exempt. UI crates are forbidden in L0 to L6.

| Layer | Crates |
|---|---|
| L0 | geom, color, opc |
| L1 | sheet, model, fonts |
| L2 | text, theme, vsdx |
| L3 | route, render |
| L4 | export |
| L5 | engine |
| L6 | mcp |

## Copied from siblings

| Local path | Source repo | Source path | Commit | Licence |
|---|---|---|---|---|
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
| `crates/geom/src/curve.rs` | cadcraft | `crates/geom/src/curve.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/geom/src/nurbs.rs` | cadcraft | `crates/geom/src/nurbs.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/geom/src/intersect.rs` | cadcraft | `crates/geom/src/intersect.rs` | `b6383b5f3c107bae23c327a30bb6d04f89d27e50` | MIT OR Apache-2.0 |
| `crates/engine/src/cmd/mod.rs` | deckcraft | `crates/engine/src/cmd/mod.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/engine/src/guard.rs` | deckcraft | `crates/engine/src/guard.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/server.rs` | deckcraft | `crates/mcp/src/server.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/backend.rs` | deckcraft | `crates/mcp/src/backend.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `crates/mcp/src/headless.rs` | deckcraft | `crates/mcp/src/headless.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/Cargo.toml` | deckcraft | `xtask/Cargo.toml` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/parity.rs` | deckcraft | `xtask/src/parity.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/main.rs` | deckcraft | `xtask/src/main.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/layers.rs` | deckcraft | `xtask/src/layers.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |
| `xtask/src/assets.rs` | deckcraft | `xtask/src/assets.rs` | `be14248577b92059eb2b8c0c9460cceffbc1f01d` | MIT OR Apache-2.0 |

Adapted `AGENTS.md` from deckcraft `src/AGENTS.md`, commit `be14248577b92059eb2b8c0c9460cceffbc1f01d`, MIT OR Apache-2.0.

Adapted `CLAUDE.md` from deckcraft `src/CLAUDE.md`, commit `be14248577b92059eb2b8c0c9460cceffbc1f01d`, MIT OR Apache-2.0.
