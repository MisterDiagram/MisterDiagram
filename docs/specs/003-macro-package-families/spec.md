# Macro-enabled stencil and template imports

Issue: https://github.com/charles-hood/DiagramCraft/issues/3

## Requirements

Accept .vssm and .vstm alongside the four supported Visio ZIP package families.
Recognize their exact main-part content types through OPC relationships, not
through the extension alone. CLI info must admit both extensions. Import must
set macros_removed and keep the existing VBA-removal behavior.

## Acceptance

- CLI info succeeds for all six extensions and rejects unrelated main types.
- Import accepts both new main types, strips the conventional VBA payload and
  preserves contributor-authored non-VBA content.
- Synthetic package tests and all cargo xtask ci gates pass.

## Out of scope

Macro execution, package writing, and renamed VBA classification (separate PR #5).
