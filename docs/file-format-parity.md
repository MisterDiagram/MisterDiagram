# File format parity

Status: pre-alpha, Phase 0 container inspection only.

## Measured

The 104 shipped dumps were produced by the non-resumable staged corpus/_vm/dump-shipped.ps1,
not the current committed script; the interrupted TSV was renamed to .partial manually.
The committed resume path is verified only by tools/tests/shipped-resume.ps1 until its first VM run.

Visio 16.0 build 20430 (16.0.20430.20146). Generated corpus: 204 drawings,
827857 ShapeSheet rows, all six requested exports; PNG densities 96 dpi. Blank: 3385 cells.
Shipped source files: 1181, with 104 completed master dumps; remaining dumps deferred.
Visio acceptance verified on 2026-10-12 for basic/0001-probe.vsdx and blank/blank.vsdx:
both OPEN OK and ACCEPT OK, 9257 cells, zero errors, with PNG, sheet TSV and part list each.
Normal VISIO.EXE launch and GetActiveObject attachment resolve the earlier automation-launch
open hang; its underlying cause remains unknown. The shared-route health probe completed
in 7.822 seconds, including shipped-template OpenEx, blank Add, both closes and Quit.
ENTITY_M/U.vssx and OBJREL_M/U.vssx use the published 2011/1/core XML namespace;
the other measured files use 2012/main. Container inspection accepts both exact URIs.

## Observed, undocumented

Visio output contradicts the Cell_Type U table
(https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/6f23bcc4-af93-4023-a380-3e78a228e166):
it writes additional U codes absent from that table. Local, unmodified witnesses:

- `NURBS`: `corpus/ms-shipped/1033/ARROWS_M.vssx#visio/masters/master10.xml`.
- `POLYLINE`: `corpus/ms-shipped/1033/APPL_M.vssx#visio/masters/master17.xml`.
- `GUID`: `corpus/ms-shipped/1033/BASIC_GANTTCHART_U.vstx#visio/pages/page1.xml`.
- `MULTIDIM`: `corpus/ms-shipped/1033/ADS_M.vssx#visio/masters/master3.xml`.
- `NUM`: `corpus/ms-shipped/1033/EVIDENCE_U.vssx#visio/document.xml`.

## Functions not implemented

All ShapeSheet evaluation is deferred to Phase 1. The unit and volatile lists are contracts,
not claims that their functions are implemented.
