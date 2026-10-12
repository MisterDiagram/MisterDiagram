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
- `common.ps1`: shared helpers loaded by the named scripts, not run independently.

CURRENT records the active file for a 30-minute per-file watchdog. The default overall timeout
is 1800 seconds. On timeout the runner kills VISIO and preserves partial outputs. DONE denotes
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
