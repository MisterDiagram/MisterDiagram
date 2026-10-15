#!/usr/bin/env bash
# The orthogonal router creates Geometry row 3 before the explicit cell overrides.
# Contributor-original recreation of the synthetic probe using engine commands.
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4}"
mkdir -p "$output_dir"
if [[ -n "${DIAGRAMCRAFT_CLI:-}" ]]; then
    cli=("$DIAGRAMCRAFT_CLI")
else
    cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p diagramcraft-cli --)
fi
"${cli[@]}" run \
  --cmd 'file.new={}' \
  --cmd 'shape.draw.rect={"x":1,"y":8,"width":2,"height":1.5}' \
  --cmd 'shape.text={"id":1,"text":"Switch A"}' \
  --cmd 'shape.draw.rect={"x":5,"y":8,"width":2,"height":1.5}' \
  --cmd 'shape.text={"id":2,"text":"Switch B"}' \
  --cmd 'shape.draw.ellipse={"x":3,"y":4,"width":2,"height":1.5}' \
  --cmd 'shape.text={"id":3,"text":"Storage"}' \
  --cmd 'master.drop={"master":2,"x":0,"y":0}' \
  --cmd 'connect.dynamic={"id":4,"end":"begin","target":1}' \
  --cmd 'connect.dynamic={"id":4,"end":"end","target":2}' \
  --cmd 'master.drop={"master":2,"x":0,"y":0}' \
  --cmd 'connect.dynamic={"id":5,"end":"begin","target":2}' \
  --cmd 'connect.dynamic={"id":5,"end":"end","target":3}' \
  --cmd 'shape.text={"id":5,"text":"32G"}' \
  --cmd 'sheet.set={"id":2,"name":"FillForegnd","formula":"RGB(204,0,0)"}' \
  --cmd 'layer.add={"name":"Fabric"}' \
  --cmd 'sheet.set={"id":5,"name":"BeginX","formula":"_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)","value":6}' \
  --cmd 'sheet.set={"id":5,"name":"BeginY","formula":"_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)","value":8}' \
  --cmd 'sheet.set={"id":5,"name":"EndX","formula":"_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)","value":5}' \
  --cmd 'sheet.set={"id":5,"name":"EndY","formula":"_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)","value":4.75}' \
  --cmd 'sheet.set={"id":5,"name":"Width","formula":"GUARD(EndX-BeginX)","force":true}' \
  --cmd 'sheet.set={"id":5,"name":"Height","formula":"GUARD(EndY-BeginY)","force":true}' \
  --cmd 'sheet.set={"id":5,"name":"Angle","formula":"GUARD(0DA)","force":true}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.X1","value":0}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.Y1","value":0}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.X2","value":0}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.Y2","value":-3.25}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.X3","value":-1}' \
  --cmd 'sheet.set={"id":5,"name":"Geometry1.Y3","value":-3.25}' \
  --cmd 'sheet.set={"id":5,"name":"Controls.TextPosition","value":0}' \
  --cmd 'sheet.set={"id":5,"name":"Controls.TextPosition.Y","value":-2.125}' \
  --save "$output_dir/probe-rebuild.vsdx"
"${cli[@]}" sheet "$output_dir/probe-rebuild.vsdx" > "$output_dir/probe-rebuild.sheet.tsv"
python3 "$repo_dir/tools/examples/check.py" probe "$output_dir"
