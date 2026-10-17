#!/usr/bin/env bash
# Contributor-original office network, MIT OR Apache-2.0. Run from the repository root.
set -euo pipefail
cli="${MISTERDIAGRAM_CLI:-target/debug/misterdiagram}"
out="${1:-target/office-network}"; image="${2:-docs/images/office-network.png}"
mkdir -p "$out" "$(dirname "$image")"
args=(--cmd 'page.size={"width":10,"height":11}' --cmd 'shape.draw.rect={"x":0,"y":0,"width":1.6875,"height":0.625}' --cmd 'format.line={"id":1,"weight":0.03125}' --cmd 'format.text={"id":1,"size":10,"align":1}' --cmd 'format.fill={"id":1,"color":"#E8F2F0"}' --cmd 'master.new_from_shape={"id":1,"name":"Office node"}' --cmd 'shape.delete={"id":1}')
names=('Sky Internet' 'Alder Firewall' 'Birch Router' 'Cedar Core' 'Elm Access' 'Fir Access' 'Grove Server' 'Hazel Wi-Fi' 'Iris PC' 'Juniper PC')
for i in "${!names[@]}"; do
  args+=(--cmd 'master.drop={"master":"Office node","x":0,"y":0}' --cmd "shape.text={\"id\":$((i+1)),\"text\":\"${names[$i]}\"}")
done
args+=(--cmd 'layout.rows={"ids":[1,2,3,4,7,5,6,8,9,10],"counts":[1,1,1,1,4,2],"region":{"x":0,"y":0.5,"width":10,"height":10},"gutter_x":0.5,"gutter_y":0.75}')
links=('1 2' '2 3' '3 4' '4 5' '4 6' '4 7' '6 8' '5 9' '6 10'); id=11
for link in "${links[@]}"; do
  read -r source target <<< "$link"
  args+=(--cmd 'master.drop={"master":2,"x":0,"y":0}' --cmd "connect.sides={\"id\":$id,\"source\":$source,\"target\":$target}")
  ((id+=1))
done
args+=(--cmd 'layout.fan={"hub":4,"spokes":[7,5,6]}')
"$cli" run "${args[@]}" --save "$out/office-network.vsdx"
"$cli" convert "$out/office-network.vsdx" "$image"
