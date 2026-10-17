#!/usr/bin/env bash
# Contributor-original recipe mirroring the M4.19 COM observation.
# https://learn.microsoft.com/en-us/office/client-developer/visio/shape-data-section
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4b-closure}"
mkdir -p "$output_dir"
if [[ -n "${MISTERDIAGRAM_CLI:-}" ]]; then cli=("$MISTERDIAGRAM_CLI"); else cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p misterdiagram-cli --); fi
"${cli[@]}" run --open "$repo_dir/corpus/blank/blank.vsdx" \
 --cmd 'shape.draw.rect={"x":1,"y":6,"width":1.5,"height":1.5}' \
 --cmd 'shape.text={"id":1,"text":"A"}' \
 --cmd 'shape.draw.rect={"x":5,"y":6,"width":1.5,"height":1.5}' \
 --cmd 'shape.text={"id":2,"text":"B"}' \
 --cmd 'shape.data.define={"id":1,"name":"Name","type":0}' \
 --cmd 'shape.data.set={"id":1,"name":"Name","value":"Switch A"}' \
 --cmd 'shape.data.define={"id":1,"name":"DomainID","type":2}' \
 --cmd 'shape.data.set={"id":1,"name":"DomainID","value":42}' \
 --cmd 'shape.data.define={"id":1,"name":"Fabric","type":1,"format":"A;B"}' \
 --cmd 'shape.data.set={"id":1,"name":"Fabric","value":"A"}' \
 --cmd 'shape.data.define={"id":1,"name":"Installed","type":5,"format":"yyyy-MM-dd"}' \
 --cmd 'shape.data.set={"id":1,"name":"Installed","value":45000}' \
 --save "$output_dir/property-rows.vsdx"
"${cli[@]}" sheet "$output_dir/property-rows.vsdx" > "$output_dir/property-rows.sheet.tsv"
status=0
python3 "$repo_dir/tools/oracle/diff_dump.py" "$output_dir/property-rows.sheet.tsv" "$repo_dir/corpus/generated/data/property-rows.sheet.tsv" \
 --ignore-hooks "$repo_dir/crates/sheet/tests/hooks.txt" --ignore-volatile "$repo_dir/crates/sheet/tests/volatile.txt" > "$output_dir/property-rows-check.txt" || status=$?
# Retain only actual differences; the successful report remains alongside it.
tail -n +2 "$output_dir/property-rows-check.txt" > "$output_dir/property-rows.diff"
exit "$status"
