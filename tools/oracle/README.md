# Visio oracle scripts

Run from the Mac with `tools/oracle/run-in-vm.sh tools/oracle/NAME.ps1 [seconds]`.
The runner starts the VM, waits up to 180 seconds for guest readiness with one retry, then
registers an interactive task as charles. It removes the old visioprobe task, watches DONE,
converts UTF-16 text outputs to UTF-8, and unregisters its task. A missing initial log receives
one retry. COM always runs within the scheduled script. Scripts set AlertResponse=7 and hide
Visio, close documents without further saves, and call Quit in finally.

- `version.ps1`: runner smoke test, version and build only.
- `dump-blank.ps1`: blank and rectangle documents, document/page/style/rectangle sheets.
- `make-corpus.ps1`: feature drawings, PNG at 96 dpi, SVG, PDF, sheets and connects. Use timeout 7200.
- `dump-shipped.ps1`: local-only copies and recursive master dumps. Run with timeout 14400.
  Completed TSVs are skipped, logs are appended, and all source files are copied before dumping.
  Incomplete dumps use .partial and are never counted as completed.
- `accept.ps1`: reads corpus/_vm/accept-list.txt, opens files without saving, dumps and exports.
- `visio-health.ps1`: launch/attach, local shipped-template OpenEx, blank Add, both closes and Quit.
- `export-masters.ps1`: reads the staged `corpus/_vm/export-masters-list.txt`, exports each master at 96 dpi under `corpus/_vm/out/export-masters`, and records IDs in `masters.tsv`. Health runs first and DONE is written last; SUMMARY is the success record.
- `common.ps1`: shared helpers loaded by the named scripts, not run independently.

CURRENT records the active file for a 30-minute per-file watchdog. The default overall timeout
is 1800 seconds. On timeout the runner requests Quit in a separate interactive task, bounded
to 20 seconds, and uses Stop-Process only if VISIO is still alive. It preserves partial outputs
and runs the health probe before returning. DONE denotes
script completion, not success: always inspect probe-log.txt for failures and SUMMARY counts.

ShapeSheet TSV columns are shape_id, shape_name, section, row, cell, name, formula, result_iu,
units. FormulaU is retained. ResultIU uses round-trip invariant formatting. Units is the COM
unit code. Strings escape backslash, tab, CR and LF. shape_name includes page/master identity
and the group path because IDs repeat across pages and masters. Shape traversal uses sections
1 through 250, SectionExists, RowCount and RowsCellCount. The Style COM object instead exposes
Section and Row objects; their Count properties and absent-row exceptions provide traversal.
Generated and acceptance dumps include document, page, style, master and page-shape sheets.
The mandated traversal is retained. Since fixed-section RowCount counts local rows rather than
logical indices, an additive pass reads missing documented Object rows through CellsSRCExists,
deduplicated by SRC within each scoped sheet. Logs separate primary and supplemental cell counts.
Cell-access failures are counted and logged. In dump-shipped, any cell error fails that file
and retains its .partial; only a fully successful dump is renamed to .tsv. Other dump scripts
retain failed cell rows with empty values for diagnosis. PNG pixel sizes are logged by the script;
the actual density check is host-side in log/check-generated.py.

Image inputs: generate an original small PNG/JPEG on the Mac in corpus/_vm/assets/sample.*,
then copy the local probe oracle-basic.emf there as sample.emf. None of these assets is tracked.
The generated corpus stays private. Shipped Microsoft content never enters either Git repo.

Microsoft references:

