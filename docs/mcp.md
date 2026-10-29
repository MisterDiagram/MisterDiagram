# MCP

Stencil lifetime controls, global search and bounded batch snapshots have
three existing real-stdio regressions for issues #58, #59 and #63 with
contributor-original 38 MiB stencils generated at runtime. The default 512 MiB retained
media allowance includes active documents, stencils, history and rollback owners;
file opening and edited reimports enforce the same aggregate transactionally.

The default session ceiling is 2 GiB. Use `misterdiagram mcp --memory-limit 4096`
(or `misterdiagram --memory-limit 4096 mcp`) to change it in whole MiB.
`MISTERDIAGRAM_MEMORY_LIMIT=4096 misterdiagram mcp` sets the same ceiling; the
flag wins. Invalid, zero, fractional and overflowing values fail before serving.
The CLI `run` and `convert` use the same session policy. This setting changes only
session retention, never per-file source/decompression/XML/render bounds or the
fixed 4,000,000-node limit. At the default, source/media/history allowances are
128/512/256 MiB with a 1,536 MiB aggregate estimate and 512 MiB transient margin.
Errors name needed, held and limit values, exact bytes and the setting. Fixed node
and stencil-count errors explicitly say that the setting cannot raise those
fixed bounds. Four additional real-stdio memory tests cover shared inventory
snapshots, media admission/drop, inherited rollback and the immutable node bound.
See docs/architecture.md for the derivation.

Run `misterdiagram mcp`. The server reads one JSON-RPC message per line on stdin
and writes replies on stdout. Diagnostics use stderr. Messages are capped at
4 MiB, including the newline. Over-cap lines are drained through their newline
without retaining bytes beyond the cap and receive one -32700 error with null id
and `MCP message exceeds cap`. Invalid UTF-8 receives the same -32700 parse-error
code as malformed JSON. The next request uses the same session. EOF or an actual
stream I/O failure ends serving. `serverInfo.name` is `misterdiagram`.

Engine command panics return an internal command error and preserve the session:
strict edits restore their snapshot; batch and composition discard the candidate.
This includes `engine.execute` delegation and orchestration outside strict edits.
MCP adds a per-message backstop: an escaped panic logs to stderr and returns
JSON-RPC -32603 with the request id (null if absent), then continues serving.
External files already written remain external side effects.

Recovery tests use real stdio and the unchanged package-comparison masks:

```sh
cargo build -p misterdiagram-cli --features test-panic-injection
MISTERDIAGRAM_TEST_INJECTIONS=1 python3 tools/mcp/test_recovery.py
```

The non-default feature is for test binaries only. Release artifacts are built
without it and checked for the absence of all injection markers.

Initialize with `initialize` (`protocolVersion: "2025-06-18"`, clientInfo and
capabilities), then send `notifications/initialized`. `tools/list` returns the
seventeen tools and their JSON Schemas. Each example below is the `params` of a
`tools/call` request, wrapped as:

```json
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_commands","arguments":{}}}
```

Results have one JSON-encoded text content item and `isError: false`. Tool
failures return `isError: true` with an error message; protocol errors use the
JSON-RPC error member. Older protocol revisions 2025-03-26 and 2024-11-05 are
also recognized. The existing ping and resource discovery methods remain.

Coordinates are inches, y-up. `page` is a one-based ordinal; shape IDs are
page-scoped IDs, not vector indices. The default page is the session's current
page. Every mutating command uses the engine's strict write-and-reimport path.

