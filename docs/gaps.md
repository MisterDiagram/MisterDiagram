# Gaps

Status: pre-alpha, Phase 0 in progress. No parity milestone claimed.

## G1: No reader yet

Phase 1 implements the Visio document model reader and ShapeSheet evaluator. Phase 0 info
inspects the OPC container only. Dormant CAD geometry algorithms resume in Phase 2.

Phase 0 estimate remains 20 to 30 hours; re-estimate when corpus measurements are available.

## Phase 0 open work

- Visio acceptance: accept.ps1 stalled in Documents.Open on basic/0001-probe.vsdx twice, including after a VM restart. No acceptance PNG, cell dump or part list was produced.
- Shipped dump: 104/1181 files complete after four hours; all 1181 files copied (725 stencils, 456 templates). Resume in later background sessions. Charles explicitly excludes this remaining dump coverage from M0 blockers.
