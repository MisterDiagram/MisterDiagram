#!/bin/bash
# Execute a Visio oracle as the interactive user. Never run Office COM as SYSTEM.
set -euo pipefail
VM='Win11 Enterprise (ARM)'
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
if [ "${1:-}" = --stop ]; then exec prlctl stop "$VM"; fi
if [ "${1:-}" = --check-task ]; then
    exec prlctl exec "$VM" powershell -NoProfile -Command 'Get-ScheduledTask -TaskName visioprobe -ErrorAction SilentlyContinue | Measure-Object | Select-Object -ExpandProperty Count'
fi
SCRIPT=${1:?usage: run-in-vm.sh script.ps1 [timeout-seconds] [accept-files...]}
shift
LIMIT=1800
if [ "$#" -gt 0 ]; then
    case "$1" in *[!0-9]*|'') ;; *) LIMIT=$1; shift;; esac
fi
if [ ! -f "$SCRIPT" ] && [ -f "$ROOT/tools/oracle/$SCRIPT" ]; then SCRIPT="$ROOT/tools/oracle/$SCRIPT"; fi
NAME=$(basename "$SCRIPT" .ps1)
case "$NAME" in ''|*[!a-zA-Z0-9_-]*) echo 'invalid script name' >&2; exit 2;; esac
case "$LIMIT" in ''|*[!0-9]*) echo 'timeout must be positive seconds' >&2; exit 2;; esac
[ "$LIMIT" -gt 0 ] || exit 2
[ -f "$SCRIPT" ] || { echo "script missing: $SCRIPT" >&2; exit 2; }
SCRIPT=$(cd "$(dirname "$SCRIPT")" && pwd)/$(basename "$SCRIPT")
STAGE="$ROOT/corpus/_vm"
OUT="$STAGE/out/$NAME"
mkdir -p "$STAGE"
case "$NAME" in
accept|accept-visible)
    python3 - "$ROOT" "$@" <<'LIST'
from pathlib import Path
import sys
root = Path(sys.argv[1]).resolve()
files = sys.argv[2:]
if not files or len(files) > 512:
    raise SystemExit('accept requires 1 to 512 Mac file paths under corpus/')
paths = []
names = set()
for value in files:
    path = Path(value).resolve(strict=True)
    if not path.is_file() or not path.is_relative_to(root / 'corpus') or any(c in str(path) for c in '\r\n'):
        raise SystemExit('accept file must be a regular file under corpus/: ' + value)
    if path.stem.casefold() in names:
        raise SystemExit('duplicate acceptance basename: ' + path.stem)
    names.add(path.stem.casefold())
    relative = path.relative_to(root)
    paths.append('\\\\Mac\\Home\\Projects\\diagramcraft\\' + str(relative).replace('/', '\\'))
(root / 'corpus/_vm/accept-list.txt').write_text('\n'.join(paths) + '\n', encoding='utf-8')
LIST
    ;;
*) [ "$#" -eq 0 ] || { echo 'extra file arguments are only valid for accept' >&2; exit 2; };;
esac
mkdir -p "$OUT"
# Shipped TSVs are checkpoints across nights. Only completion/watchdog markers are stale.
if [ "$NAME" = dump-shipped ]; then
    rm -f "$OUT/DONE" "$OUT/CURRENT"
else
    find "$OUT" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
fi
if [ "$SCRIPT" != "$STAGE/$NAME.ps1" ]; then /bin/cp -f "$SCRIPT" "$STAGE/$NAME.ps1"; fi
if [ -f "$ROOT/tools/oracle/common.ps1" ]; then /bin/cp -f "$ROOT/tools/oracle/common.ps1" "$STAGE/common.ps1"; fi
WIN="\\\\Mac\\Home\\Projects\\diagramcraft\\corpus\\_vm\\$NAME.ps1"
# Bound host-side prlctl calls too: a stalled guest command must not bypass the watchdog.
bounded() {
    python3 - "${ORACLE_CALL_TIMEOUT:-25}" "$@" <<'PY'
import subprocess, sys
try:
    result = subprocess.run(sys.argv[2:], timeout=int(sys.argv[1]))
    sys.exit(result.returncode)
except subprocess.TimeoutExpired:
    print('prlctl host command timed out', file=sys.stderr)
    sys.exit(124)
PY
}
ps() { bounded prlctl exec "$VM" powershell -NoProfile -Command "$1"; }
ready=0
for attempt in 1 2; do
    state=$(prlctl list -a)
    printf '%s\n' "$state" >&2
    if printf '%s\n' "$state" | grep -F "$VM" | grep -q stopped; then
        prlctl start "$VM" >&2
    fi
    start=$SECONDS
    while [ $((SECONDS-start)) -lt 180 ]; do
        if ORACLE_CALL_TIMEOUT=$((180-SECONDS+start)) bounded prlctl exec "$VM" cmd /c echo ok > "$STAGE/ready.txt" 2>&1; then ready=1; break; fi
        sleep 3
    done
    [ "$ready" -eq 1 ] && break
    echo "VM readiness attempt $attempt failed" >&2
 done
