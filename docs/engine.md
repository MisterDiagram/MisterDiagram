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

Geometry, topology and uncertain formula effects cancel the fast exclusion proof,
so their evaluated dependencies are still checked. M5.71 selects connector writes
by reason within that transaction:

1. Changed glue, a changed glued target or ancestor transform, or changed endpoint
   cells selects that connector. ConFixedCode 2 retains its path and updates only
   terminal coordinates. Codes 1 and 3 retain their existing glue behavior.
2. A placeable shape whose placement bounds changed selects only ConFixedCode 0
   connectors whose saved route polyline intersects its new bounds with routing
   clearance. ObjType 1 is placeable; zero uses the existing glue promotion rule.
   A group flagged 8 (its members are placeable) is not an obstacle itself; its
   placeable members block and select routes through their own page scopes, and
   they move with the group. Moving an obstacle away or moving an unrelated label
   does not request a reroute.
3. Explicit `route.reroute`, layout commands and page routing policy changes keep
   their existing routing extent, including explicit lane assignment. Page policy
   includes RouteStyle, LineJump*, PlowCode, LineToNode*, LineToLine*, BlockSize*,
   AvoidPageBreaks and PageWidth/Height.

Unrequested routes are immutable context during ordinary edits. They are not
speculatively solved, assigned lanes or given new capacity outcomes. The jump pass
uses both old and new crossings and includes unglued ObjType 2 connectors. It
changes only routed connectors and neighbors whose crossings changed; ObjType 0
lines are excluded. Untouched Geometry, formulas, controls and complete Shape XML
remain as opened. The writer copies equal Shape subtrees from their original byte
spans only when their namespace context is unchanged, within existing XML limits.
Namespace copy/comparison work is bounded, and UTF-16 pages keep their encoding.
Undo, redo and rollback retain the existing document-identity validation contract;
placement evidence is local to preparation, with no persistent signature cache.

The same transaction may reuse up to 4,096 numeric predecessor endpoint entries
when predecessor evaluation has no assignment or contextual effects. Saturation
falls back to normal evaluation. Current endpoints always evaluate with active
glue hooks, so indirect endpoint and trigger references remain observable.

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

## Connector lane observations and lane pass (M5.39, M5.40)

Brief 34 observes Visio build 16.0.20430.20182 on contributor-authored synthetic
rows, columns, fan-outs and obstacle grids. Generated drawings and exports remain
in the private corpus. Charles adopted the restricted recurrence as the lane rule in Brief 34b:
exact on the restricted family, approximation elsewhere. The observations do
not establish Visio's general lane algorithm.

