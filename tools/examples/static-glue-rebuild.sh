#!/usr/bin/env bash
# Recreate the static glue witness with named engine commands.
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4}"
mkdir -p "$output_dir"
if [[ -n "${DIAGRAMCRAFT_CLI:-}" ]]; then cli=("$DIAGRAMCRAFT_CLI"); else cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p diagramcraft-cli --); fi
"${cli[@]}" run \
  --cmd 'file.new={}' \
  --cmd 'shape.draw.rect={"x":1,"y":6,"width":1.5,"height":1.5}' \
  --cmd 'shape.text={"id":1,"text":"A"}' \
  --cmd 'shape.draw.rect={"x":5,"y":6,"width":1.5,"height":1.5}' \
  --cmd 'shape.text={"id":2,"text":"B"}' \
  --cmd 'shape.connection.add={"id":1,"x":"Width*0.5","y":"Height*0.5"}' \
  --cmd 'shape.connection.add={"id":2,"x":"Width*0.5","y":"Height*0.5"}' \
  --cmd 'master.drop={"master":2,"x":0,"y":0}' \
  --cmd 'connect.static={"id":3,"end":"begin","target":1,"point":"Connections.X1"}' \
  --cmd 'connect.static={"id":3,"end":"end","target":2,"point":"Connections.X1"}' \
  --save "$output_dir/static-glue.vsdx"
"${cli[@]}" sheet "$output_dir/static-glue.vsdx" > "$output_dir/static-glue.sheet.tsv"
python3 "$repo_dir/tools/examples/check.py" static "$output_dir"
