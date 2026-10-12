# Implementation plan

Keep depth and node validation at the OPC XML event boundary. Handle Start and
Empty together for the shared check, then dispatch the admitted element to the
stack or its parent. This removes the duplicated resource rule.

The depth of an incoming element is stack.len() + 1; reject it when stack.len()
is already MAX_DEPTH. Both event forms increment the node count once.
No public API or error variant changes.

Add table-driven boundary tests for both serialization forms at MAX_DEPTH - 1,
MAX_DEPTH and MAX_DEPTH + 1, plus a VSDX public-entry regression. Build strings
in the tests; no external document assets are needed.

Reference for the consuming format:
https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
