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
ACCEPT_OUTPUT=${ORACLE_ACCEPT_OUTPUT:-accept}
case "$ACCEPT_OUTPUT" in accept|baseline) ;; *) echo 'invalid acceptance output tree' >&2; exit 2;; esac
OUTPUT_NAME=$NAME
if [ "$NAME" = accept ]; then OUTPUT_NAME=$ACCEPT_OUTPUT; fi
OUT="$STAGE/out/$OUTPUT_NAME"
mkdir -p "$STAGE"
case "$NAME" in
accept)
    printf '%s\n' "$ACCEPT_OUTPUT" > "$STAGE/accept-output.txt"
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
dump-shipped)
    python3 - "$ROOT" "$@" <<'LIST'
from pathlib import Path
import sys
root = Path(sys.argv[1]).resolve()
files = sys.argv[2:]
manifest = root / 'corpus/_vm/dump-shipped-select.txt'
if not files:
    manifest.unlink(missing_ok=True)
else:
    if len(files) > 512:
        raise SystemExit('at most 512 shipped files may be selected')
    names = set()
    for value in files:
        path = Path(value).resolve(strict=True)
        if not path.is_file() or not path.is_relative_to(root / 'corpus/ms-shipped') or path.suffix.lower() not in {'.vssx', '.vstx'} or any(c in path.name for c in '\r\n'):
            raise SystemExit('selection must be a shipped stencil or template: ' + value)
        names.add(path.name.lower())
    manifest.write_text('\n'.join(sorted(names)) + '\n', encoding='utf-8')
LIST
    ;;
