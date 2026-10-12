# Plan

Register TRIM in the canonical function registry and existing text dispatcher.
Validate one argument and obtain its text through the existing capped helper.
Use an explicit whitespace predicate and a single bounded pass with pending-gap
state. Output cannot exceed input UTF-8 bytes because each collapsed whitespace
run becomes at most one ASCII space. No new dependencies or public API.

Add public formula tests and a contributor-authored OPC package test that
imports Input/Label document cells and evaluates TRIM(Input). Reference:
https://learn.microsoft.com/en-us/office/client-developer/visio/trim-function
The precise scalar list is from the public MS-VSDX v20150316 PDF, section
2.5.3.166; no product assets or implementation are used.