| Tool | Worked call (`params`) | Result or effect |
| --- | --- | --- |
| open | `{"name":"open","arguments":{"path":"drawing.vsdx"}}` | Replaces the active document; returns page count and macro-removal flag. |
| save | `{"name":"save","arguments":{"path":"drawing.vsdx"}}` | Saves VSDX, VSSX or VSTX by extension; returns bytes and warnings. |
| export | `{"name":"export","arguments":{"path":"drawing.svg","page":1}}` | VSDX, PNG, SVG or PDF by extension. VSDX takes only path. PDF without page includes foreground pages. SVG defaults to editable text; `text_outlines: true` preserves glyph paths. |
| render | `{"name":"render","arguments":{"path":"drawing.png","page":1,"scale":2}}` | Renders PNG, SVG or PDF at the requested scale. SVG defaults to editable text; optional boolean `text_outlines` selects glyph paths. |
| list_commands | `{"name":"list_commands","arguments":{}}` | Lists command IDs, labels, schemas and enabled state. |
| run_command | `{"name":"run_command","arguments":{"command":"shape.draw.rect","params":{"x":1,"y":2,"width":2,"height":1}}}` | Creates a rectangle and returns its ID. Any registered engine command is available. |
| batch | `{"name":"batch","arguments":{"commands":[{"command":"shape.draw.rect","as":"cedar","params":{"x":1,"y":2,"width":2,"height":1}},{"command":"shape.text","params":{"id":"$cedar","text":"Cedar"}}]}}` | Creates and labels a rectangle as one undo step. Returns results and the reference map. |
| inspect_document | `{"name":"inspect_document","arguments":{"page":1}}` | Pages, layers, flattened shapes, inherited text/data, bounds and Connects. |
| inspect_shape | `{"name":"inspect_shape","arguments":{"page":1,"id":1,"cells":["Width","PinX"]}}` | Only id, bounds and the requested cells; optional sections select other groups. |
| sheet_get | `{"name":"sheet_get","arguments":{"id":1,"name":"Width"}}` | Evaluated value, cached value, formula and units. |
| sheet_set | `{"name":"sheet_set","arguments":{"id":1,"name":"Width","formula":"2 in"}}` | Changes a cell and publishes its dependencies. GUARD requires force. |
| stencil_list | `{"name":"stencil_list","arguments":{}}` | Lists stencil index, name, canonical display path and master count. Index 0 is the document stencil. |
| stencil_available | `{"name":"stencil_available","arguments":{}}` | Lists `.vssx` filenames, folders and byte sizes on the configured search path without opening packages. |
| stencil_close | `{"name":"stencil_close","arguments":{"stencil":1}}` | Closes external stencil 1; later external indices shift down by one. |
| stencil_search | `{"name":"stencil_search","arguments":{"query":"Dynamic","limit":10}}` | Searches the selected stencil, or the document and every open stencil when omitted. Every result includes its stencil index. |
| drop_master | `{"name":"drop_master","arguments":{"master":2,"x":0,"y":0}}` | Drops the contributor-original Dynamic connector master and returns its shape ID. |
| connect | `{"name":"connect","arguments":{"id":3,"end":"begin","target":1,"mode":"static","point":"Connections.X1"}}` | Glues connector 3 to an existing connection point on shape 1. Omit point/mode for dynamic glue to the target shape. |

Passive sheet reads and saves treat root SETF and witnessed GUARD(SETF(...))/1+SETF(...) Action formulas as zero without
executing the assignment. Text NOT and numeric Action formulas retain their
stored values, following the M5.10 witness and ruling.

Open external stencils with `run_command` / `stencil.open {"path":"original.vssx"}`.
Engine commands `stencil.list` and `stencil.close {"stencil":1}` have the same
schemas as their MCP tools. The list includes index 0 for the document stencil,
whose path is null for an unsaved document. Non-UTF-8 path bytes appear as
replacement characters in display strings. External indices are 1 through 64.
`file.new` and `file.open` keep external stencils open. Closing one compacts the
indices, so list again before reusing stored indices. Index 0 cannot be closed.
Search, master listing and drop accept explicit index 0 for the document.
Global search visits the document first, then open stencils in index order;
`limit` caps the combined results, while `matched` counts all matching masters.

