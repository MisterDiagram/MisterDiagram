# Visio feature checklist

> **Drafted:** 2026-10-11 from the observed app (visio-observed.md) and Microsoft's documentation · **Target:** Microsoft Visio Standard 2021 16.0.20430 (Professional rows marked Pro) · **Status:** Phase 1 reader implemented; authoring, evaluation and rendering remain in progress. Format follows the storytold apps' `docs/parity-checklist.md`: area, feature, priority, Core Y/N, status D/P/M.

<!-- SUMMARY -->
**Weighted breadth parity: 23%** over 45 Core=Y features.

| Priority | Features | Done | Partial | Missing | Parity |
|---|---|---|---|---|---|
| P0 | 27 | 1 | 15 | 11 | 31% |
| P1 | 15 | 0 | 1 | 14 | 3% |
| P2 | 3 | 0 | 0 | 3 | 0% |

| Area | Parity |
|---|---|
| Files | 32% |
| Pages | 25% |
| Shapes | 17% |
| Stencils | 43% |
| Connectors | 10% |
| Text | 43% |
| Format | 38% |
| Arrange | 0% |
| Layers | 50% |
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
| Files | Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links) | P0 | Y | P |
| Files | Export DWG/DXF; import DWG/DXF as a CAD underlay or converted shapes | P1 | Y | M |
| Files | Export to PowerPoint (one page per slide, native shapes) and Word | P1 | Y | M |
| Files | Print: page setup, fit to pages, headers and footers | P1 | N | M |
| Pages | Multiple pages, background pages, page size and orientation, drawing scale and units | P0 | Y | P |
| Pages | Rulers, grid, dynamic grid, guides, snap and glue settings | P0 | N | M |
| Pages | Page tabs: add, rename, reorder, duplicate, delete | P0 | Y | M |
| Shapes | Draw tools: rectangle, ellipse, line, freeform, arc, pencil | P0 | Y | M |
| Shapes | Drop masters from a stencil; master instances inherit and can be overridden | P0 | Y | P |
| Shapes | Selection, move, resize, rotate, flip, duplicate, group and ungroup | P0 | Y | M |
| Shapes | Shape text with text block control handle; double-click to edit | P0 | Y | P |
| Shapes | Shape data (custom properties) and the Shape Data window | P0 | Y | M |
| Shapes | Hyperlinks on shapes | P1 | Y | M |
| Shapes | Control handles, connection points (add, move, type in/out/both) | P0 | Y | M |
| Shapes | Geometry sections with formulas (Width*0.5 style), guarded cells | P0 | Y | P |
| Shapes | Boolean operations: union, combine, fragment, intersect, subtract | P1 | Y | M |
| Shapes | Protection (lock position, size, aspect, text, delete) | P1 | Y | M |
| Stencils | ShapeSheet model: Object, Geometry, Character, Paragraph, Tabs, Controls, Connection, User, Actions, Scratch, gradient stops | P0 | Y | D |
| Stencils | Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation | P0 | Y | P |
| Stencils | New master from a shape; edit master; document stencil | P0 | Y | M |
| Stencils | Read vendor stencil packs (.vssx) and render their masters correctly | P0 | Y | P |
| Stencils | Master icons, master names and prompts, search index | P1 | Y | M |
| Connectors | Dynamic connector: glue to connection points and to shapes (dynamic glue), _WALKGLUE behaviour | P0 | Y | M |
| Connectors | Orthogonal routing with reroute on move; straight and curved styles | P0 | Y | M |
| Connectors | Line jumps (style, code, size) and routing preferences per page | P1 | Y | M |
| Connectors | Connector text that stays with the line; begin and end arrows; line ends per theme | P0 | Y | P |
| Connectors | AutoConnect arrows and quick shapes | P1 | Y | M |
| Connectors | Connect Shapes command, Re-Layout Page (flowchart, hierarchy, radial, circular) | P1 | Y | M |
| Text | Character and paragraph formatting, tabs, bullets, fields (date, page, shape data) | P0 | Y | P |
| Text | Text block rotation, margins, vertical alignment, text resize behaviour | P0 | Y | P |
| Text | Spelling, find and replace across pages | P2 | Y | M |
| Format | Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane | P0 | Y | P |
| Format | Themes, variants, theme colours and effects; THEMEVAL cells follow them | P0 | Y | P |
| Format | Format Painter, Quick Styles | P1 | Y | M |
| Arrange | Align, distribute, same size, bring to front/back, position pane | P0 | Y | M |
| Arrange | Auto Align & Space, Auto Size page | P1 | Y | M |
| Layers | Layers per page: visible, print, lock, snap, glue, colour; layer membership | P0 | Y | P |
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