- [Visio file format](https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx)
- [ShapeSheet reference](https://learn.microsoft.com/en-us/office/client-developer/visio/visio-shapesheet-reference)
- [Style.Section](https://learn.microsoft.com/en-us/office/vba/api/visio.style.section)
- [Raster resolution](https://learn.microsoft.com/en-us/office/vba/api/visio.applicationsettings.setrasterexportresolution)
- [Selection.Layout](https://learn.microsoft.com/en-us/office/vba/api/visio.selection.layout)
- [Characters.AddField](https://learn.microsoft.com/en-us/office/vba/api/visio.characters.addfield)

After a generation timeout, preserve the entire partial output folder under corpus/_vm/history/.
`python3 tools/oracle/checkpoint_generated.py PRESERVED_FOLDER` copies only drawings logged as
complete, with all six exports present, into corpus/_vm/resume-make-corpus. It records SHA256
checksums and preserves the producing recipe. Use only the just-finished run after reviewing any
recipe edits; continuation must change checkpoint handling only, not drawing construction. A later make-corpus run verifies the shared dump implementation and every copied
file, then reuses those exact bytes and logs DRAWING RESUMED. Its made count includes resumed
drawings; cell counters describe freshly dumped cells only. Use a longer explicit runner timeout
for that continuation. No failed or incomplete drawing is reused. Preserve the earlier log.
Remove the checkpoint directory after completion or before requesting a fresh full generation.

Host checks (no VM activity):

```sh
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tools/oracle -p 'test_*.py'
pwsh -NoProfile -File tools/tests/checkpoint.ps1
pwsh -NoProfile -File tools/tests/shipped-resume.ps1
bash -n tools/oracle/run-in-vm.sh
```

To resume shipped dumping after Step 5, copy corpus/ms-shipped-dumps back to
corpus/_vm/out/dump-shipped (preserving its TSVs and log), run the script with 14400,
then move the updated output back to corpus/ms-shipped-dumps. The runner preserves this
output directory and removes only stale DONE/CURRENT markers. Do not reuse .partial files.

Acceptance inputs are generated by the runner from Mac paths under corpus/, never hand-edited:

```sh
tools/oracle/run-in-vm.sh accept.ps1 300 corpus/generated/basic/0001-probe.vsdx
tools/oracle/run-in-vm.sh accept.ps1 900 corpus/generated/basic/0001-probe.vsdx corpus/blank/blank.vsdx
```

The runner writes UTF-8 Windows paths to corpus/_vm/accept-list.txt. Each input is copied
to the interactive user's TEMP/diagramcraft-accept directory, opened locally with OpenEx(..., 2),
and removed in finally. Package parts are read from the same local copy. Outputs stay on the share.

Use `tools/oracle/run-in-vm.sh --check-task` for the required leftover-task count and
`tools/oracle/run-in-vm.sh --stop` to stop the VM. The count check passes when it prints 0,
even if PowerShell returns 1 for its silenced missing-task lookup.

## Known VM behaviour

On 2026-10-12, since 02:28, an automation-launched Visio (`VISIO.EXE /Automation -Embedding`,
created by `New-Object -ComObject`) hangs in Documents.Open and OpenEx, including on a shipped
template it previously opened. Documents.Add still works. Local copies and share paths both
hang. A normally launched VISIO.EXE opens those files promptly, and COM calls through
GetActiveObject work against that instance. The cause of the mode difference is unknown;
this is not evidence of a file, share or cache problem. Private evidence is in
corpus/_vm/out/diag-* and corpus/_vm/out/accept-visible.

Every oracle now uses common.ps1 New-Visio: stop leftover VISIO, launch Office16/VISIO.EXE
normally, poll GetActiveObject once a second for up to 90 seconds, then set AlertResponse=7
and retain the script's visibility setting. Do not reintroduce New-Object -ComObject.
The local-copy path in accept.ps1 remains in use.

The runner automatically runs `visio-health.ps1` before accept and dump-shipped, and after a
watchdog cleanup. Run it directly with `tools/oracle/run-in-vm.sh visio-health.ps1 60`.
It uses the same normal-process launch and attach as every oracle, opens the local shipped
ACTDIR_U.VSTX read-only, adds a blank drawing, closes both and quits. Every step logs its
duration. Healthy means all steps, including Quit, complete in less than 60 seconds; DONE
alone does not pass. A failed probe aborts with
`Visio health probe failed, see tools/oracle/README.md`. Prior probe outputs are preserved
under corpus/_vm/history/visio-health.* before another probe runs.

Timeout cleanup first stops the hung scheduled script, then schedules an interactive
GetActiveObject/Quit call with its own 20-second limit. Only a still-running VISIO is forcibly
stopped. One recovery health probe follows. If that probe also times out, cleanup ends the
attempt and reports failure, without an unbounded cycle of probes and forced kills.
Repeated forced kills are suspected of contributing to the damaged guest state; this has
not been established as the cause.

If the health probe fails twice, a human administrator can run the silent Office quick repair
(about five minutes), then re-run the probe. Agents must not run repair or elevate privileges:

```text
"C:\Program Files\Common Files\Microsoft Shared\ClickToRun\OfficeClickToRun.exe" scenario=Repair platform=x64 culture=en-us RepairType=QuickRepair forceappshutdown=True DisplayLevel=False
```

Quick repair did not fix plain COM launch on this guest. Keep the normal launch-and-attach
fallback after repair; a successful Add alone does not establish that OpenEx is healthy.

`run-in-vm.sh dump-shipped.ps1 600 corpus/ms-shipped/1033/BASFLO_U.vssx`
selects a shipped file for a missing COM observation. With no file arguments the
full background sweep is unchanged. Both paths run the health probe first, copy
all shipped files before traversal, preserve completed TSVs and write DONE last.
Selected output stays in `corpus/_vm/out/dump-shipped/`; do not publish it into the
shared read-only `corpus/ms-shipped-dumps` input directory.

### P3 acceptance

Run `cargo test -q -p diagramcraft-model --test from_scratch` to build the five
private acceptance drawings under `corpus/_state/m3/scratch/`. The test also runs
without the private corpus, except its explicitly skipped BASIC_U case. The
selection manifest `corpus/_state/m3/accept-plan.json` lists 204 generated drawings,
one blank, five scratch documents and twenty shipped files chosen by size spread.

`cargo run -q --release -p diagramcraft-corpus --example prepare_accept` writes
recalculated packages to `corpus/_state/m3/accept-inputs/`, computed and source-backed dumps and page/
master PNGs to `corpus/_state/m3/accept-expected/`. Run
`python3 tools/oracle/accept_report.py --prepare` to hash the complete selection
and snapshot literal serialized cells in `corpus/_state/m3/accept-manifest.json`.

From the repository root:

```sh
tools/oracle/run-in-vm.sh accept.ps1 7200 corpus/_state/m3/accept-inputs/*.vsdx corpus/_state/m3/accept-inputs/*.vssx corpus/_state/m3/accept-inputs/*.vstx
python3 tools/oracle/accept_report.py corpus/_vm/out/accept
```

The runner health-checks Visio before the interactive scheduled task. Each file
gets its own directory containing the actual post-open `saved.*`, a COM dump,
part inventory and every page/master PNG. A hashed `status.json` is the per-file
completion record. A rerun reuses only successful, unchanged input/script/output
hashes and archives stale or failed observations. DONE means the script finished,
not that the acceptance gate passed. Only the host report determines acceptance.

The report rejects any changed part list or changed serialized sheet cell outside
the existing hook/volatile exclusions. It also checks COM against concrete local serialized caches, including missing
rows and units. Inherited and Themed cells have no local numeric cache: their
evaluator-derived comparisons are diagnostics, not claims about written values.
Stored formula and cache comparisons are separate from re-evaluation. Connects
records are compared too, so lost glue cannot pass merely because pixels match. PNG groups use the
frozen MAD/fraction limits, unweighted file MAD and pixel-weighted differing
fraction; one pixel per axis is tolerated over the common area. No alignment,
resampling or expectation updates occur. Full diffs and worst files are written
to `corpus/_state/m3/accept-report.json`. Missing or stale outputs fail closed.

Host parser regressions: `PYTHONDONTWRITEBYTECODE=1 python3 tools/oracle/test_accept_report.py`.
