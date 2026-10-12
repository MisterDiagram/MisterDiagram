# VBA part classification during import

Issue: https://github.com/charles-hood/DiagramCraft/issues/2

## Problem and requirements

Import discards a part called vbaProject.bin but preserves a renamed part with
VBA project content type. The returned Document must discard VBA project parts
identified by OPC content type as well as the existing conventional basename.

Set macros_removed when a part is discarded, even if the main document content
type is not macro-enabled. Preserve unrelated opaque bytes and original part
names. Keep the existing macro-enabled main-part flag behavior.

## Acceptance

- Renamed VBA project parts with default or override content types are omitted.
- Conventional names remain omitted, including mixed ASCII case.
- The removal flag is true whenever such a part is discarded.
- Non-VBA binary parts remain byte-identical, including other .bin parts.
- Synthetic package regressions and cargo xtask ci pass.

## Out of scope

Executing or interpreting VBA, changing the preserved raw XML/relationship
tables, writing sanitized packages, accepting new macro-enabled main content
types, or detecting mislabeled unknown bytes without package metadata.
