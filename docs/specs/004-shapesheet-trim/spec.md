# ShapeSheet TRIM

Issue: https://github.com/charles-hood/DiagramCraft/issues/7

## Requirements

Evaluate unary TRIM instead of returning Unsupported. Remove leading/trailing
whitespace and collapse an internal whitespace run to one ASCII space. Use the
explicit set in MS-VSDX v20150316 section 2.5.3.166, including U+180E; unrelated
Unicode separators remain unchanged. Reuse existing text coercion and errors.
Keep the existing 65536-byte text bound and reject wrong arity.

## Acceptance

- Verify the public example, every listed whitespace scalar and preserved text.
- Cover empty/all-space inputs, references, coercion, errors and resource bounds.
- Recalculate an imported cell with a deliberately stale saved value.
- Pass all cargo xtask ci gates, including wasm.

## Out of scope

Locale case conversion, other string functions, changes to Unicode indexing,
frozen hooks or private comparison thresholds. Reference conformance does not
establish application-runtime parity.
