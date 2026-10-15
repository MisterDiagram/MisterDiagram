#!/usr/bin/env bash
# Two contributor-original rectangles and a connector from the document stencil.
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4}"
mkdir -p "$output_dir"
if [[ -n "${DIAGRAMCRAFT_CLI:-}" ]]; then cli=("$DIAGRAMCRAFT_CLI"); else cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p diagramcraft-cli --); fi
"${cli[@]}" run \
  --cmd 'file.new={}' \
  --cmd 'shape.draw.rect={"x":1,"y":6,"width":1.5,"height":1.5}' \
  --cmd 'shape.draw.rect={"x":5,"y":6,"width":1.5,"height":1.5}' \
  --cmd 'master.drop={"master":2,"x":0,"y":0}' \
  --cmd 'connect.dynamic={"id":3,"end":"begin","target":1}' \
  --cmd 'connect.dynamic={"id":3,"end":"end","target":2}' \
  --save "$output_dir/two-boxes-glued.vsdx"
"${cli[@]}" convert "$output_dir/two-boxes-glued.vsdx" "$output_dir/two-boxes-glued.png"
