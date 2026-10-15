#!/usr/bin/env bash
# Contributor-original dual-fabric SAN, built only through engine commands.
set -euo pipefail
repo_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
output_dir="${1:-$repo_dir/corpus/_state/m4}"
mkdir -p "$output_dir"
if [[ -n "${DIAGRAMCRAFT_CLI:-}" ]]; then cli=("$DIAGRAMCRAFT_CLI"); else cli=(cargo run -q --manifest-path "$repo_dir/Cargo.toml" -p diagramcraft-cli --); fi
python3 "$repo_dir/tools/examples/san-topology.py" "$output_dir" "${cli[@]}"