`connect` uses the existing connector's ID. `end` is `begin` or `end`.
`mode` is `static` or `dynamic`; the default is static when point is supplied,
otherwise dynamic. Dynamic glue accepts preference `horizontal` or `vertical`.
Create connection points through run_command/shape.connection.add.

Batch supports 1 to 1,024 commands within the engine's 1 MiB parameter budget.
An item may carry `"as":"name"`; later shape-ID parameters may use `"$name"`,
including ID arrays and tree edges. Names contain 1 through 64 ASCII letters,
digits, underscores, periods or hyphens, are case-sensitive and resolve in order.
Any successful command returning an `id` can be named, including grouped master
drops. The result includes `references`, a name-to-ID object, alongside `results`.
Unknown names, duplicates and naming a command without an ID fail the whole batch
with the item index. References are local to one batch. Text is literal except
that a leading `$$` loses one `$`, so `$$cost` produces `$cost`; `$cedar` in a text
parameter remains text. Nested batches are rejected; combine their command lists
into one batch. The engine command `engine.batch` exposes the same contract.

Every item executes exactly as run_command does, including publication before
later reads or edits. If any item fails, the entire session is restored:
document, selection, open stencils, path, journal, undo and redo. Ordinary item errors report
both the zero-based failing index and the one-based command number. Successful
batches have one undo step, including stencil changes. The output list has a
1 MiB budget; exceeding a budget fails transactionally. Unchanged open stencils
are charged once by identity even when an inventory-changing batch includes them
in its undo snapshot. A batch changing the stencil inventory retains references
for undo and is still subject to the existing
256 MiB history estimate and 4,000,000 retained XML node limit. The history
estimate charges shared immutable nonmedia part bytes once across snapshots,
plus 1 MiB per historical document; detached allocations are charged separately.
Shared page/master content nodes are charged once in the aggregate retention
census only when the reader proves identical source, DOM, Shapes and Connects
allocations. Detached contents and metadata remain charged; per-file checks and
all ceilings are unchanged. An oversized snapshot fails with `batch undo snapshot exceeds retained budget` and
the actual history-byte or fixed-node bound. Inventory rollback refusals also
explain that opening stencils in their own call avoids that extra retention. Files already written
by save/export commands are external side effects and are not removed on a
later failure. Prefer saving after a successful batch. This transaction wrapper
does not implement the deferred-publication optimization shelved under #39.

Engine control methods `engine.execute`, `engine.commands`, `document.inspect`
and `shape.inspect` appear in CLI `commands` and `describe` with schemas.
`engine.execute` takes command and optional object params, delegates without an
extra undo boundary, and rejects self-invocation. Inspection is read-only.
Bounds are page-space alignment boxes including group transforms, not painted
ink. Cell values are evaluated strings with formula, cache and units retained.
Up to 256 cell names can be requested. With `cells` and no `sections`, shape
inspection returns exactly `id`, `bounds` and `cells`. Both inspection tools
accept `sections`: an array of `xform`, `text`, `connection_points`, `data`,
`glue`, `cells`. Explicit sections select only those groups in addition to id
and bounds; `text` includes text transforms, `glue` includes Connects, and
`cells` includes the requested names. An empty sections array requests identities
and bounds only. Document inspection retains its page/layer/selection envelope.
Omitting both filters retains the default summary.

Inspection has a 65,536-byte (64 KiB) serialized JSON budget. Every MCP tool
result, including errors, has a 1,048,576-byte (1 MiB) serialized content-envelope
budget, including JSON escaping. Exceeding either budget returns an error naming
the applicable `cells`, `sections`, `page`, `query`/`limit` filters or batch size.
Results are never silently truncated. Select fewer sections or a single shape,
or split a large batch. The engine checks the escaped-result budget before
publishing session changes, so oversized command and batch results roll back
in-memory state. Files already written remain external side effects. The engine
reserves 64 bytes for the transport envelope inside the same 1 MiB allowance.