[ "$ready" -eq 1 ] || { echo 'VM did not answer after one retry' >&2; exit 124; }
cleanup() { ps "Get-ScheduledTask -TaskName diagramcraft-oracle -ErrorAction SilentlyContinue | Unregister-ScheduledTask -Confirm:\$false -ErrorAction Stop; exit 0" >/dev/null 2>&1 || true; }
convert() {
    while IFS= read -r -d '' f; do
        if file "$f" | grep -q 'UTF-16'; then
            iconv -f UTF-16 -t UTF-8 "$f" > "$f.utf8"
            /bin/mv -f "$f.utf8" "$f"
        fi
    done < <(find "$OUT" -type f \( -name '*.tsv' -o -name '*.txt' -o -name '*.log' \) -print0)
}
trap cleanup EXIT
trap 'exit 130' INT TERM
ps "Get-ScheduledTask -TaskName visioprobe -ErrorAction SilentlyContinue | Unregister-ScheduledTask -Confirm:\$false -ErrorAction Stop; exit 0" >/dev/null
launch() {
    cleanup
    ps "\$ErrorActionPreference='Stop'; \$a=New-ScheduledTaskAction -Execute 'powershell.exe' -Argument '-NoProfile -ExecutionPolicy Bypass -File \"$WIN\"'; \$p=New-ScheduledTaskPrincipal -UserId charles -LogonType Interactive; \$s=New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Days 2); Register-ScheduledTask -TaskName diagramcraft-oracle -Action \$a -Principal \$p -Settings \$s -Force | Out-Null; Start-ScheduledTask -TaskName diagramcraft-oracle" >&2
}
logged=0
for attempt in 1 2; do
    launch
    start=$SECONDS
    while [ $((SECONDS-start)) -lt 180 ]; do
        if [ -s "$OUT/probe-log.txt" ] && [ "$OUT/probe-log.txt" -nt "$STAGE/$NAME.ps1" ]; then logged=1; break; fi
        sleep 2
    done
    [ "$logged" -eq 1 ] && break
    echo "No oracle log after launch attempt $attempt" >&2
 done
if [ "$logged" -ne 1 ]; then
    ps 'Stop-Process -Name VISIO -Force -ErrorAction SilentlyContinue' >/dev/null || true
    echo 'Scheduled task produced no log after one retry' >&2
    exit 124
fi
start=$SECONDS
while [ ! -f "$OUT/DONE" ]; do
    elapsed=$((SECONDS-start))
    if [ -f "$OUT/CURRENT" ]; then
        file_age=$(python3 - "$OUT/CURRENT" <<'AGE'
import os, sys, time
try:
    print(int(time.time() - os.stat(sys.argv[1]).st_mtime))
except FileNotFoundError:
    print(0)
AGE
)
        if [ "$file_age" -ge 1800 ]; then
            ps 'Stop-Process -Name VISIO -Force -ErrorAction SilentlyContinue' >/dev/null || true
            convert
            echo "Per-file oracle timeout: $OUT/CURRENT" >&2
            exit 124
        fi
    fi
    if [ "$elapsed" -ge "$LIMIT" ]; then
        ps 'Stop-Process -Name VISIO -Force -ErrorAction SilentlyContinue' >/dev/null || true
        convert
        echo "Oracle timed out after $elapsed seconds: $OUT" >&2
        exit 124
    fi
    sleep 2
 done
convert
cleanup
trap - EXIT
printf '%s\n' "$OUT"