The public [LineToLineX reference](https://learn.microsoft.com/en-us/office/client-developer/visio/linetolinex-cell-page-layout-section)
and [LineToLineY reference](https://learn.microsoft.com/en-us/office/client-developer/visio/linetoliney-cell-page-layout-section)
define horizontal and vertical connector clearance. They do not specify lane
ordering or a minimum-spacing fallback. In both measured five-connector cases,
changing those page cells from 0.125 to 0.25 inches leaves the lane coordinates
unchanged. LineToNode remains 0.125. The experiment does not establish which
of these settings determines the initial 0.1875-inch displacement.

The implemented rule describes the positive-direction two-row/column
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

This rule does not explain the whole discovery set. Alternating directions
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


Further private controls falsify two attempted generalizations. With the native
300-box grid's target bottoms fixed at y=28, changing only target height from
0.5 to either 0.25 or 1 inch leaves all ten lane coordinates unchanged. A proposed
band whose width equals target height misses coordinates by as much as 0.40625
and 0.625 inch respectively. A fixed half-inch band still fits that sequence,
but its boundary is not derived from an obstacle or a supported geometric rule.

In the alternating-direction five-connector control, page LineToLine X/Y=0.0625
produces `[1.6875, 4.3125, 1.625, 4.25, 4.75]`. The attempted symmetric insertion
predicts 4.375 for the fourth connector, an error of 0.125 inch. At page spacing
0.25, that drawing retains `[1.6875, 4.3125, 1.625, 4.1875, 4.75]`. Thus the
unchanged original 0.125/0.25 lanes do not establish that page spacing is ignored.

Routing state also matters. Explicitly setting
[ConFixedCode](https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section)
to 0 before the same nudge and Layout procedure yields, for the positive-direction
five-connector drawing, `[1.5625, 1.6875, 1.625, 1.65625, 4.75]` at spacing 0.125
and `[1.5625, 1.625, 1.6875, 1.65625, 4.75]` at spacing 0.25. Both final dumps
return ConFixedCode 6. These controls expose a state-dependent ordering that the
restricted creation-order recurrence does not predict. They are additional
observations; none replaces the original witness or hold-out.

Without the explicit routing-state change, initial glue, source nudge, first
Layout and second Layout retain the measured routes. Halving or doubling
LineToNode also leaves those particular routes unchanged. A trial originally
labelled last-source nudge actually moved an unconnected box; its record is
retained under that corrected description. A separate corrected last-source
trial also retains the original lanes. Three optional attempts to write spacing
through No Style failed with COM exceptions before producing drawings and supply
no evidence about style control.

M5.39 is incomplete. The original hold-outs have zero residual, comprising
fourteen in-gap lane coordinates and two explicitly outside-gap crossings.
They do not validate the unresolved band, rerouting order, opposite-direction
behavior or fan-out ports. Brief 34b separately adopts the restricted recurrence
as the production approximation described below. It does not claim a general
Visio explanation or change the equal-length route tie choice.

### Lane implementation and limits

After the ordinary router, effective styles 0 and 1 participate in shared,
collinear bend-to-bend trunk groups. ShapeRouteStyle overrides the page style,
except zero inherits RouteStyle. Shape IDs determine creation order. Each trunk
uses the physical channel edge facing its begin shape, even for opposite-direction
connectors. Occupied or physically blocked positions are skipped in the sequence
above, down to 1/1024 inch. A lane must remain strictly outside every
non-permeable shape; Visio's witnessed lanes may be closer than the ordinary
router's inflated clearance. LineToLine does not scale this adopted band.

Other styles, unshared routes and capacity fallback routes retain their previous
geometry and clearance. Their segments still reserve occupied positions. Moving
a bend can lengthen an endpoint leg into another connector. In the general-case
approximation, a bounded internal jog shortens that leg on the same line while
preserving its glue point. Subpath breaks are preserved, and new jogs cannot
self-overlap. Failure restores the entire connector, checks dependent lane
assignments and reports a route warning naming the connector.

Lane work is bounded by 500,000 retained points, 16,384 candidate trunks per
page and four million comparisons/candidate visits. Band enumeration stops at
1/1024 inch. These are lane-pass bounds; the existing obstacle/grid/corridor
limits remain unchanged. Warning fallbacks retain the pre-lane route.

The generated CI tests compare the actual in-gap, bend-to-bend segment coordinates
against the contributor's numeric M5.39 observations, at 1e-6 inch. They cover
20 restricted cases and three hold-outs. A terminal leg crossing the central cut
is not a trunk: the existing vertical endpoint/tie choice can differ from Visio
while its in-gap trunk has the exact required coordinate. Horizontal final
connectors retain the documented outside-gap tie difference. Full-route parity
is not implied. The 100-box generated page checks ten distinct lane heights,
all connector pairs and physical box interiors.


### Shared-segment inspection (M5.42)

`layout.check` adds rule h for collinear straight-segment overlaps longer than
1e-6 inch. Effective Flowchart/Tree styles 5, 6, 7, 8, 12, 13, 14, 15 and the
separately witnessed office style 17 allow shared buses when both connectors
qualify. The named fan-out exception requires matching actual glue targets and
terminal points. The checker is read-only and does not change routing.

The accepted office VM observation nudged all ten 2-D shapes by 0.01 inch and
back. Visio preserved nine routes within 1e-6 inch, all 18 Connects and five
shared intervals, including the style 17 trunk of connectors 14/15 at y=3.75,
length 0.875 inch. This supplies the SimpleNS exemption evidence independently
of the restricted style 0/1 lane rule.

The explicit `sheet.set ShapeRouteStyle` command requests a targeted reroute,
retaining the established capacity retry and one-step undo behavior.
