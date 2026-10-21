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

## Connector lane observations (M5.39, incomplete)

Brief 34 observes Visio build 16.0.20430.20182 on contributor-authored synthetic
rows, columns, fan-outs and obstacle grids. Generated drawings and exports remain
in the private corpus. These observations do not yet establish a general lane
assignment algorithm; production routing is unchanged.

The public [LineToLineX reference](https://learn.microsoft.com/en-us/office/client-developer/visio/linetolinex-cell-page-layout-section)
and [LineToLineY reference](https://learn.microsoft.com/en-us/office/client-developer/visio/linetoliney-cell-page-layout-section)
define horizontal and vertical connector clearance. They do not specify lane
ordering or a minimum-spacing fallback. In both measured five-connector cases,
changing those page cells from 0.125 to 0.25 inches leaves the lane coordinates
unchanged. LineToNode remains 0.125. The experiment does not establish which
of these settings determines the initial 0.1875-inch displacement.

A restricted candidate describes the positive-direction two-row/column
fixtures. Two rows each have 2N placeable, non-permeable half-inch boxes at a
2-inch pitch. Source i connects to opposite-row index 2N-1-i. The source-facing
edge is 1.5 inches. The first trunk is 0.1875 inches from that edge. Subsequent
trunks fill toward the edge in connector creation order at an initial 0.0625-inch
step. Once those slots are occupied, halve the step and scan again, retaining
already assigned lanes. The measured clear gap, 0.75 or 3 inches, does not alter
this compact band. This describes the observations, not a proven dependence on
LineToLine or LineToNode.

```text
used = [0.1875]
step = 0.0625
while fewer than required trunks:
    for offset = 0.1875-step down to greater than zero, by step:
        if offset is not already occupied:
            append offset to used
            stop when the required count is reached
    step = step/2
lane coordinates = source-facing edge + used
```

Columns use all N offsets. In the horizontal fixture, the innermost connector
instead chooses a horizontal-vertical-horizontal path and crosses the central
cut at the target-row center (1.75 + gap). Thus only N-1 horizontal trunks lie
inside that gap; the last reported coordinate is explicitly an outside-gap
crossing, not another horizontal lane. The vertical trunk of that connector
still passes through the gap. This asymmetry is part of the observation.

This candidate does not explain the whole discovery set. Alternating directions
produce lower-side lanes 1.6875/1.625 and upper-side lanes 4.3125/4.1875 for the
3-inch gap (2.0625/1.9375 for the 0.75-inch gap). They do not mirror the lower-side
insertion sequence. Fan-outs select distinct points on the shared source and
have no shared centerline segment in either the two-target or six-target case.
A general rule for those port positions and the upper-side packing is unresolved.
Native-large-300's offsets below target edge 28 are 0.1875, 0.3125, 0.125, 0.25,
0.0625, 0.375, 0.4375, 0.15625, 0.21875 and 0.09375 inches. They also exceed the
restricted candidate's band. No ordering permutation repairs that limitation.

The three grid sizes select a horizontal first leg, a vertical trunk at
x=1.6875, and a horizontal trunk 0.1875 inches below the target. This is consistent
with the large native grid but does not establish a universal tie-break. In the
two-target fan-out, Visio selects a vertical-first one-bend path over an
unobstructed horizontal-first alternative of equal length and bend count.
`jump-three-crossings`' endpoint displacement remains separately unexplained.

The three preregistered hold-outs are four horizontal connectors with gap 3,
seven vertical connectors with gap 0.75, and five horizontal connectors with gap
1.25, all at page spacing 0.125 and RouteStyle 1. The script and numerical
predictions are frozen before their VM generation. All sixteen predicted central-cut coordinates match exactly (zero error,
1e-6-inch tolerance). This validates interpolation within the restricted
fixture family, not the unresolved general-rule behavior above.


Measured coordinates in connector creation order are below. Let
`L = [1.6875, 1.625, 1.5625, 1.65625, 1.59375, 1.53125, 1.671875,
1.640625, 1.609375, 1.578125]`. `L[:k]` means its first k entries.

| Witness family | Connectors | Clear gap (in) | Page LineToLine X/Y (in) | Measured central-cut coordinates (in) |
| --- | ---: | --- | --- | --- |
| Horizontal rows, each count independently | 2, 3, 5, 10 | 3 and 0.75 | 0.125 | L[:N-1], followed by 4.75 or 2.5 respectively |
| Vertical columns, each count independently | 2, 3, 5, 10 | 3 and 0.75 | 0.125 | L[:N] |
| Alternating directions | 5 | 3 | 0.125 | 1.6875, 4.3125, 1.625, 4.1875, 4.75 |
| Alternating directions | 5 | 0.75 | 0.125 | 1.6875, 2.0625, 1.625, 1.9375, 2.5 |
| Horizontal spacing variant | 5 | 3 and 0.75 | 0.25 | L[:4], followed by 4.75 or 2.5 respectively |
| Horizontal default RouteStyle | 3 | 3 and 0.75 | 0.125 | L[:2], followed by 4.75 or 2.5 respectively |
| Frozen horizontal hold-out | 4 | 3 | 0.125 | 1.6875, 1.625, 1.5625, 4.75 |
| Frozen vertical hold-out | 7 | 0.75 | 0.125 | L[:7] |
| Frozen intermediate-gap hold-out | 5 | 1.25 | 0.125 | 1.6875, 1.625, 1.5625, 1.65625, 3 |

The two-target fan-out starts at (2.25,1.5) and (2.375,1.5), with horizontal
legs at y=4.75 and y=4.625. The six-target fan-out begins, in connector order,
at (6,1.25), (6,1.375), (6.25,1.5), (6.375,1.5), (6.5,1.25) and (6.5,1.375).
Their main horizontal legs are at y=1.25, 1.375, 4.75, 4.625, 1.25 and 1.375.
Equal coordinates in this list do not imply overlap: these legs occupy disjoint
x intervals. Neither fan-out has a shared centerline interval. These are measured
ports and paths, not outputs of the restricted predictor.

For square obstacle grids of side N=3,5,7 at a two-inch pitch, the measured path
is (1.5,1.25), (1.6875,1.25), (1.6875,2N-1.1875),
(2N-0.75,2N-1.1875), (2N-0.75,2N-1). The current router differs on all three.
The private Brief 34 archive retains all drawings, PNGs, full COM dumps,
Connects, generator inputs, frozen predictions and per-cell differences.