Resources `misterdiagram://document` and `misterdiagram://commands` return these
same engine queries. The Python example and real stdio regression are runnable:

```sh
cargo build -p misterdiagram-cli
python3 tools/mcp/two-boxes.py target/mcp-example --cli target/debug/misterdiagram
MISTERDIAGRAM_CLI="$PWD/target/debug/misterdiagram" python3 tools/mcp/test_stdio.py
MISTERDIAGRAM_CLI="$PWD/target/debug/misterdiagram" python3 tools/mcp/test_stencils.py
```

The regression compares the saved two-boxes package with the CLI script and
compares batch with individual commands using `tools/bench/compare.py`. Only
fresh master GUIDs and docProps save times are masked. It also asserts rollback
when the third command of five fails.

Facing-side glue through run_command:

```json
{"name":"run_command","arguments":{"command":"connect.sides","params":{"id":3,"source":1,"target":2}}}
```

This glues an existing connector to the facing side centers of two 2D shapes,
creating canonical connection rows if absent. Existing inbound points at those
positions are reused. Rotation and enclosing group transforms are honored.
Separated physical tiers use top/bottom points and style-17 midpoint routing,
including flipped endpoints.

Ordered layout is also available through run_command:

```json
{"name":"run_command","arguments":{"command":"layout.rows","params":{"ids":[1,2,3],"counts":[2,1],"region":{"x":0,"y":0,"width":10,"height":8},"gutter_x":0.5,"gutter_y":1}}}
{"name":"run_command","arguments":{"command":"layout.fan","params":{"hub":1,"spokes":[2,3]}}}
```

Rows run top to bottom in list order, centered within the region (the page by
default), and return placed alignment bounds. Gutters default to max(BlockSize,
2*LineToNode) on each axis. Oversized layouts, duplicate IDs, selected ancestors
and descendants, and 1D shapes are rejected transactionally. Group-child positions
are translated through their parent transforms.

Fan requires exactly one existing connector from the hub to each listed spoke.
It assigns distinct points at both ends, considering every connected peer at a
spoke, and uses witnessed straight routing (style 2) for separate ISL tracks.
Within each facing side, points approximate even spacing in spoke order using
the nearest distinct binary fractions. Five spokes use 1/8, 3/8, 1/2, 5/8 and 7/8.
Layout normalizes selected sizes and pins to 1/64 inch; formula dimensions keep
their dependencies inside a grid expression. Rows reserve 1/32-inch size slots
to preserve gutters after pin snapping. Imported, unselected shapes stay unchanged.
The response lists assignments and any route/shape crossings in the supplied set.
Boundary contact is allowed; open alignment-box interiors are tested. Route
inspection is bounded to 100,000 segments and two million segment/box checks.

Run the full topology through stdio from the repository root:

```sh
cargo build -p misterdiagram-cli
python3 tools/mcp/san-topology.py target/mcp-san
```

