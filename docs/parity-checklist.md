# Visio feature checklist

> **Drafted:** 2026-10-11 from the observed app (visio-observed.md) and Microsoft's documentation · **Target:** Microsoft Visio Standard 2021 16.0.20430 (Professional rows marked Pro) · **Status:** M4 authoring milestone met under the dated closure-2 rulings; remaining evaluation, routing and rendering gaps are recorded. Format follows the storytold apps' `docs/parity-checklist.md`: area, feature, priority, Core Y/N, status D/P/M.

<!-- SUMMARY -->
**Weighted breadth parity: 51%** over 45 Core=Y features.

| Priority | Features | Done | Partial | Missing | Parity |
|---|---|---|---|---|---|
| P0 | 27 | 8 | 19 | 0 | 65% |
| P1 | 15 | 1 | 4 | 10 | 20% |
| P2 | 3 | 0 | 0 | 3 | 0% |

| Area | Parity |
|---|---|
| Files | 50% |
| Pages | 50% |
| Shapes | 63% |
| Stencils | 61% |
| Connectors | 43% |
| Text | 43% |
| Format | 38% |
| Arrange | 60% |
| Layers | 50% |
| Containers | 0% |
| Data | 0% |
| Automation | 75% |

Open P0 items: Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links), Multiple pages, background pages, page size and orientation, drawing scale and units, Page tabs: add, rename, reorder, duplicate, delete, Draw tools: rectangle, ellipse, line, freeform, arc, pencil, Shape text with text block control handle; double-click to edit, Control handles, connection points (add, move, type in/out/both), Geometry sections with formulas (Width*0.5 style), guarded cells, Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation, New master from a shape; edit master; document stencil, Read vendor stencil packs (.vssx) and render their masters correctly, Dynamic connector: glue to connection points and to shapes (dynamic glue), _WALKGLUE behaviour, Orthogonal routing with reroute on move; straight and curved styles, Connector text that stays with the line; begin and end arrows; line ends per theme, Character and paragraph formatting, tabs, bullets, fields (date, page, shape data), Text block rotation, margins, vertical alignment, text resize behaviour, Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane, Themes, variants, theme colours and effects; THEMEVAL cells follow them, Layers per page: visible, print, lock, snap, glue, colour; layer membership, Build a diagram from a script: drop, glue, route, lay out, export.

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
| Files | Open and save .vsdx (full fidelity, unknown parts kept) | P0 | Y | D |
| Files | Open .vssx stencils and .vstx templates; save a stencil | P0 | Y | D |
| Files | Open legacy .vsd (binary) and .vdx (XML 2003) | P1 | Y | M |
| Files | Open .vsdm/.vssm/.vstm (drop macros, keep the rest) | P1 | Y | P |
| Files | Export SVG (with Visio extension namespace), PNG, JPEG, EMF/WMF, PDF (vector, text, links) | P0 | Y | P |
| Files | Export DWG/DXF; import DWG/DXF as a CAD underlay or converted shapes | P1 | Y | M |
| Files | Export to PowerPoint (one page per slide, native shapes) and Word | P1 | Y | M |
| Files | Print: page setup, fit to pages, headers and footers | P1 | N | M |
| Pages | Multiple pages, background pages, page size and orientation, drawing scale and units | P0 | Y | P |
| Pages | Rulers, grid, dynamic grid, guides, snap and glue settings | P0 | N | M |
| Pages | Page tabs: add, rename, reorder, duplicate, delete | P0 | Y | P |
| Shapes | Draw tools: rectangle, ellipse, line, freeform, arc, pencil | P0 | Y | P |
| Shapes | Drop masters from a stencil; master instances inherit and can be overridden | P0 | Y | D |
| Shapes | Selection, move, resize, rotate, flip, duplicate, group and ungroup | P0 | Y | D |
| Shapes | Shape text with text block control handle; double-click to edit | P0 | Y | P |
| Shapes | Shape data (custom properties) and the Shape Data window | P0 | Y | D |
| Shapes | Hyperlinks on shapes | P1 | Y | D |
| Shapes | Control handles, connection points (add, move, type in/out/both) | P0 | Y | P |
| Shapes | Geometry sections with formulas (Width*0.5 style), guarded cells | P0 | Y | P |
| Shapes | Boolean operations: union, combine, fragment, intersect, subtract | P1 | Y | M |
| Shapes | Protection (lock position, size, aspect, text, delete) | P1 | Y | M |
| Stencils | ShapeSheet model: Object, Geometry, Character, Paragraph, Tabs, Controls, Connection, User, Actions, Scratch, gradient stops | P0 | Y | D |
| Stencils | Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation | P0 | Y | P |
| Stencils | New master from a shape; edit master; document stencil | P0 | Y | P |
| Stencils | Read vendor stencil packs (.vssx) and render their masters correctly | P0 | Y | P |
| Stencils | Master icons, master names and prompts, search index | P1 | Y | P |
| Connectors | Dynamic connector: glue to connection points and to shapes (dynamic glue), _WALKGLUE behaviour | P0 | Y | P |
| Connectors | Orthogonal routing with reroute on move; straight and curved styles | P0 | Y | P |
| Connectors | Line jumps (style, code, size) and routing preferences per page | P1 | Y | P |
| Connectors | Connector text that stays with the line; begin and end arrows; line ends per theme | P0 | Y | P |
| Connectors | AutoConnect arrows and quick shapes | P1 | Y | M |
| Connectors | Connect Shapes command, Re-Layout Page (flowchart, hierarchy, radial, circular) | P1 | Y | P |
| Text | Character and paragraph formatting, tabs, bullets, fields (date, page, shape data) | P0 | Y | P |
| Text | Text block rotation, margins, vertical alignment, text resize behaviour | P0 | Y | P |
| Text | Spelling, find and replace across pages | P2 | Y | M |
| Format | Fill, line, shadow, effects (glow, soft edge, bevel, 3-D) with the Format Shape pane | P0 | Y | P |
| Format | Themes, variants, theme colours and effects; THEMEVAL cells follow them | P0 | Y | P |
| Format | Format Painter, Quick Styles | P1 | Y | M |
| Arrange | Align, distribute, same size, bring to front/back, position pane | P0 | Y | D |
| Arrange | Auto Align & Space, Auto Size page | P1 | Y | M |
| Layers | Layers per page: visible, print, lock, snap, glue, colour; layer membership | P0 | Y | P |
| Containers | Containers (own their members, resize, headings), lists, callouts | P1 | Y | M |
| Data | Shape data sets, Define Shape Data dialog | P1 | Y | M |
| Data | Link data to shapes from Excel, data graphics, refresh (Pro) | P2 | Y | M |
| Data | Data Visualizer and Org Chart wizard from a table (Pro) | P2 | Y | M |
| Review | Comments, ink, compare drawings | P2 | N | M |
| Review | Validation rules and the Issues window (Pro) | P2 | N | M |
| View | Zoom, pan window, full screen, presentation mode, task panes | P0 | N | M |
| Automation | Every command on a CLI, a JSON control channel and MCP; headless render to PNG/SVG/PDF | P0 | Y | D |
| Automation | Build a diagram from a script: drop, glue, route, lay out, export | P0 | Y | P |
| Platforms | macOS, Windows, Linux, web from one workflow | P0 | N | M |

