# Visio feature checklist

> **Drafted:** 2026-10-11 from the observed app (visio-observed.md) and Microsoft's documentation · **Target:** Microsoft Visio Standard 2021 16.0.20430 (Professional rows marked Pro) · **Status:** Phase 1 reader implemented; authoring, evaluation and rendering remain in progress. Format follows the storytold apps' `docs/parity-checklist.md`: area, feature, priority, Core Y/N, status D/P/M.

<!-- SUMMARY -->
**Weighted breadth parity: 10%** over 45 Core=Y features.

| Priority | Features | Done | Partial | Missing | Parity |
|---|---|---|---|---|---|
| P0 | 27 | 1 | 5 | 21 | 13% |
| P1 | 15 | 0 | 1 | 14 | 3% |
| P2 | 3 | 0 | 0 | 3 | 0% |

| Area | Parity |
|---|---|
| Files | 24% |
| Pages | 0% |
| Shapes | 11% |
| Stencils | 32% |
| Connectors | 0% |
| Text | 0% |
| Format | 0% |
| Arrange | 0% |
| Layers | 0% |
| Containers | 0% |
| Data | 0% |
| Automation | 0% |

Open P0 items: Open and save .vsdx (full fidelity, unknown parts kept), Open .vssx stencils and .vstx templates; save a stencil, Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links), Multiple pages, background pages, page size and orientation, drawing scale and units, Page tabs: add, rename, reorder, duplicate, delete, Draw tools: rectangle, ellipse, line, freeform, arc, pencil, Drop masters from a stencil; master instances inherit and can be overridden, Selection, move, resize, rotate, flip, duplicate, group and ungroup, Shape text with text block control handle; double-click to edit, Shape data (custom properties) and the Shape Data window, Control handles, connection points (add, move, type in/out/both), Geometry sections with formulas (Width*0.5 style), guarded cells, Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation, New master from a shape; edit master; document stencil, Read vendor stencil packs (.vssx) and render their masters correctly, Dynamic connector: glue to connection points and to shapes (dynamic glue), _WALKGLUE behaviour, Orthogonal routing with reroute on move; straight and curved styles, Connector text that stays with the line; begin and end arrows; line ends per theme, Character and paragraph formatting, tabs, bullets, fields (date, page, shape data), Text block rotation, margins, vertical alignment, text resize behaviour, Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane, Themes, variants, theme colours and effects; THEMEVAL cells follow them, Align, distribute, same size, bring to front/back, position pane, Layers per page: visible, print, lock, snap, glue, colour; layer membership, Every command on a CLI, a JSON control channel and MCP; headless render to PNG/SVG/PDF, Build a diagram from a script: drop, glue, route, lay out, export.

Core=N rows (excluded from milestone scoring):

| Priority | Features | Done | Partial | Missing |
|---|---|---|---|---|
| P0 | 6 | 0 | 0 | 6 |
| P1 | 3 | 0 | 0 | 3 |
| P2 | 2 | 0 | 0 | 2 |

## Rows