The client saves `p5a-san-topology.vsdx`, `.png` and `.svg`, verifies 80 Connects
and rejects reported switch crossings. The recipe is shared with the CLI example.
Fresh Visio keeps its routes and passes the unchanged render ceilings and
unrepaired package gate under the narrow [route-geometry ruling](file-format-parity.md#ruling-router-owned-connector-geometry-charles-2026-10-17).
Both human visual verdicts remain pending.
Rows whose formula-driven bounds change when moved are rejected transactionally.
Legacy JSON-RPC arrays share the 4 MiB response cap; oversized replies return an
error without emitting an oversized frame. This is separate from the `batch` tool.

The compiled network library opens with `stencil.open {"builtin":"network"}`.
Search it with `stencil_search`; `drop_master` and `master.drop` accept semantic
names such as `network.router` or `storage.switch` across open stencils. An
ambiguous name requires an explicit stencil index. Drop coordinates default to
zero so a layout command can place the devices. All eleven masters share native
geometry, editable category colors and the same label band below the square icon.
`sheet.set` also accepts the seven optional Text Transform cells directly on a
plain shape or dropped instance; no section/row creation is needed (#61).

`format.line.units` accepts `IN` (default), `PT`,
`MM`, `CM` or `FT`. Weight is converted to internal inches and its display unit
is retained; `{"id":1,"weight":1.5,"units":"PT"}` writes a 1.5 point line. Units
require weight. CLI `misterdiagram --version` prints the crate version, the build commit and the
target triple (`misterdiagram 0.1.0-alpha.16 (0123456789ab, aarch64-apple-darwin)`; source builds say
`unknown commit`); MCP serverInfo.version is the crate version alone. The office and SAN examples use the built-in semantic library.

`layout.tree` accepts `root` plus ordered `edges: [[parent,child],...]`, or `root`
plus `ids` whose existing connects form a tree. It creates missing connectors for
explicit edges and reuses unambiguous existing links. Each depth shares a PinY;
parents centre over their immediate children's alignment-box span. Nodes use
physical bottom/top connection points and witnessed SimpleNS routing (style 17),
with horizontal jogs midway between tiers. Edge order controls sibling order;
inferred links use shape-ID order.

Spacing defaults to the greater of BlockSizeX/Y and twice LineToNodeX/Y, with
`gutter_x` and `gutter_y` overrides. `region` has x/y/width/height; otherwise the
page is used. `fit_page: true` resizes around the tree with `margin` (default
0.5 inches). A fit that would exclude unrelated content is rejected. Shapes must
be unrotated top-level nodes. Dimensions round up to sixteenths, pins and gutters
to thirty-seconds, leaving connector midpoints on the 1/64-inch grid. Impossible
region fits, position-dependent transforms/bounds, and obstacles requiring an
off-grid or non-midpoint detour roll back the whole command. The node/edge limits
are 10,000/9,999, with two million node-edge work units.

`layout.check` is a read-only query with optional `page`, `root` and `fit_page`.
It reports `count` and `violations`, each with a rule letter, shape IDs, connector
IDs and detail. Rules a-g check connection-point glue and drawn endpoints,
shape crossings, parallel border clearance within 1/16 inch (inclusive), shared
segments outside a continuous source trunk, tree tier/parent alignment, equal
fitted margins within 1/64 inch, and label crossings. Tree checks require `root`;
margin checks require `fit_page: true`. Labels use evaluated Text Transform cells,
including nested shapes and connector text. A connector crossing its own visible
label is also a violation. Disconnected geometry cannot extend a trunk exception.
Traversal is capped at 10,000 shapes, 100,000 flattened segments, two million
comparisons and 10,000 violations; uncheckable input returns an error.

The command and CLI/README tests share the same checker through:

```sh
python3 tools/check-layout.py drawing.vsdx --cli target/debug/misterdiagram
python3 tools/check-layout.py tree.vsdx --cli target/debug/misterdiagram --root 1 --fit-page
```

Exit 0 means no violations, 1 means violations, and 2 means the input could not
be checked. The checker never edits the drawing.

The office recipe in `tools/examples/office-network.sh` drops ten named semantic
masters, then calls `layout.tree` with nine parent-child pairs and `fit_page`.
It has no placement coordinates. `tools/mcp/san-topology.py OUTPUT` builds
24 `storage.switch` instances and 40 ISLs in two fabrics through MCP, then
requires `layout.check` to report zero violations before saving and exporting.
Both examples pass fresh Visio open/save with zero repair and route differences,
and the unchanged equal-size pixel ceilings. Their human visual gates remain pending.

Creation commands `shape.draw.rect`, `shape.draw.ellipse`, `shape.draw.line` and
`master.drop` accept optional `text` (a string), `fill` (format.fill properties),
`line` (format.line properties) and `font` (format.text properties). Formatting
objects omit `id` and `page`. These edits have one undo step and save the same
cells as the corresponding fine-grained commands, without command markers.

`connector.add {from,to,glue,from_point,to_point,line}` requires distinct endpoint
shapes. It drops the document's
Dynamic connector and glues both ends. `glue` defaults to `static`, using facing
sides; `dynamic` uses whole-shape glue. Explicit points require both point names
and static glue. Automatic links reserve unused ports when an endpoint is already
occupied. Parallel links use separate points at both ends and straight route
style 2; existing static ports, including explicitly selected ports, stay put.
Automatic dynamic endpoints become static points when links run in parallel.
`layout.fan` accepts multiple connectors per spoke and assigns every link its own
pair of points using the same straight style. These are ordinary connection rows,
glue records and route cells, with no private saved state.

Examples use the built-in contributor-original network library:

```sh
python3 tools/mcp/office-network.py target/office
python3 tools/mcp/san-topology.py target/san
python3 tools/mcp/fabric-page.py target/fabric
```

Office uses one reference batch for ten labeled devices and the tree layout.
SAN uses one batch for the larger topology. The fabric example demonstrates three
array links per fabric and reports container-related layout hits separately.

Measured recipe sizes are office 12 commands in one batch (four total tool
calls), SAN 338 commands in one batch (five total calls), and fabric 28 build
commands in two batches (five total calls). Totals include layout inspection and
save/render or export. The fabric page reports 10 rule b and 10 rule g hits on
its ordinary boxes and headings; a, c and d pass. These boxes are not containers.

Use MCP for interactive edits, inspection and the render-and-look loop. Use
`render` for previews; use `export` for deliverables. Both reach the same export
engine, but naming the intent makes a tool transcript easier to follow.
`list_commands` returns each command's `params` as a JSON object. Every parameter
has a description stating its units and coordinate anchor where applicable.
In particular, rectangle/ellipse x/y are the bottom-left corner in page inches;
master.drop x/y are the pin in page inches. This documents existing behavior.

For a drawing generated from data with hundreds of commands, write a script that
builds the command list and runs it with the CLI. Keep the data and script so the
drawing can be rebuilt when the data changes. `run --batch FILE` reads at most
1 MiB of JSON with the same `{"commands":[...]}` envelope as MCP batch. A short
worked example writes two labeled rectangles and a link from a data list:

```sh
python3 tools/mcp/script-route.py target/script-route --cli target/release/misterdiagram
# The script writes commands.json, then runs:
target/release/misterdiagram run --batch target/script-route/commands.json --save target/script-route/drawing.vsdx
```

The source uses `as` names and `$name` references, so adding another data row does
not require predicting or renumbering shape IDs. Saving follows the successful
batch as a separate operation.

`file.new` and `open`/`file.open` refuse to replace a document with unsaved edits.
The error tells callers to pass `discard: true` when replacement is intentional.
A fresh session is clean; save marks the current revision clean. Undo and redo
track the saved revision, and inspection, previews and external stencil loading
do not mark the drawing dirty. The guard state lives only in the session and is
never written into the document.
If a transaction returns an operation error after a save has written a file, the
restored in-memory document is conservatively marked unsaved. Panic recovery
restores revision/save bookkeeping exactly along with the rest of the session.
Neither path can roll back external file writes. The operation-error protection
prevents a later new/open from discarding content that the save overwrote.
Save the restored document, or explicitly discard it, to clear that protection.


The Brief 27 final joint acceptance retains 18/80/20 Connects and every route
for office/SAN/fabric after Visio re-save, with zero repair differences. Office
and SAN pass all applicable layout checks. Fabric retains the documented box
and heading hits; these ordinary shapes do not gain container semantics.

### Routing capacity warnings

Successful engine command results, including CLI commands and MCP `run_command`
and `batch`, include `route_warnings` when the session contains degraded routes.
The field is omitted when empty. Each entry contains `page` (the saved page ID,
not the one-based command page ordinal), `connector` (shape ID), and `limit`
(the named routing work bound). For example:

```json
{"route_warnings":[{"page":0,"connector":261,"limit":"route obstacle count limit"}]}
```

Capacity exhaustion publishes a connected obstacle-free fallback route. Invalid
coordinates and other geometry errors retain their failure behavior. `file.save`
also lists every still-degraded connector in its existing `warnings` and
`warning_count`. `route.reroute` retries routing and clears warnings for recovered
connectors. Degraded status is session state, restored with undo/redo; it is not
extra package metadata. Opening a saved package preserves its stored geometry.

The named capacity bounds are `route obstacle count limit` (4,096 local
obstacles above the unchanged 256-candidate path), `route grid limit`
(16,384 vertices), `route queue limit`, `route obstacle closure limit`, and
`route corridor growth limit` (32 attempts). Corridor selection uses the page
LineToNode and LineToLine spacing and grows before declaring a geometric failure.

### Master drawing scale

`stencil.masters` and `stencil.search` keep `width` and `height` in master drawing
units. Each result also includes `scale` (`page_scale`, `drawing_scale`,
`drawing_scale_type`) and `drop_size` (`width`, `height`, `factor`) for the current
page. With no current page, `drop_size` is `"page-dependent"`.

`master.drop`, the `drop_master` tool and layouts that drop masters use the same
observed Visio rule. When exactly one of the master and page is NoScale, the drop keeps its
paper size only when the relative scale exceeds eightfold in either direction.
Exactly eightfold is unchanged, as are drops between like scale types. Explicit
`width` or `height` overrides that dimension. Line endpoints follow the dropped
width. A missing master PageSheet or scale declaration uses the unscaled 1:1
default. Built-in 1:1 masters on the default 1:1 page keep their original saved representation.
`page.scale` selects NoScale for equal PageScale and DrawingScale, otherwise
Custom. See the M5.29 matrix in `docs/file-format-parity.md` for the measured
boundary and the multi-root master limitation.

### Stencil folders

Set `MISTERDIAGRAM_STENCIL_PATH` to an ordered list of folders. macOS and Linux
use a colon; Windows uses a semicolon. `--stencil-path` accepts the same list and
replaces the environment setting, including on `misterdiagram mcp`.

```sh
export MISTERDIAGRAM_STENCIL_PATH="$HOME/Stencils:$HOME/TeamStencils"
misterdiagram run --cmd 'stencil.available={}'
misterdiagram run --cmd 'stencil.open={"path":"affinity_native"}'
misterdiagram mcp --stencil-path "$HOME/Stencils:$HOME/TeamStencils"
```

```powershell
$env:MISTERDIAGRAM_STENCIL_PATH = 'C:\Stencils;D:\TeamStencils'
misterdiagram mcp --stencil-path 'C:\Stencils;D:\TeamStencils'
```

In an MCP client config, for example:
`"args": ["mcp", "--stencil-path", "/Users/you/Stencils:/Users/you/TeamStencils"]`.
Use semicolons and doubled JSON backslashes for Windows paths.

A bare filename searches each folder's top level in order. Within a folder, the
exact filename is tried first, then the name with `.vssx` appended if omitted.
The first match wins, and `stencil.open` returns its resolved absolute `path`.
A missing filename reports every folder searched. A path containing a directory
separator, `..`, NUL or a drive prefix uses the existing explicit-path route and
never searches these folders. Without a configured list, the working directory
is searched; an empty list entry also means the working directory.

`stencil.available` (MCP tool `stencil_available`) returns `files` with `name`,
`folder` and byte `size`, plus the configured `folders`. It lists `.vssx` files
without opening or parsing them, including a file that would fail admission when
opened. It does not recurse or build a master index. `stencil.list` reports each
open external stencil's resolved absolute path; the document stencil has no path
until opened or saved, and the built-in library retains `builtin:network`.
Files found by name pass the same per-file and session memory checks as explicit
paths. The search setting persists across document replacement.