*) [ "$#" -eq 0 ] || { echo 'extra file arguments are only valid for accept or dump-shipped'  >&2; exit 2; };;
esac
if [ "$SCRIPT" != "$STAGE/$NAME.ps1" ]; then /bin/cp -f "$SCRIPT" "$STAGE/$NAME.ps1"; fi
/bin/cp -f "$ROOT/tools/oracle/common.ps1" "$STAGE/common.ps1"
/bin/cp -f "$ROOT/tools/oracle/visio-health.ps1" "$STAGE/visio-health.ps1"
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
# prlctl reconstructs a Windows command line. Encode the script so embedded
# diagnostic quotes and semicolons cannot become a second PowerShell command.
ps() {
    local encoded
    encoded=$(python3 - "$1" <<'ENCODE'
import base64, sys
print(base64.b64encode(sys.argv[1].encode('utf-16le')).decode('ascii'))
ENCODE
) || return 1
    bounded prlctl exec "$VM" powershell -NoProfile -EncodedCommand "$encoded"
}
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
cleanup() {
    ps "Get-ScheduledTask -TaskName misterdiagram-oracle -ErrorAction SilentlyContinue | Stop-ScheduledTask -ErrorAction SilentlyContinue; Get-ScheduledTask -TaskName misterdiagram-oracle -ErrorAction SilentlyContinue | Unregister-ScheduledTask -Confirm:\$false -ErrorAction Stop; exit 0" >/dev/null 2>&1 || true
}
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
    local win=$1 duration=$2
    cleanup
    ps "\$ErrorActionPreference='Stop'; \$a=New-ScheduledTaskAction -Execute 'powershell.exe' -Argument '-NoProfile -ExecutionPolicy Bypass -File \"$win\"'; \$p=New-ScheduledTaskPrincipal -UserId charles -LogonType Interactive; \$s=New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Seconds $duration); Register-ScheduledTask -TaskName misterdiagram-oracle -Action \$a -Principal \$p -Settings \$s -Force | Out-Null; Start-ScheduledTask -TaskName misterdiagram-oracle" >&2
}
# Run one staged script. A watchdog returns 124; the caller owns recovery.
run_one() {
    local name=$1 limit=$2
    local output_name=$name
    if [ "$name" = accept ]; then output_name=$ACCEPT_OUTPUT; fi
    local OUT="$STAGE/out/$output_name" win="\\\\Mac\\Home\\Projects\\diagramcraft\\corpus\\_vm\\$name.ps1"
    local started=$SECONDS launched=$SECONDS logged=0 attempt=1 file_age file_limit=1800
    # Large acceptance dumps need over 40 minutes; the overall caller budget bounds them.
    if [ "$name" = accept ]; then file_limit=$limit; fi
    mkdir -p "$OUT" || return 1
    if [ "$name" = dump-shipped ] || [ "$name" = accept ]; then
        rm -f "$OUT/DONE" "$OUT/CURRENT" || return 1
    else
        # Preserve each health observation before the next probe resets its output.
        if [ "$name" = visio-health ] && [ -f "$OUT/probe-log.txt" ]; then
            local archive
            mkdir -p "$STAGE/history" || return 1
            archive=$(mktemp -d "$STAGE/history/visio-health.XXXXXX") || return 1
            /bin/cp -R "$OUT/." "$archive/" || return 1
        fi
        find "$OUT" -mindepth 1 -maxdepth 1 -exec rm -rf {} + || return 1
    fi
    launch "$win" 172800 || return 1
    while [ ! -f "$OUT/DONE" ]; do
        if [ -s "$OUT/probe-log.txt" ] && [ "$OUT/probe-log.txt" -nt "$STAGE/$name.ps1" ]; then logged=1; fi
        if [ $((SECONDS-started)) -ge "$limit" ]; then
            echo "Oracle timed out after $((SECONDS-started)) seconds: $OUT" >&2
            return 124
        fi
        if [ "$logged" -eq 0 ] && [ $((SECONDS-launched)) -ge 180 ]; then
            if [ "$attempt" -eq 2 ]; then
                echo 'Scheduled task produced no log after one retry' >&2
                return 124
            fi
            echo 'No oracle log after launch attempt 1' >&2
            launch "$win" 172800 || return 1
            launched=$SECONDS; attempt=2
        fi
        if [ -f "$OUT/CURRENT" ]; then
            file_age=$(python3 - "$OUT/CURRENT" <<'AGE'
import os, sys, time
try:
    print(int(time.time() - os.stat(sys.argv[1]).st_mtime))
except FileNotFoundError:
    print(0)
AGE
) || return 1
            if [ "$file_age" -ge "$file_limit" ]; then
                echo "Per-file oracle timeout: $OUT/CURRENT" >&2
                return 124
            fi
        fi
        sleep 1
    done
    convert || return 1
    cleanup
}
# Quit must run in charles's interactive session, not in prlctl's SYSTEM session.
gentle_stop() {
    cleanup
    rm -f "$STAGE/QUIT-DONE"
    cat > "$STAGE/quit-visio.ps1" <<'QUIT'
$ErrorActionPreference = 'Stop'
try {
    $v = [Runtime.InteropServices.Marshal]::GetActiveObject('Visio.Application')
    $v.AlertResponse = 7
    $v.Quit()
} catch {
    "QUIT $($_.Exception.Message)" | Out-File (Join-Path $PSScriptRoot 'quit-log.txt') -Append
} finally {
    'DONE' | Out-File (Join-Path $PSScriptRoot 'QUIT-DONE')
}
QUIT
    local start=$SECONDS
    if launch '\\Mac\Home\Projects\diagramcraft\corpus\_vm\quit-visio.ps1' 20; then
        while [ ! -f "$STAGE/QUIT-DONE" ] && [ $((SECONDS-start)) -lt 20 ]; do sleep 1; done
    fi
    cleanup
    ps 'if (Get-Process -Name VISIO -ErrorAction SilentlyContinue) { Write-Output "VISIO still alive after Quit; forcing stop"; Stop-Process -Name VISIO -Force -ErrorAction SilentlyContinue } else { Write-Output "VISIO exited after Quit" }; exit 0' >&2 || true
}
health_error() { echo 'Visio health probe failed, see tools/oracle/README.md' >&2; }
health_once() {
    local result=0
    run_one visio-health 60 || result=$?
    [ "$result" -eq 0 ] || return "$result"
    [ -f "$STAGE/out/visio-health/HEALTHY" ] && grep -q 'HEALTH OK seconds=' "$STAGE/out/visio-health/probe-log.txt"
}
# Exactly one recovery probe after a watchdog. A recovery-probe timeout ends recovery,
# rather than recursing into repeated forced kills on an unhealthy guest.
recover_timeout() {
    local timed_out=$1 result=0
    gentle_stop
    local output_name=$timed_out
    if [ "$timed_out" = accept ]; then output_name=$ACCEPT_OUTPUT; fi
    local OUT="$STAGE/out/$output_name"
    convert || return 1
    health_once || result=$?
    if [ "$result" -ne 0 ]; then
        if [ "$result" -eq 124 ]; then
            gentle_stop
            OUT="$STAGE/out/visio-health"
            convert || true
        fi
        health_error
        return 1
    fi
}
if [ "$NAME" = accept ] || [ "$NAME" = dump-shipped ] || [ "$NAME" = export-masters ] || [ "$NAME" = visio-health ]; then
    result=0
    health_once || result=$?
    if [ "$result" -ne 0 ]; then
        if [ "$result" -eq 124 ]; then recover_timeout visio-health || true; fi
        health_error
        exit 1
    fi
    if [ "$NAME" = visio-health ]; then
        printf '%s\n' "$STAGE/out/visio-health"
        exit 0
    fi
fi
result=0
run_one "$NAME" "$LIMIT" || result=$?
if [ "$result" -eq 124 ]; then
    recover_timeout "$NAME" || exit 1
    exit 124
fi
[ "$result" -eq 0 ] || exit "$result"
printf '%s\n' "$OUT"
