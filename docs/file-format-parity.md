# File format parity

Status: pre-alpha, Phase 0 container inspection only.

## Measured

Visio 16.0 build 20430 (16.0.20430.20146). Generated corpus: 204 drawings,
827857 ShapeSheet rows, all six requested exports; PNG densities 96 dpi. Blank: 3385 cells.
Shipped source files: 1181, with 104 completed master dumps; remaining dumps deferred.
Visio acceptance remains unverified after two Documents.Open timeouts.
ENTITY_M/U.vssx and OBJREL_M/U.vssx use the published 2011/1/core XML namespace;
the other measured files use 2012/main. Container inspection accepts both exact URIs.

## Observed, undocumented

## Functions not implemented

All ShapeSheet evaluation is deferred to Phase 1. The unit and volatile lists are contracts,
not claims that their functions are implemented.
