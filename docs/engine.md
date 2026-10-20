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

Strict publication also shares immutable package allocations. The writer still
builds and evaluates the complete candidate using the existing dependency policy.
It reuses source bytes only when the whole generated XML tree equals an admitted
source tree, or when the completed serialization equals the whole original byte
sequence. Page, document and theme changes therefore retain the writer's existing
formula invalidation. An unchanged filename alone never permits reuse.

The reader's private memo binds an original part name to its exact byte allocation,
parsed XML, parsed shape/connect vectors and original logical node charge. Only
that proof can skip parsing on publication. Each document version still incurs
its original logical node charge; reuse never raises the 4,000,000-node bound.
All file, XML, part, entry and decompressed-package limits remain unchanged.
Internal publication transfers the admitted part map without ZIP round trips.
A conservative compressed-size envelope proves the original compressed-file cap;
when inconclusive, a counting ZIP writer checks the actual compressed size.
Ordinary file saves compress once and check their actual output size.

Picture bytes use copy-on-write ownership across documents, stencils, history,
batches and rollback snapshots. A reservation pins each counted picture allocation
and charges it once, including its raw-byte contribution. Replacing or mutating a
picture detaches that allocation; the old version keeps its original bytes.
Nonmedia source, history and logical-node accounting retain their prior rules.
The default 2 GiB session ceiling and all derived limits are unchanged.

Within one evaluator, immutable ShapeSheet resolution is reused separately from
live evaluated values. Effective inherited maps share read-only allocations;
bare-cell default availability is memoized by scope and property category, with
cycle errors checked before every hit. Baseline-cell lookups cache source cells
and misses only after inheritance construction finishes. Formula overrides,
SETF, context changes and dependency invalidation retain their existing paths.
Entry and name-storage caps disable further caching when reached, then fall back
to the original resolver. They do not reject additional input or alter an existing
construction, recursion or file bound.
