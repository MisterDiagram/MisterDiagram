# MCP

M5.13b includes stencil lifetime controls, global search and bounded batch
snapshots. Three real-stdio regressions cover issues #58, #59 and #63 with
contributor-original 38 MiB stencils generated at runtime. The 256 MiB retained
media allowance includes active documents, stencils, history and rollback owners;
file opening and edited reimports enforce the same aggregate transactionally.

Run `misterdiagram mcp`. The server reads one JSON-RPC message per line on stdin
and writes replies on stdout. Diagnostics use stderr. Messages are capped at
4 MiB. `serverInfo.name` is `misterdiagram`. EOF shuts down the server.

Initialize with `initialize` (`protocolVersion: "2025-06-18"`, clientInfo and
capabilities), then send `notifications/initialized`. `tools/list` returns the
sixteen tools and their JSON Schemas. Each example below is the `params` of a
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
| export | `{"name":"export","arguments":{"path":"drawing.svg","page":1}}` | VSDX, PNG, SVG or PDF by extension. VSDX takes only path. PDF without page includes foreground pages. |
| render | `{"name":"render","arguments":{"path":"drawing.png","page":1,"scale":2}}` | Renders PNG, SVG or PDF at the requested scale. |
| list_commands | `{"name":"list_commands","arguments":{}}` | Lists command IDs, labels, schemas and enabled state. |
| run_command | `{"name":"run_command","arguments":{"command":"shape.draw.rect","params":{"x":1,"y":2,"width":2,"height":1}}}` | Creates a rectangle and returns its ID. Any registered engine command is available. |
| batch | `{"name":"batch","arguments":{"commands":[{"command":"shape.draw.rect","params":{"x":1,"y":2,"width":2,"height":1}},{"command":"shape.text","params":{"id":1,"text":"Cedar"}}]}}` | Creates and labels shape 1 in a new document, as one undo step. Returns results in order. |
| inspect_document | `{"name":"inspect_document","arguments":{"page":1}}` | Pages, layers, flattened shapes, inherited text/data, bounds and Connects. |
| inspect_shape | `{"name":"inspect_shape","arguments":{"page":1,"id":1,"cells":["Width","PinX"]}}` | Evaluated XForm, text transforms, data, connection points, glue and requested cells. |
| sheet_get | `{"name":"sheet_get","arguments":{"id":1,"name":"Width"}}` | Evaluated value, cached value, formula and units. |
| sheet_set | `{"name":"sheet_set","arguments":{"id":1,"name":"Width","formula":"2 in"}}` | Changes a cell and publishes its dependencies. GUARD requires force. |
| stencil_list | `{"name":"stencil_list","arguments":{}}` | Lists stencil index, name, canonical display path and master count. Index 0 is the document stencil. |
| stencil_close | `{"name":"stencil_close","arguments":{"stencil":1}}` | Closes external stencil 1; later external indices shift down by one. |
| stencil_search | `{"name":"stencil_search","arguments":{"query":"Dynamic","limit":10}}` | Searches the selected stencil, or the document and every open stencil when omitted. Every result includes its stencil index. |
| drop_master | `{"name":"drop_master","arguments":{"master":2,"x":0,"y":0}}` | Drops the contributor-original Dynamic connector master and returns its shape ID. |
| connect | `{"name":"connect","arguments":{"id":3,"end":"begin","target":1,"mode":"static","point":"Connections.X1"}}` | Glues connector 3 to an existing connection point on shape 1. Omit point/mode for dynamic glue to the target shape. |

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
Every item executes exactly as run_command does, including publication before
later reads or edits. If any item fails, the entire session is restored:
document, selection, open stencils, path, journal, undo and redo. Errors report
both the zero-based failing index and the one-based command number. Successful
batches have one undo step, including stencil changes. The output list has a
1 MiB budget; exceeding a budget fails transactionally. Unchanged open stencils
are shared by reference and omitted from batch undo snapshots. A batch changing
the stencil inventory retains it for undo and is still subject to the existing
256 MiB history estimate and 4,000,000 retained XML node limit. A genuinely
oversized snapshot fails with `batch undo snapshot exceeds retained budget`. Files already written
by save/export commands are external side effects and are not removed on a
later failure. Prefer saving after a successful batch. This transaction wrapper
does not implement the deferred-publication optimization shelved under #39.

Engine control methods `engine.execute`, `engine.commands`, `document.inspect`
and `shape.inspect` appear in CLI `commands` and `describe` with schemas.
`engine.execute` takes command and optional object params, delegates without an
extra undo boundary, and rejects self-invocation. Inspection is read-only.
Bounds are page-space alignment boxes including group transforms, not painted
ink. Cell values are evaluated strings with formula, cache and units retained.
Up to 256 extra cell names can be requested. Inspection entries share a 1 MiB
budget and oversized queries fail without truncation; use a page or shape query.

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
