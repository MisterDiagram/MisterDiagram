# Contributing

Thank you for looking at DiagramCraft. Pull requests are welcome, and the first ones were
merged within a day of review. A few rules keep the project honest and keep review fast.

## Clean room, no exceptions

- Test inputs are contributor-authored: synthetic XML strings, in-memory packages built with
  the test helpers, or drawings produced by this repo's own scripts. Never a Microsoft stencil,
  template, icon or font, and never a file saved by Visio.
- Visio is observed only as a black box, on drawings our scripts made, in a licensed copy.
- Formats and functions come from Microsoft's published references. Every element, cell and
  function cites its reference page in its doc comment; `cargo xtask sources` regenerates
  docs/sources.md and CI checks it.
- If you consult any non-Microsoft documentation about Visio or its formats, add a line to
  docs/provenance.md section 3 in the same commit. Never open the source of libvisio or any
  other Visio implementation while working here.
- A PR without the private oracle corpus can claim reference coverage. Measured parity
  against Visio is confirmed by the maintainer, who runs the corpus. Say which one you claim.

## Shape of a change

- One issue per PR, one PR at a time, small enough to review in one sitting.
- Plain commit subjects. The `M<phase>.<n>:` prefix is reserved for milestone commits by the
  maintainer.
- Gaps and follow-ups go in docs/gaps.md; decisions in plan/STATUS.md (gitignored, so
  summarise in the PR). Do not add a docs/specs tree.
- Never edit an oracle output, a threshold, a floor or a denominator to make a test pass.
- `cargo xtask ci` green, with the hostile-input tests extended for every new parser path.
- The maintainer may rebase your branch onto master before merging; authorship is kept.

## Where things are

AGENTS.md has the rules every agent and contributor works under, docs/architecture.md the
layers, docs/development.md the build gates and the oracle VM workflow, ROADMAP.md the
milestones and the numbers each one has to reach.