## Phase 1 evidence

- Open and save .vsdx (full fidelity, unknown parts kept): P, `misterdiagram-vsdx::reads_shape_cells_sections_text_and_unknown_content`.
- Open .vssx stencils and .vstx templates; save a stencil: P, `misterdiagram-vsdx::all_corpus_parses_and_seeded_mutations_never_panic`.
- Open .vsdm/.vssm/.vstm (drop macros, keep the rest): P, `misterdiagram-vsdx::macro_payload_removed_and_flagged`, `typed_vba_parts_are_removed_with_default_and_override_content_types` and `imports_macro_enabled_stencils_and_templates_without_vba_payloads`. Renamed typed VBA parts are removed by content type (PR #5); all six package families are admitted in library and CLI tests (PR #6). Full preservation and real-file fidelity remain partial.
- ShapeSheet model: Object, Geometry, Character, Paragraph, Tabs, Controls, Connection, User, Actions, Scratch, gradient stops: D, `misterdiagram-vsdx::reads_shape_cells_sections_text_and_unknown_content`.

- Drop masters from a stencil; master instances inherit and can be overridden: P, `misterdiagram-vsdx::evaluate::inherit::tests::corpus_basic_rect_and_probe_inheritance_witnesses`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

- Geometry sections with formulas (Width*0.5 style), guarded cells: P, `misterdiagram-vsdx::evaluate::path::tests::path_sampling_and_nearest_use_inherited_rows_and_partial_overrides`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

- Formula engine: cell references, cross-sheet references (Sheet.N!Cell), GUARD, Inh, THEMEVAL, SETATREF, dependency recalculation: P, `misterdiagram-vsdx::evaluate::tests::dirty_recalc_orders_dependencies_and_retains_unrelated_cache`. Read/evaluation subset; editing, host implementations and full corpus accuracy remain open.

## M2.0 evidence

- ShapeSheet model remains D: `misterdiagram-vsdx::dump::tests::basic_rectangle_corpus_has_every_com_identity_and_unit` checks all basic-rectangle COM identities and units, including Connection and Actions rows.
- Formula engine remains P: `misterdiagram-corpus::tests::strict_denominator_classes_and_empty_serialized_string` protects the corrected cached-oracle denominator, formula classification and empty-string cache comparison. Theme and text hosts remain open.

## M2.1 geometry evidence

- Geometry sections with formulas remain P: `misterdiagram-vsdx::evaluated_master_and_group_paths_use_inherited_rows` binds evaluated inherited rows to the L0 converter; `every_geometry_row_has_a_real_corpus_witness` covers all fifteen kinds, including RelEllipticalArcTo. Authoring remains Phase 4.
- Group transforms remain an implemented rendering prerequisite, without scoring move/resize/edit commands: `misterdiagram-geom::rows::tests::infinite_line_clips_and_group_composition_stays_y_up`.

## M2.2 paint evidence (historical)

- Format and connector-arrow rows remain M until an output backend and render comparisons exist. `misterdiagram-render::style::tests::measured_line_families_and_fill_density` checks the observed dash families and procedural fill coverage; `all_patterns_and_arrow_sizes_are_finite_and_bounded` checks all identifiers and sizes.

## M2.3 theme evidence

- Themes and variants: P, `misterdiagram-theme::resolve::tests::microsoft_themeval_examples_and_variant_selectors`. Package color/font/format tables, variant selectors and eight theme functions are evaluated. Corpus rates include theme cells; final rendered fidelity and full effect coverage remain open.

## M2.4 text evidence

- Character and paragraph formatting remains partial: `misterdiagram-vsdx::evaluate::text::tests::markers_fields_and_hostile_text` and `misterdiagram-text::tests::finite_wrapping_bidi_and_autofit`. Nonvolatile string/numeric field expansion is tested by `fields_expand_current_values_and_format_pictures`; aligned tabs and font-size spacing by `visio_font_percentage_empty_line_and_aligned_tabs`. Date-host fields retain their saved display.
- Text block control remains partial: `misterdiagram-vsdx::evaluate::text::tests::generated_text_group_resolves_runs_and_transforms` and `misterdiagram-text::tests::finite_wrapping_bidi_and_autofit`. `probe_connector_empty_text_retains_minimum_line_height` fixes the original empty-label discrepancy. Pixel fidelity and full text metrics have not passed.

## M2.5 output evidence

- PNG, SVG and PDF export: P, `misterdiagram-render::generated_pages_and_foreign_objects_produce_outputs` produces all three formats for all 204 generated drawings. PDF uses vector paths and embedded glyph text; visual and extraction acceptance follow in M2.6. JPEG/EMF export and links remain open.
- Pages and backgrounds: P, `misterdiagram-render::layers_visibility_print_and_background_pages` checks background inclusion and rejects a background cycle. Page size and drawing scale feed the scene; pixel fidelity is not yet accepted.
- Layers: P, `misterdiagram-render::layers_visibility_print_and_background_pages` distinguishes hidden screen layers from printable layers. Editing, layer colours, locking, snapping and glue remain open.
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

- M2.4.4 composed width evidence: `misterdiagram-text::tests::composed_width_terminal_space_and_advance_grid` reproduces twenty private cached witnesses. Text remains P until the independent pixel and metric gates pass.

- M2.4.5 text formatting evidence: `misterdiagram-render::text_metric_terminal_fields_and_unit_override_witnesses` and `misterdiagram-sheet::numbered_field_pictures_and_integral_fractions`. Both text rows remain P; no blanket claim for unresolved fonts or export bounds.

- M2.6.9 text block formatting remains P: `png_composition_and_logical_bounds_have_private_witnesses` resolves six dimensions; four one-pixel export residuals remain. SVG block placements retain their separate regression.

- M2.8 human gate accepted on 2026-10-12 at 16:45. Render export evidence: `one_pixel_common_area_preserves_rows_and_rejects_two_pixels`, `group_weights_common_pixels_and_keeps_size_failures_mandatory`, and `ceilings_match_charles_frozen_record`. All ten generated groups pass the frozen ceilings; four one-pixel boundaries are compared over the common area. Feature rows remain P where editing, broader fidelity or independent review is incomplete.

- M2.9.1: vendor stencil rendering remains P. `shipped_cisco_kubernetes_and_aws_have_drawing_ink` and `labels_do_not_turn_blank_masters_into_rendered_masters` replace successful-call counting with drawing-ink evidence.24stencils1108ink/0blank/0errors, corrected from465ink/643blank; visual fidelity remains bounded by the documented scope.

- M2.9 closure verification: two consecutive clean frozen passes at 3dc3b21. Vendor stencil rendering remains P: 1,108 ink-bearing drawing areas, zero blank or failed cells. Tests: `shipped_cisco_kubernetes_and_aws_have_drawing_ink`, `group_display_mode_hides_or_orders_group_paint`, `caller_built_sheet_cannot_split_cell_budget_across_sections`. Ink coverage and memory bounds do not establish complete visual or authoring parity.

## M3.2 writer evidence

- Open and save .vsdx (full fidelity, unknown parts kept): P. `authored_skeleton_writes_complete_opc_and_recalculates_local_cells`, `unknown_parts_and_extensions_survive_modelled_edits` and `instances_keep_only_local_overrides_and_master_relationships` cover serialization. Full corpus dump/render equality and Visio repair detection remain the M3.4/M3.5 gates.

## M3.3 package-kind evidence

- Open .vssx stencils and .vstx templates; save a stencil: P. `authored_stencil_and_template_retain_their_distinct_contracts` verifies the main content types, removal of drawing content for a document stencil, preservation of template pages, and re-reading. `existing_shipped_stencil_and_template_write_and_reopen` covers BASIC_U and BASFLO_U. Visio acceptance remains M3.5.

## M3.4 comparison evidence

- Open and save .vsdx (full fidelity, unknown parts kept): P. `every_corpus_package_roundtrips_at_least_ninety_nine_percent_of_dump_cells` covers all private packages; `complete_comparison_includes_master_pages_unknown_cells_and_deleted_rows`, `duplicate_cell_names_cannot_hide_an_earlier_change` and `rewritten_geometry_changes_are_visible_to_the_render_gate` guard the comparison surface. Visio's 230-file repair and re-export acceptance remains required before this row can be done.

## M3.5 construction evidence

- New master from a shape; edit master; document stencil: P.
  `five_documents_build_and_reread` constructs three independently identified masters,
  saves a document stencil and verifies its re-read. Editing commands remain Phase 4.
- Drawing and stencil/template save rows remain P until the 230-file Visio repair
  and re-export gate passes. The five-document test also covers a rectangle with
  text, Dynamic connector Connects, a private BASIC_U drop and metric/background/
  two-layer structure. `serialized_dump_uses_written_cache_without_claiming_inherited_predictions`
  guards acceptance against confusing evaluation errors with serialized values.

## M3.41 save acceptance evidence (2026-10-14)

- Open and save .vsdx (full fidelity, unknown parts kept): D for the M3 file
  contract. `every_corpus_package_roundtrips_at_least_ninety_nine_percent_of_dump_cells`
  reports 15080738/15080738 strict cells across 1387 packages.
  `unknown_parts_and_extensions_survive_modelled_edits`,
  `shipped_acceptance_shapes_preserve_local_structure_and_literal_attributes` and
  `shipped_acceptance_relationships_are_written_as_read` protect preservation.
  The 230-file Visio acceptance reports zero residual repairs and COM cells
  under the explicit baseline and first-save classes. Ten page-size groups pass.
  Charles's small-image ruling and the Phase 4 renderer gaps remain disclosed;
  this does not promote the separate rendering or authoring-command rows.
- Open .vssx stencils and .vstx templates; save a stencil: D for the M3 package
  contract. `authored_stencil_and_template_retain_their_distinct_contracts`,
  `existing_shipped_stencil_and_template_write_and_reopen` and
  `five_documents_build_and_reread` cover both save families and three authored
  masters. The ten shipped stencils, ten shipped templates and scratch stencil
  open with zero residual repairs and COM differences in the M3.41 VM record.
  The vendor-master rendering row remains partial.

## P4a authoring evidence (2026-10-15)

The engine owns a transactional Document session with bounded undo and journal.
Every registered command has a JSON schema and hostile-parameter coverage.
`session::probe_session_open_save_reopen_is_strictly_identical` and the session
suite exercise save/reopen and rollback. The four-file Visio acceptance record
is in `file-format-parity.md`. P4a met on 2026-10-15 under Charles's ruling:
cell/geometry equality and the unchanged corpus connector group ceiling govern
straight routes. Individual pair pixels remain informational until the connector
stroke rendering gap closes. M4 still requires 4b.

- Page tabs M to P: add, delete, rename, reorder, size, scale and background commands
  have authoring tests; page duplication remains open.
- Draw tools M to P: observed rectangle, ellipse and line Geometry is authored and
  the probe rebuild matches 6381 eligible cells. Freeform, arc and pencil tools remain open.
- Master drop P to D: `masters::basic_rectangle_drop_has_exact_local_and_inherited_cell_set`
  verifies local cells and Inh; ten BASFLO_U/NETSME_U drops are checked in Visio.
- Selection and transforms M to D: authoring and composition tests cover move,
  resize, rotation, flip, duplication, grouping, ungrouping and selection preservation.
- Shape data M to P and hyperlinks M to D: define/set Prop and hyperlink commands
  persist through save/reopen. Shape data sets and a GUI remain outside this evidence.
- Connection points M to P: add and ShapeSheet edits cover positions, directions
  and types; general control-handle interaction remains open.
- Master metadata/search M to P: case-insensitive name/keyword search and sizes
  are available; imported vendor icons are not copied into authored files.
- Dynamic glue M to P: static points and bounded whole-shape walking have live
  evaluator hooks; the ten-cell move-after diff is empty within 1e-6. Full walking
  preferences and obstacle-aware choices remain open.
- Routing M to P: straight routes update Geometry and XForm; orthogonal routing,
  obstacles, curved routes and line jumps remain 4b.
- Arrange M to D: align, distribute, same size and four order operations have
  authoring tests, including line endpoints after same-size and later movement.
- CLI automation M to P: commands, describe, run and convert share Session and
  pass hostile-argument tests. MCP authoring remains Phase 5.
- Script generation M to P: probe and two-boxes scripts run in CI; both local and
  Visio two-boxes PNGs have ink between boxes. Per-pair pixel fidelity remains
  an open renderer item under the 2026-10-15 ruling.

No row count, Core designation, priority or scoring weight changes in P4a.

## P4b partial evidence

- Orthogonal routing remains P:17 non-jump sweep cases, original obstacle and
  four straight witnesses match within1e-6; obstacle and page-layout edits reroute
  transactionally. Curved routes, general page/grid/plow semantics and dense
  attachment packing remain open.
- Line jumps M to P: original and five new jump cases match, including ArcTo A,
  gap and PolylineTo rows. The sixth has the named1/64in attachment-packing gap.
- Theme authoring remains P: SetThemeVariant refused COM; theme.apply deferred.
- Script generation remains P: SAN has24 Switch instances,40ISLs and80Connects,
  all-path ink and Visio re-save route equality. Required SAN serialization, font
  and render-height gates fail, so M4 is not met.

No row count, Core designation, priority, denominator or scoring weight changes.

P4b-closure candidate: the original-backed Property implementation matches all
11 local cells in four row types and3970 live COM cells; fresh Property repair
and COM residuals are0. The shape data row stays P while the candidate is blocked
from landing by full CI. No breadth point is claimed for an uncommitted change.
Master/text serialization still has redundant-marker residuals; font identity
and SAN extent focused checks pass. Routing/theme rows are unchanged.


## M4.20 to M4.27 closure evidence

Shape data P to D for the headless Core row: full eleven-cell Property rows,
canonical string units/formulas,3970-cell original rebuild with zero differences,
and fresh/retained Visio Property residual0 and COM0. Regression:
property_rows_match_visio_original_local_cells. Shape data sets and data-linking
remain separate missing rows; GUI windows are outside this headless score.
Master/text authoring preserves original cells while following Visio's authored
canonical pp/app.xml form. Master editing remains partial, routing and line jumps
remain partial for the named packing/unwitnessed preferences gaps, theme remains
partial with theme.apply deferred. No other row changes.

## M5.1 control inspection

Automation control-channel row remains P: engine.execute, engine.commands,
document.inspect and shape.inspect are available through the command registry.
Inspection tests cover inherited data/text, static glue, group page-space bounds,
read-only behavior and hostile parameters. The stdio server remains pending M5.2.

## M5.2 stdio tools

Automation command-channel row moves P to D: fourteen schema-described MCP tools
serve through the CLI with the existing 4 MiB framing cap. Tests exercise every
tool, full-session batch rollback, single undo/redo and exact saved-part equality
with strict commands. Python stdio builds the same two-boxes package as the CLI.
The script-building row remains P while P5a layout and topology are pending.

## M5.3 facing-side glue

Connection-point and glue rows remain P for the existing broader gaps.
connect.sides is covered by hostile parameters, point reuse and a fresh private
Visio original: canonical seven-cell rows, static Connects and route equality.

## M5.4 layout evidence

Connect Shapes/Re-Layout row moves M to P for facing-side connections, ordered
rows and fan point assignment. General flowchart/hierarchy/radial/circular
algorithms remain missing. Four layout integration tests and the SAN ink/bounds
test cover bounds, point order, hostile inputs, 80 Connects and no switch crossing.

## M5.5 topology and README evidence

The script-authoring row remains P. The MCP SAN recipe produces 24 switches,
40 ISLs and 80 Connects; Visio retains all routes and its equal-size export
passes the frozen pixel ceilings. The CLI-only office recipe produces ten
original device instances and nine links. `readme_office_network_matches_script_and_pixels`
checks exact README text and decoded image equality on each CI platform.
Fresh package checks pass with 0 SAN and 0 office repairs under Charles's
router-owned numeric Geometry ruling (296/28 judged cells, eight/zero changed
within 1e-6). Every other cell, formula and structure stays exact. Both human
visual gates are pending. No score increase for these
examples. Discovery regressions cover dependent bounds, tiny fan points,
text-sized layout and bounded aggregate MCP replies.

## M5.32 to M5.33 routing capacity

The orthogonal-routing row remains P. Capacity limits now publish connected
obstacle-free routes with per-connector warnings, preserving ordinary undo and
retry. `route_capacity` covers obstacle/grid exhaustion, glued endpoints,
save warnings, retry recovery and geometry-error rollback. Route sweep and
re-save numbers remain 28/29 and 40 connectors with zero differences.
Connector-local gathering preserves all 1,439 corpus/recipe packages and all
600 generated connector routes match the full obstacle set within 1e-9.
The four layouts include a far barrier requiring corridor growth.
Reference: https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute

- M5.44, Themes and variants: P unchanged. The strict `themeval_sweep` gate
  covers 264,405 private rows with no cached fallback; baseline 147,346 equal.
  Font faces compare by name; unresolved style rules remain explicit failures.
  Test: `themeval_sweep::tests::hostile_rows_and_exact_comparison`. Reference:
  <https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function>.

- M5.47, Themes and variants: P unchanged. The same strict population now has
  261,855 equal rows, including all 148,320 gradient rows, with no evaluator
  errors. Secondary backgrounds and dormant angles keep full parity open.
  Contributor-written transform, connector and font save/capture tests cover
  the implemented rules. Reference:
  <https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function>.


## M5.71 routing scope

Orthogonal routing and line jumps remain P. Ordinary edits select connectors by
changed glue/endpoints or a moved placeable crossing a free route. Unrelated
connector Shape XML remains byte-identical, including authored bends and jumps.
Explicit routing and layout retain their established scope. The geometry sweep
and its known jump-packing gap still bound the feature. See [engine.md](engine.md)
and [gaps.md](gaps.md) for the reason rule and witness table.

- Brief 49, export row remains P: editable SVG text and outline opt-in are covered by
  `editable_lines_requested_fonts_styles_and_outlines` and
  `cli_engine_and_mcp_share_svg_text_default_and_outline_flag`. Strict native text
  positions/wrapping and Visio extension/re-import acceptance remain open.
  Standard SVG reference: https://www.w3.org/TR/SVG11/text.html#TextElement


## M5.89 themed Character cells

Themes and Character formatting both remain P; scores are unchanged. Stored
Themed results now select the Character property through inherited formulas,
and indexed rows share text-contrast selectors and root defaults. Tests:
`stored_themed_character_values_use_root_defaults_and_keep_formulas`,
`intermediate_inherited_themed_cache_does_not_replace_font_provider`, and
`themed_character_contrast_applies_to_inherited_and_local_rows`. The private
native matrix matches 258/270 results; twelve absent Color/Style cells in sparse
rows remain a separate gap. See [gaps.md](gaps.md) for the witnesses and limits.
References: [Cell](https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml),
[THEMEVAL](https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function),
[QuickStyleVariation](https://learn.microsoft.com/en-us/office/client-developer/visio/quickstylevariation-cell-quick-style-section).

- M5.92 text rendering remains P: synthetic TextBkgnd palette, transparency, multiline extent and transform tests protect background painting in PNG/SVG/PDF; labelled authored connectors retain the existing theme default. Native witness: private `field/textbkgnd-2026-10-30/`.

- M5.93 image rendering remains P: local hidden Geometry paths now clip raster and EMF pictures; original #54 dimensions and hostile references have regression coverage. Native witness: private `field/image-crop-2026-10-30/visio-grid/`.

M5.94 corrects vertical PDF glyph offsets and editable SVG combining-mark placement; general text/script parity remains P.

M5.95 replaces the fitted marker law with synthetic all-type physical units and names the measured metric twin in editable SVG. Strict wide-matrix raster parity remains partial; no parity row is promoted.

M5.97 corrects empty-line SVG baselines and measured script advances. Native
synthetic controls cover all three vertical alignments and Calibri/Arial at two
sizes, with width and wrapping regressions. Text rendering stays P; no score or
denominator changes. References and remaining font limits are in gaps.md.

M5.98 progress: synthetic legacy THEME line defaults and nominal editable SVG
strokes now have native witnesses and regressions, including zero/dashed/double
pens. THEMEVAL remains unchanged. The independent text-only page crop extent is
still open in gaps.md. Existing partial parity rows and denominators are unchanged.

M5.100 progress: synthetic native page/master witnesses pin dangling-Connect
repair and first-occurrence duplicate shape IDs on import, including group
subtrees, Sheet 0 and inherited tombstones. Saved output preserves the same
identity and topology. Warning and hostile-limit regressions are portable.
Existing partial format/render rows, score and denominators remain unchanged.

M5.101 progress: parsed Character Pos runs now pin all six superscript/subscript
baseline shifts through the evaluator, including a general-family control and
mixed-case names. Each individual value mutation is detected. Production values,
parity scores and denominators are unchanged.

M5.102 progress: physical PDF shape/connector/marker pens follow native minimum
and zero rules without changing PNG/SVG output. Synthetic content-stream and
hostile-metadata tests cover export scales, dashes, groups and normal doubles.
Native malformed subminimum double bands remain an explicit gap. Existing
partial rendering rows, scores and denominators remain unchanged.
