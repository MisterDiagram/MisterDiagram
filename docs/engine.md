# Strict publication and routing scope

Every edit keeps the existing transaction rollback and strict write/import boundary.
The engine records successful routing validation with a weak document identity.
Opening a document or restoring a different history identity requires full validation
again. The marker adds no retained document owner. The saved package remains the compatibility contract, including inherited caches,
connector controls and line-jump geometry.

Routing uses a conservative exclusion proof (`route::scope`). A document with no
Connect records and no possible connector object category has an empty connector
closure after geometry validation. Uninherited shapes and literal non-connector categories can establish that
proof; formulas, inherited shapes and assignment effects retain the ordinary path.

For documents with connectors, the exclusion proof compares geometry, topology,
connection-point rows, page settings, document settings, styles, masters and retained
parts. Only edits with a validated predecessor can take this fast path. Only text, the listed fill/line presentation cells and Property rows can be
excluded. References to any excluded cell anywhere in the document, including a
formula intermediary, cancel the proof. Unknown functions, indirect cell-name construction,
assignment effects and bounded-analysis exhaustion also cancel it. Text metric
references cancel exclusion for text edits. This deliberately considers more
formula users than a routing-only dependency graph.

An edit that changes geometry, position or connection points keeps the existing
full evaluated page dependency closure. This includes both old and new obstacle
bounds and every route corridor, as well as formula-dependent target transforms.
The fallback is intentionally broad: the orthogonal visibility graph uses obstacle
edges beyond the current polyline, and strict publication also materializes
connector controls. A merely disjoint current polyline is not a proof of independence.
Page layout, document cells, theme/part changes and uncertain dependencies likewise
retain that closure. Forced reroutes bypass the exclusion proof.

Within preparation, endpoints sharing a target reuse its evaluated bounds and
connection points in one evaluator epoch. SETF assignments and contextual hook
activity invalidate the epoch; a computation that changes it is never cached.
Old and new document evaluators have separate caches. Empty routed sets skip
construction of the crossing-decoration evaluator.