| Area | Feature | Priority | Core | Status |
| --- | --- | --- | --- | --- |
| Application shell | Start screen (templates by category, recent, blank drawing) | P0 | N | M |
| Application shell | Ribbon: Home, Insert, Draw, Design, Data, Process, Review, View, Developer | P0 | N | M |
| Application shell | Shapes window: open stencils, search shapes, quick shapes, favorites | P0 | N | M |
| Application shell | ShapeSheet window (sections, cells, formulas, live edit) | P1 | N | M |
| Application shell | Command palette / Tell me | P1 | N | M |
| Files | Open and save .vsdx (full fidelity, unknown parts kept) | P0 | Y | P |
| Files | Open .vssx stencils and .vstx templates; save a stencil | P0 | Y | P |
| Files | Open legacy .vsd (binary) and .vdx (XML 2003) | P1 | Y | M |
| Files | Open .vsdm/.vssm/.vstm (drop macros, keep the rest) | P1 | Y | P |
| Files | Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links) | P0 | Y | M |
| Files | Export DWG/DXF; import DWG/DXF as a CAD underlay or converted shapes | P1 | Y | M |
| Files | Export to PowerPoint (one page per slide, native shapes) and Word | P1 | Y | M |
| Files | Print: page setup, fit to pages, headers and footers | P1 | N | M |
| Pages | Multiple pages, background pages, page size and orientation, drawing scale and units | P0 | Y | M |
| Pages | Rulers, grid, dynamic grid, guides, snap and glue settings | P0 | N | M |
| Pages | Page tabs: add, rename, reorder, duplicate, delete | P0 | Y | M |
| Shapes | Draw tools: rectangle, ellipse, line, freeform, arc, pencil | P0 | Y | M |
| Shapes | Drop masters from a stencil; master instances inherit and can be overridden | P0 | Y | P |
| Shapes | Selection, move, resize, rotate, flip, duplicate, group and ungroup | P0 | Y | M |
| Shapes | Shape text with text block control handle; double-click to edit | P0 | Y | M |
| Shapes | Shape data (custom properties) and the Shape Data window | P0 | Y | M |
| Shapes | Hyperlinks on shapes | P1 | Y | M |
| Shapes | Control handles, connection points (add, move, type in/out/both) | P0 | Y | M |
| Shapes | Geometry sections with formulas (Width*0.5 style), guarded cells | P0 | Y | P |
| Shapes | Boolean operations: union, combine, fragment, intersect, subtract | P1 | Y | M |
| Shapes | Protection (lock position, size, aspect, text, delete) | P1 | Y | M |
| Stencils | ShapeSheet model: Object, Geometry, Character, Paragraph, Tabs, Controls, Connection, User, Actions, Scratch, gradient stops | P0 | Y | D |
| Stencils | Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation | P0 | Y | P |
| Stencils | New master from a shape; edit master; document stencil | P0 | Y | M |
| Stencils | Read vendor stencil packs (.vssx) and render their masters correctly | P0 | Y | M |
| Stencils | Master icons, master names and prompts, search index | P1 | Y | M |
| Connectors | Dynamic connector: glue to connection points and to shapes (dynamic glue), _WALKGLUE behaviour | P0 | Y | M |
| Connectors | Orthogonal routing with reroute on move; straight and curved styles | P0 | Y | M |
| Connectors | Line jumps (style, code, size) and routing preferences per page | P1 | Y | M |
| Connectors | Connector text that stays with the line; begin and end arrows; line ends per theme | P0 | Y | M |
| Connectors | AutoConnect arrows and quick shapes | P1 | Y | M |
| Connectors | Connect Shapes command, Re-Layout Page (flowchart, hierarchy, radial, circular) | P1 | Y | M |
| Text | Character and paragraph formatting, tabs, bullets, fields (date, page, shape data) | P0 | Y | M |
| Text | Text block rotation, margins, vertical alignment, text resize behaviour | P0 | Y | M |
| Text | Spelling, find and replace across pages | P2 | Y | M |
| Format | Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane | P0 | Y | M |
| Format | Themes, variants, theme colours and effects; THEMEVAL cells follow them | P0 | Y | M |
| Format | Format Painter, Quick Styles | P1 | Y | M |
| Arrange | Align, distribute, same size, bring to front/back, position pane | P0 | Y | M |
| Arrange | Auto Align & Space, Auto Size page | P1 | Y | M |
| Layers | Layers per page: visible, print, lock, snap, glue, colour; layer membership | P0 | Y | M |
| Containers | Containers (own their members, resize, headings), lists, callouts | P1 | Y | M |
| Data | Shape data sets, Define Shape Data dialog | P1 | Y | M |
| Data | Link data to shapes from Excel, data graphics, refresh (Pro) | P2 | Y | M |
| Data | Data Visualizer and Org Chart wizard from a table (Pro) | P2 | Y | M |
| Review | Comments, ink, compare drawings | P2 | N | M |
| Review | Validation rules and the Issues window (Pro) | P2 | N | M |
| View | Zoom, pan window, full screen, presentation mode, task panes | P0 | N | M |
| Automation | Every command on a CLI, a JSON control channel and MCP; headless render to PNG/SVG/PDF | P0 | Y | M |
| Automation | Build a diagram from a script: drop, glue, route, lay out, export | P0 | Y | M |
| Platforms | macOS, Windows, Linux, web from one workflow | P0 | N | M |

## Phase 1 evidence

- Open and save .vsdx (full fidelity, unknown parts kept): P, `diagramcraft-vsdx::reads_shape_cells_sections_text_and_unknown_content`.
- Open .vssx stencils and .vstx templates; save a stencil: P, `diagramcraft-vsdx::all_corpus_parses_and_seeded_mutations_never_panic`.
- Open .vsdm/.vssm/.vstm (drop macros, keep the rest): P, `diagramcraft-vsdx::macro_payload_removed_and_flagged`.
- ShapeSheet model: Object, Geometry, Character, Paragraph, Tabs, Controls, Connection, User, Actions, Scratch, gradient stops: D, `diagramcraft-vsdx::reads_shape_cells_sections_text_and_unknown_content`.

- Drop masters from a stencil; master instances inherit and can be overridden: P, `diagramcraft-vsdx::evaluate::inherit::tests::corpus_basic_rect_and_probe_inheritance_witnesses`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

- Geometry sections with formulas (Width*0.5 style), guarded cells: P, `diagramcraft-vsdx::evaluate::path::tests::path_sampling_and_nearest_use_inherited_rows_and_partial_overrides`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

- Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation: P, `diagramcraft-vsdx::evaluate::tests::dirty_recalc_orders_dependencies_and_retains_unrelated_cache`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

## M2.0 evidence

- ShapeSheet model remains D: `diagramcraft-vsdx::dump::tests::basic_rectangle_corpus_has_every_com_identity_and_unit` checks all basic-rectangle COM identities and units, including Connection and Actions rows.
- Formula engine remains P: `diagramcraft-corpus::tests::strict_denominator_classes_and_empty_serialized_string` protects the corrected cached-oracle denominator, formula classification and empty-string cache comparison. Theme and text hosts remain open.

## M2.1 geometry evidence

- Geometry sections with formulas remain P: `diagramcraft-vsdx::evaluated_master_and_group_paths_use_inherited_rows` binds evaluated inherited rows to the L0 converter; `every_geometry_row_has_a_real_corpus_witness` covers all fifteen kinds, including RelEllipticalArcTo. Authoring remains Phase 4.
- Group transforms remain an implemented rendering prerequisite, without scoring move/resize/edit commands: `diagramcraft-geom::rows::tests::infinite_line_clips_and_group_composition_stays_y_up`.

## M2.2 paint evidence

- Format and connector-arrow rows remain M until an output backend and render comparisons exist. `diagramcraft-render::style::tests::measured_line_families_and_fill_density` checks the observed dash families and procedural fill coverage; `all_patterns_and_arrow_sizes_are_finite_and_bounded` checks all identifiers and sizes.
