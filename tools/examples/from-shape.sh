#!/usr/bin/env bash
# Contributor-original master recipe, https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4b-closure}"
mkdir -p "$output_dir"
if [[ -n "${DIAGRAMCRAFT_CLI:-}" ]]; then cli=("$DIAGRAMCRAFT_CLI"); else cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p diagramcraft-cli --); fi
"${cli[@]}" run --open "$repo_dir/corpus/blank/blank.vsdx" \
 --cmd 'shape.draw.rect={"x":1,"y":6,"width":1.5,"height":0.75}' \
 --cmd 'shape.text={"id":1,"text":"Switch\nOriginal"}' \
 --cmd 'format.text={"id":1,"align":1}' \
 --cmd 'master.new_from_shape={"id":1,"name":"Switch"}' \
 --cmd 'master.drop={"master":"Switch","x":2,"y":6}' \
 --cmd 'master.drop={"master":"Switch","x":5,"y":6}' \
 --cmd 'shape.delete={"id":1}' \
 --cmd 'shape.draw.rect={"x":1,"y":8,"width":3,"height":0.6}' \
 --cmd 'shape.text={"id":4,"text":"Single run"}' \
 --cmd 'format.line={"id":4,"pattern":0}' \
 --cmd 'format.fill={"id":4,"pattern":0}' \
 --save "$output_dir/from-shape.vsdx"
"${cli[@]}" sheet "$output_dir/from-shape.vsdx" > "$output_dir/from-shape.sheet.tsv"
status=0
python3 "$repo_dir/tools/oracle/diff_dump.py" "$output_dir/from-shape.sheet.tsv" "$repo_dir/corpus/generated/master/from-shape.sheet.tsv" \
 --ignore-hooks "$repo_dir/crates/sheet/tests/hooks.txt" --ignore-volatile "$repo_dir/crates/sheet/tests/volatile.txt" > "$output_dir/from-shape-check.txt" || status=$?
tail -n +2 "$output_dir/from-shape-check.txt" > "$output_dir/from-shape.diff"
exit "$status"
