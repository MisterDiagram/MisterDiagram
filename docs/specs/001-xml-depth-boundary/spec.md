# Consistent XML depth boundary

Issue: https://github.com/charles-hood/DiagramCraft/issues/1

## Problem and requirements

The OPC parser currently counts open elements toward MAX_DEPTH but admits a
self-closing child beyond that boundary. Element depth must be independent of
whether an empty element is serialized as `<leaf/>` or `<leaf></leaf>`.

Accept a path containing exactly MAX_DEPTH elements; reject a path containing
MAX_DEPTH + 1. Retain MAX_NODES and return OpcError::Xml on invalid input.
The public VSDX parse_xml entry point must inherit the same behavior.

## Acceptance

- Paired element and self-closing forms pass at the limit and fail above it.
- The boundary holds for OPC and VSDX without a private corpus.
- `cargo xtask ci` passes.

## Out of scope

Changing limits, namespace rules, UTF-16 decoding, rendering or private oracles.