## M2.2 paint evidence (historical)

- Format and connector-arrow rows remain M until an output backend and render comparisons exist. `diagramcraft-render::style::tests::measured_line_families_and_fill_density` checks the observed dash families and procedural fill coverage; `all_patterns_and_arrow_sizes_are_finite_and_bounded` checks all identifiers and sizes.

## M2.3 theme evidence

- Themes and variants: P, `diagramcraft-theme::resolve::tests::microsoft_themeval_examples_and_variant_selectors`. Package color/font/format tables, variant selectors and eight theme functions are evaluated. Corpus rates include theme cells; final rendered fidelity and full effect coverage remain open.

## M2.4 text evidence

- Character and paragraph formatting remains partial: `diagramcraft-vsdx::evaluate::text::tests::markers_fields_and_hostile_text` and `diagramcraft-text::tests::finite_wrapping_bidi_and_autofit`. Nonvolatile string/numeric field expansion is tested by `fields_expand_current_values_and_format_pictures`; aligned tabs and font-size spacing by `visio_font_percentage_empty_line_and_aligned_tabs`. Date-host fields retain their saved display.
- Text block control remains partial: `diagramcraft-vsdx::evaluate::text::tests::generated_text_group_resolves_runs_and_transforms` and `diagramcraft-text::tests::finite_wrapping_bidi_and_autofit`. `probe_connector_empty_text_retains_minimum_line_height` fixes the original empty-label discrepancy. Pixel fidelity and full text metrics have not passed.

## M2.5 output evidence

- PNG, SVG and PDF export: P, `diagramcraft-render::generated_pages_and_foreign_objects_produce_outputs` produces all three formats for all 204 generated drawings. PDF uses vector paths and embedded glyph text; visual and extraction acceptance follow in M2.6. JPEG/EMF export and links remain open.
- Pages and backgrounds: P, `diagramcraft-render::layers_visibility_print_and_background_pages` checks background inclusion and rejects a background cycle. Page size and drawing scale feed the scene; pixel fidelity is not yet accepted.
- Layers: P, `diagramcraft-render::layers_visibility_print_and_background_pages` distinguishes hidden screen layers from printable layers. Editing, layer colours, locking, snapping and glue remain open.
- Geometry and master inheritance remain P, with the generated output test exercising the evaluated path adapter. Shape authoring rows remain M.

- M2.6 partial evidence, Shape text with text block control handle; double-click to edit: P, `generated_text_group_resolves_runs_and_transforms`. Rendering existing text only; control-handle editing remains absent.

- M2.6 partial evidence, Read vendor stencil packs (.vssx) and render their masters correctly: P, `largest_stencil_and_500_shape_page`. Contact sheets render a subset of masters and mark failures; full master correctness fails.

- M2.6 partial evidence, Connector text that stays with the line; begin and end arrows; line ends per theme: P, `generated_pages_and_foreign_objects_produce_outputs`. Existing connector geometry and markers render; arrow bounds and pixel fidelity fail.

- M2.6 partial evidence, Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane: P, `generated_svg_resvg_and_pdf_text`. Paint backends work with procedural fills and strokes; compound offsets, shadows and effects remain incomplete.

