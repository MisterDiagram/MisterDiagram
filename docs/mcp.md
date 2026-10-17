# MCP

M5.1 adds the engine control methods `engine.execute`, `engine.commands`,
`document.inspect` and `shape.inspect`. They appear in CLI `commands` and
`describe`, including their JSON parameter schemas. Stdio tools remain a scaffold
until M5.2.

`engine.execute` takes `command` and optional object `params`. It delegates to
normal strict execution with exactly one publication and undo boundary. It rejects
self-invocation. `engine.commands` takes an empty object.

`document.inspect` takes optional one-based `page` and returns pages, evaluated
layer cells, flattened shapes with IDs, names, master identity, page-space
alignment bounds, inherited text and Shape Data cells, and Connects.
`shape.inspect` requires `id`, accepts optional one-based `page` and up to 256
requested `cells`, and adds evaluated XForm, text-transform, connection-point and
glue cells. Cell records contain value, formula, cache and units. Inspection is
read-only; response entries share a 1 MiB budget and oversized inspections fail
without truncation. Bounds are alignment bounds, not painted-ink bounds.

```sh
misterdiagram describe shape.inspect
misterdiagram run --open drawing.vsdx --cmd 'shape.inspect={"id":1,"cells":["Width"]}'
```

The library currently implements newline-delimited JSON-RPC with a 4 MiB message
cap and MCP lifecycle. CLI stdio startup and the fourteen-tool surface are next.