- P2-followup-2 item1, Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links): P: 129 arrow/line dimensions match; ten text-dependent sizes remain wrong. Tests: `arrow_and_cap_export_dimensions_match_all_generated_witnesses`.

- P2-followup-2 item2, Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane: P: both observed edge profiles match, but other pixel groups still fail. Tests: `observed_probe_and_hairline_edge_profiles`.

- P2-followup-2 item3, Themes, variants, theme colours and effects; THEMEVAL cells follow them: P: correct the extension table index; incomplete cached theme fidelity remains. Tests: `basic_drop_theme_extension_uses_its_zero_style_slot`.

- P2-followup-2 item4, Read vendor stencil packs (.vssx) and render their masters correctly: P: 1100 of1108 contact masters render, with eight documented errors. Tests: `contact_cells_fit_large_masters_and_skip_empty_text_blocks`.

- P2-followup-2 item5, Multiple pages, background pages, page size and orientation, drawing scale and units: P: select foreground and composite background; apply physical scale. Metric text extent still fails. Tests: `layers_visibility_print_and_background_pages; scaled_page_dimensions_are_physical_inches`.

- P2-followup-2 item6, Character and paragraph formatting, tabs, bullets, fields (date, page, shape data): P: correct baseline and nonempty height slack; exact wrapping and font-width metrics remain incomplete. Tests: `percentage_line_baseline_matches_generated_alignment_svg; composed_text_height_slack_matches_visio_and_preserves_empty_blocks`.

- P2-followup-2 text and master follow-on evidence: P remains. `text_metrics_return_scaled_drawing_units`, `empty_flowchart_text_does_not_recurse_through_wrapping_width`, and `repeated_knots_accept_serialization_noise_but_reject_descending_values` recover unit conversion and additional master errors. Contacts1107/1108; frozen render gate still fails.

- P2-followup-3 text placement remains P: `text_blocks_and_unkerned_lines_match_visio_svg` checks16block rectangles and8wrapped line starts/baselines. Logical PNG text bounds remain open.
- P2-followup-3 arrows remain P: `svg_marker_scales_aliases_and_shaft_setbacks` and `arrow_and_cap_export_dimensions_match_all_generated_witnesses` check SVG geometry and129PNG dimensions. Pixel acceptance remains separate.

- M2.6.7 group/stroke rendering prerequisite: `transformed_strokes_preserve_physical_pen_and_fit_export_viewport` checks physical pen widths and both export edge profiles. Authoring scores remain unchanged.

- M2.6.8 image rendering remains P: `image_filter_uses_premultiplied_alpha_and_transparent_border`, `destination_only_raster_operation_is_a_noop`, and `generated_svg_resvg_and_pdf_text`. Image group passes; EMF+ remains missing.

- P2-followup-3 M2.7.2, vendor master rendering remains P pending pixel parity. `clamped_cycles_are_proven_without_cached_formula_values` and `empty_flowchart_text_does_not_recurse_through_wrapping_width` resolve BASFLO_U Custom1 from interval bounds, matching four freshly observed COM values without cached formula fallback.

- P2-followup-3 M2.4.3, text fidelity remains P until both text oracles promote. Tests: `microsoft_find_example_and_hostile_arguments`, `metric_font_size_grid_matches_com_height_witnesses`, `text_metric_grid_and_empty_fields_match_private_com_witnesses`. Fine metric size rounding and empty fields now match independent COM witnesses; FIND has its reference example and bounded-input tests.

- M2.8.0, public XML resource boundary: `depth_boundary_is_independent_of_empty_element_syntax` and `public_xml_parser_enforces_depth_for_both_empty_element_forms` accept depth 256 and reject depth 257 for both empty-element forms. Reader feature scores stay unchanged; no private-corpus fidelity measurement was made.
- M2.4.4 composed width evidence: `diagramcraft-text::tests::composed_width_terminal_space_and_advance_grid` reproduces twenty private cached witnesses. Text remains P until the independent pixel and metric gates pass.
