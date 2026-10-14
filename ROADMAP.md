# Roadmap

Status: pre-alpha, M0 and M1 met; M2 human gate accepted, independent closure verification complete. Target: Visio Standard 2021.

Phase 0 establishes the workspace, conventions, oracle scripts and private corpus. Phases 1
through 5 add reading and ShapeSheet evaluation, rendering, writing, authoring, and exports.
M0 closed at 1473b0d. M1 code closed at e7dfbd4 after all acceptance commands and two
clean independent frozen verification passes.

## Progress

- 2026-10-11: M0.1 scaffold, bounded OPC and CLI info. CI: 44 tests, 14 wasm libraries, all six gates passed.
- 2026-10-11: M0.2 conventions and Core-aware checklist. All feature rows remain missing.

- 2026-10-11: M0.3 runner smoke test.
- 2026-10-12: M0.4 oracle scripts. 204 generated drawings, 827857 sheet rows; blank dump 3385 cells.
- 2026-10-12: M0.5 corpus pin and 1239-file checksum manifest. All 1181 shipped files local; 104/1181 dumps complete, remainder deferred to later nights.
- 2026-10-12: M0.9 normal-process launch and COM attachment pass two-file Visio acceptance; M0 met.
- 2026-10-12: M1.1 to M1.4 reader, grammar,70 functions, inheritance and dirty recalc. All1387 packages parse;5548 deterministic mutations produce no panics.
- 2026-10-12: M1.5 CLI sheet/corpus tooling. Probe5344 eligible cells match; first full measurement shipped cached91.411080%, COM97.553995%. M1 remains incomplete while the97%cached floor is unmet. Shipped dumps104/1181.

- 2026-10-12: M1.6 numeric requirements pass: shipped cached 97.978884% over 5355260 cells; resolved 98.763183% over 7271082 cells. CI quick corpus gate and 104 functions implemented. All acceptance commands and both frozen verification passes are clean; M1 met. Shipped dumps 104/1181.

- 2026-10-12: M2.0 discovery fixes: strict cached 4302905/4408935 (97.595111%), resolved 7209425/7271082 (99.152024%), non-literal resolved 99.836044%. Basic-rectangle dump zero differences; CI seven gates pass; parity remains 10%.

- 2026-10-12: M2.1 all fifteen geometry row kinds to kurbo paths, bounded NURBS/splines, section flags and y-up group transforms. Private corpus witness and numerical tests added; renderer work remains next.

- 2026-10-12: M2.2 procedural paints, strokes and arrow primitives. Corpus-derived fill densities and dash lengths tested; rendered fidelity is not yet measured.

- 2026-10-12: M2.3 all eight theme functions evaluated and included in the cell oracles. Full cached98.128577%, resolved99.192198%, theme-only resolved99.875218%. Parity11%; image parity remains unmeasured.

- 2026-10-12: M2.4 partial worktree stopped for the missing Latin font pack. Full cached95.161026% fails the frozen floor; resolved99.100234%, text-only20.248276%. CI fails its mandatory corpus gate. Text rows remainP, parity14%; no M2.4 commit or M2 closure.

- 2026-10-12: M2.4.0 embeds the requested 18 OFL Latin faces and adds Charles's 90% text promotion gate. CI and public no-craft-fonts tests pass (230 tests); quick main oracles98.879415%/99.037858%, text39.341608%/41.824862%, still deferred. Text completion and rendering remain open.

- 2026-10-12: M2.4 and M2.5 committed with passing CI: text layout/fields/tabs and PNG/SVG/vector PDF backends. Shared UTF-16 dump coverage is read without mutation. M2.6/M2.7 remain uncommitted at the partial gate: 204 renders, 35 size mismatches, nine failing pixel groups; SVG/PDF checks pass. Twenty-four paired sheets cover 1108 masters, with 591 failed local renders. Parity 23%; no M2 acceptance.

- 2026-10-12: P2-followup-2 worktree: arrow/cap dimensions fixed (35 size failures reduced to10), two edge profiles match exactly, BASIC theme extension indexing corrected, contacts1107/1108, foreground/background selection corrected. Text height slack, percentage baselines, drawing-unit conversion and empty-height handling have corpus regressions. Gates remain partial; no new Rust commit while CI is red.

- 2026-10-12: P2-followup-2 partial gate: main cached98.118866%, COM99.226552%; text80.083190%/80.165717%, still deferred. Ten wrong sizes, eight failing render groups; fill/line pass. Contacts1107/1108, one BASFLO direct cycle. CI fails render, so no commits or M2 closure.

- 2026-10-12: P2-followup-3 commits M2.6/M2.7 under Charles's advisory-render CI rule, then SVG text advances, marker profiles, transformed strokes, transparent image sampling, the final master cycle, FIND and fixed-point text metrics. CI 8 gates pass; strict corpus remains partial: 8/10 render groups pass, 10 size mismatches; text 84.481614%/83.415329% stays deferred. Contacts 1108/1108, 24 paired sheets; 204 render commands and SVG/PDF checks pass. No ceilings, floors or oracle files changed.

- 2026-10-12: P2-followup-4 fixes composed width, field formatting, inherited display units and six text export dimensions. Text passes promotion at 91.021727% cached and 90.713982% COM; oracle1 is 97.491192% on its promoted denominator, rebased floor 97.4911% by Charles. CI passes all eight gates. Strict corpus remains partial: nine groups pass, four one-pixel text dimensions fail. All 204 renders and SVG/PDF checks pass; 24 paired sheets cover 1108 masters. Human gate pending, no M2 closure.

- 2026-10-12 16:45: Charles accepts the M2 human gate with one-pixel export tolerance over the common area. Original pixel ceilings are frozen, render CI is mandatory again, and promoted cell floors stand. All ten render groups pass the rule test. Remaining text, 17 EMF+ comments and four non-default compound-line witnesses are Phase 3 gaps. Independent M2 review runs before Phase 3.

- 2026-10-12 M2.9.1: independent review invalidated prior1108/1108 rendering claims:465cells had ink and643were blank. EMF header detection, compound fills and group ordering restore1108ink/0blank/0errors across24stencils. Generated oracle numbers are unchanged; closure verification remains pending.

- 2026-10-12 M2.9 closure: corrected shipped coverage is 1,108 drawing cells with ink, zero blank and zero failed, from 465 ink and 643 blank. Permanent text promotion, pre-decode image/XML bounds and per-file pixel disclosure pass regressions. Independent pass 1 found an aggregate sheet-cell gap, fixed in M2.9.5; passes 2 and 3 are clean. Final full P2 acceptance passes all six commands: CI eight mandatory gates, ten render groups, both main/text floors, 204 render commands and SVG/PDF checks, 49 contact PNGs including 24 regenerated pairs. Parity remains 23%; ten individual pixel outliers and Phase 3 gaps remain disclosed. M2 closed; stop before the next brief.

- 2026-10-12: M3.4 adds complete source-cell round-trip comparison and original-viewport image pairs. CI passes all eight gates, including all1,387packages at the99%cell floor and the256-package quick corpus gate. Passive Action/Event reads, last-valid error references and deleted-shape painting have regressions. Full final-tree render measurement and the230-file Visio acceptance remain pending; stopping before M3.5 at Charles's budget handoff.

## M3.10 COM coverage ruling, 2026-10-13

The historical population at 9eb358d has 301 completed shipped dumps plus one partial,
not 302 completed. Membership was reconstructed from 104 FILE OK entries in probe-log.txt
and 197 additional entries before the 2026-10-13T00:32:39Z host-pull marker in winvm-log.txt.
The partial AZUREAPPSERVICES_M dump is not eligible. Generated/blank 213 dumps remain old.
The independently reviewed normalized membership SHA256 is
05375146b6ef812251baa5f22a41266dd2d261a43469fc927bf85b71cdf9bc81.

| Population | Completed dumps | Passed / eligible | COM rate |
|---|---:|---:|---:|
| Old, including generated/blank | 514 | 20805945 / 21055732 | 98.813686458% |
| New shipped | 401 | 23240476 / 23671431 | 98.179429879% |
| Combined | 915 | 44046421 / 44727163 | 98.478012120% |

The old rate meets the 98.6241% contract. This is coverage growth, not regression.
Charles's P3-closure M3.10 explicitly authorizes rebasing the expanded denominator.
The COM floor is now 98.478012100% (achieved fraction rounded down at nine decimals),
and may only rise from this population. Text ratchets, cached floor, exclusions and
render ceilings are unchanged. The shared directory remains read-only.
Measurement: cargo run -q --release -p diagramcraft-corpus --example com_coverage --
corpus/_state/m3/old-coverage-paths.txt; log/m3-10-coverage.log.

## M3.11 acceptance rerun (2026-10-13): partial

All 230files opened and produced artifacts; the scratch connector has2 live COM connects.
All 230files fail the unchanged repair comparison (4,107 literal/part differences).
All 10generated page render groups pass; other acceptance groups still fail.
Host read-write-read cells remain 100% and image pairs are identical or the 642 authorized
same-error pairs. Main COM meets its measured coverage rebase; promoted COM text is
90.413403%, below its unchanged 90.713900% ratchet. No M3 acceptance or Phase 4 start.
Representative witnesses and per-group results are in docs/file-format-parity.md.

### M3.21: pinned COM text cohort ruling

Charles's P3-closure-2 M3.21 authorizes the same coverage rule for sticky text COM.
Old text: 12807/14118 = 90.713982150%, meeting the 90.7139% contract.
New text: 14553/16143 = 90.150529641%. Combined: 27360/30261 = 90.413403391%.
The combined sticky text floor is 90.413403300%, rounded down at nine decimal places.
The cached text floor is unchanged. Pinned historical 702-dump main COM remains 44,046,421/44,727,163 =
98.478012120%, with floor98.478012100%. Its old cohort was20,805,945/21,055,732 =
98.813686458%, above98.6241%; new was23,240,476/23,671,431 =98.179429879%.

The preceding brief's413-dump observation was27,416,668/27,803,388 =98.609090%,
framed as302old+111new. Proven completed membership was301old+112new, because
one old artifact was partial. The702-dump measured main floor stands. The later872-dump audit measured
53,865,489/54,718,413 =98.441248652%, below that standing floor; it is not rebased. Both main
and text cohorts are now committed under apps/diagramcraft-corpus/tests/cohorts/,
with exact dump paths, counts and SHA256 pins in floors.toml. Generated/blank213
dumps stay in the old population; old total514, combined915. These fixed lists
cannot silently move to a later snapshot. The shared source directory is read-only.

## M3.23 fresh acceptance, 2026-10-14 UTC: partial

Required results are not met. All 230 files opened, but 226 are repaired under the
M3.20 definition and 229 fail at least one mandatory check. There are 5,856 repair
differences and 179,958 source COM differences. COM is a separate column and does
not by itself mark a file repaired. No new repair exception was added.
| Set | Files | Opened | Repaired | Failed | Thumbnail added | MasterType | UniqueID |
|---|---:|---:|---:|---:|---:|---:|---:|
| generated | 204 | 204 | 203 | 203 | 0 | 7 | 0 |
| blank | 1 | 1 | 1 | 1 | 0 | 0 | 0 |
| from-scratch | 5 | 5 | 5 | 5 | 5 | 7 | 0 |
| shipped-stencil | 10 | 10 | 7 | 10 | 1 | 0 | 0 |
| shipped-template | 10 | 10 | 10 | 10 | 0 | 0 | 0 |

Strict round-trip cells: 15,079,024/15,091,678 = 99.916152%. Successful image pairs
are identical; 642 identical error pairs remain separately disclosed. Host CI,
parity, round-trip, malformed and from-scratch tests pass; full corpus and acceptance
reporter fail. All 230 files opened, including the three authored masters and the
resized dropped master. Scratch connector has two COM connections and visible ink.
The required pixel, repair and metric COM conditions remain unmet. Parity stays
25% (1 D, 17 P, 27 M of 45 Core rows). M3 is partial; Phase 4 has not started.

| COM cohort | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Old, 301 shipped plus 213 generated/blank | 20805945 / 21055732 | 98.813686458% | 12807 / 14118 | 90.713982150% |
| New within pinned 702, 401 dumps | 23240476 / 23671431 | 98.179429879% | 14553 / 16143 | 90.150529641% |
| Pinned 702 shipped, 915 total | 44046421 / 44727163 | 98.478012120% | 27360 / 30261 | 90.413403391% |
| Later 170 shipped, outside rebase | 9819068 / 9991250 | 98.276672088% | 11477 / 11764 | 97.560353621% |
| Full measured 872 shipped, 1085 total | 53865489 / 54718413 | 98.441248652% | 38837 / 42025 | 92.414039% |

Old text meets 90.7139%, authorizing M3.21's pinned combined text floor
90.413403300%. Cached text is unchanged: 61,748/67,797 = 91.077776%, above
90.9875%. Main COM retains 98.478012100%; later coverage fails that floor and is
not rebased. Cached main is 4,675,663/4,783,865 = 97.738189%, above 97.4911%.
The preceding 413-dump measurement was 27,416,668/27,803,388 = 98.609090%, framed
in the brief as 302 old plus 111 new; proven complete membership is 301 plus 112.
Both pinned main/text memberships and SHA256 values are committed in
apps/diagramcraft-corpus/tests/cohorts/ and named in floors.toml. Ordinary full
checks keep consuming all completed coverage. Workers' shared input remains read-only.

Final frozen verification passes1and2 each returned10YES/4NO. The NO lines are
M3.13 (dropped-master pixels), M3.15 (metric COM units), M3.16 (connector pixels)
and M3.23-Gates. Both seeds were caught101/restored0. No two-clean-pass claim.
M3.23 records partial results only. Full private report:
corpus/_state/P3-closure-2-report.md; detailed register:
plan/m3-verification-register.md. Final Python reporter suite25/25 passes.

## M3.27 pinned COM populations (2026-10-14)

Charles's P3-closure-3 authorizes the 702-to-872 coverage rebase. The old population
still meets both contracts; no evaluator regression is hidden.

| Population | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Old 702 shipped plus generated/blank | 44046421 / 44727163 | 98.478012120% | 27360 / 30261 | 90.413403391% |
| Additional 170 shipped | 9819068 / 9991250 | 98.276672088% | 11477 / 11764 | 97.560353621% |
| Combined 872 shipped plus generated/blank | 53865489 / 54718413 | 98.441248652% | 38837 / 42025 | 92.414039262% |

Both main and text COM gates now measure exactly the 1085 dumps named in
`apps/diagramcraft-corpus/tests/cohorts/m3-872-dumps.txt`, including 213 generated/blank
dumps. SHA256: c6e799b605821b38e35a3c3ad372af75344738364c4d23904736e6da4a390ff7.
The 872 package list has SHA256
3fe68431102e8e8f46b154bafb7bac0d5c9e38cb872c325c114aa45c72a1a2b3.
Missing members or changed membership hashes fail. CI uses the full COM population;
its cached and round-trip sample remains 50 shipped files. Later dump additions
are printed separately and cannot move the gate. At the audit, another 246 dumps
were informational: main 9751471/9999792 (97.516738348%), text 8574/9193
(93.266615903%). Cached floors and render ceilings are unchanged.

## M3.30 closure-3 (2026-10-14): partial

230/230 opened, 225/225 originals baselined. Generated/blank repairs are zero;
eight files retain 4,705 repair differences. Scratch COM is zero, shipped residual
COM is 309 (SOCIAL_DISTANCING_M). All ten generated page groups pass; generated
masters and scratch page/master groups fail the unchanged ceilings. Strict host
round-trip 99.916152%, CI eight gates green, 5,548 mutations without panic, parity
25%. The pinned 872 population passes at 98.441248652% main and 92.414039262%
text, against old 702 rates 98.478012120% and 90.413403391% (full cohort table
above). M3 remains unaccepted pending repair/pixel fixes and clean frozen review.

## P3-closure-4: stopped at M3.33 (2026-10-14)

M3.31 preserves the exact local structure and literal attributes of unchanged read
shapes. All 20 shipped acceptance originals pass the regression. Strict round-trip
is now 15,080,738/15,080,738 cells (100%) across 1,387 packages, from 99.916152%
and 12,654 mismatches. The comparison and 99% floor are unchanged; the writer no
longer adds 10,940 local inherited cells to the union being compared.

M3.32 completes the observed authored package metadata and narrows the three
permitted scratch first-save classes. Host validation passes; fresh Visio residual
repair and COM validation is pending. The acceptance inputs and VM artifacts are
still the closure-3 run. Reclassifying those old inputs leaves 673 scratch repair
records (from 693), including seven legacy MasterType changes now counted as
repairs under the current three-class contract. This is not a new-writer result.

M3.33 measured 66 straight-edge views and six global vertical mappings. None
passes all required groups. Best diagnostic fractions are 2.055466% for scratch
masters and 2.490908% for scratch pages, from different candidates, against 2%.
The horizontal grid trial passes generated masters at 0.101071% but fails image
at 2.022449%. No transform was adopted. The specified M3.33 stop rule applies;
M3.34 comparison refinement, the changed-input VM run and full verification are
pending. M3 remains unaccepted and Phase 4 has not started.

All 1,181 shipped dumps are now complete (1,394 total). The pinned 872-shipped
population still gives main COM 98.441248652% and text 92.414039262%. CI's full
COM directory informational rates are 98.426041% and 93.143692%; cached quick
coverage is only 50 shipped files and is not a full cached-corpus measurement.
No floor, ceiling or pinned population changed. Parity remains 25%.

## M3.36 final COM coverage (2026-10-14)

Charles authorizes this final coverage rebase in P3-closure-5. The pinned 872
population retains its measured main and text rates, so this is coverage growth,
not an evaluator regression. The 309 additions were evaluated as a separate
cohort through the same oracle, with both partitions reconciled to the directory.

| Population | Main passed / eligible | Main rate | Text passed / eligible | Text rate |
|---|---:|---:|---:|---:|
| Pinned 872 shipped plus 213 generated/blank dumps | 53,865,489 / 54,718,413 | 98.441248652% | 38,837 / 42,025 | 92.414039262% |
| Additional 309 shipped dumps | 21,088,976 / 21,434,670 | 98.387220330% | 15,354 / 16,155 | 95.041782730% |
| Final 1,181 shipped plus 213 generated/blank dumps | 74,954,465 / 76,153,083 | 98.426041399% | 54,191 / 58,180 | 93.143691990% |

The final gate pins 1,394 dump paths (1,181 shipped plus 213 generated/blank) in
`apps/diagramcraft-corpus/tests/cohorts/m3-1181-dumps.txt`, SHA256
`d571c4e903ea24a08ed1b2190d6984f8aed0add91977b08e9429b360b0a75b44`.
The package membership SHA256 is
`0370d8758e00ca1573c70d909e504bbade52acce674418f466a9b5d375f77a4a`.
Main floor: 0.984260413; sticky text COM floor: 0.931436919, each rounded
down to nine decimal places as before. Cached floors and render ceilings are
unchanged. The old 872 and earlier cohort lists remain committed as evidence.
The worker is retired: 1,181 of 1,181 shipped dumps are complete, with no partials.

## M3.38 final closure-5 measurement (2026-10-14): partial

The changed-input run selected exactly seven of 230 packages: five scratch files,
PTSKST_M and SOCIAL_DISTANCING_M. The first 3600s run completed six and timed
out (124) on SOCIAL. Its unchanged-budget single-file retry completed in about
58 minutes (exit 0). Checks were at least 900 seconds apart; baseline was not
rerun. All 230 files now have complete opened observations.

The acceptance reporter exits 1: five files retain 15 genuine repair records.
Four scratch Page 0 entries change ViewCenterX, ViewCenterY and ViewScale (12
records); AZURECOMPUTE_M has three added root-thumbnail relationship records
after our writer removed a relationship present in the original and its baseline.
No broader exception is applied. All 230 files have zero residual COM differences.
SOCIAL has D_ours 1011, D_base 1012, subtracted 1011 and residual repairs 0;
its 15712 raw COM differences all occur in the baseline, leaving residual 0.
The previous 309 added-cell differences are gone. PTSKST_M also has zero repairs
and residual COM.

All required page render groups pass the frozen ceilings and one-pixel rule.
Generated masters: MAD 1.202102, over16 2.648070%; scratch masters: 2.823616,
4.452799%; scratch pages: 4.609743, 3.019190%. These three groups remain
measured, ruling pending. There is no renderer change.

CI (8 gates), parity, corpus, current-target roundtrip, malformed and from_scratch
all exit 0. Strict cells: 15080738/15080738 (100%), every mismatch class zero.
Writer fuzz: 1387 files, 5548 mutations, zero panics. The stale standalone
roundtrip log from a reused seed Cargo target is superseded by the main-target
-current logs; CI and corpus independently agree with the corrected run.
Five source seeds were caught and restored exactly. M3 remains unaccepted
because the reporter's zero-repair condition is unmet, not because of the three
small-image groups. Phase 4 has not started.

Two independent frozen passes completed in order: closure5_review then
m3_writer_review. Each has 8 YES and 2 NO (M3.35 and M3.38-Gates), both due to
the 15 residual metadata repairs. All five production seeds are caught and
restored byte for byte. These are two completed passes, not two clean passes.
No additional source changes, comparison exceptions or render changes were
introduced to manufacture closure.

## M3 gate ruling: small-image export mapping (Charles, 2026-10-14)

M3 render acceptance rests on the ten page-size groups, with unchanged frozen
ceilings and the one-pixel/common-area rule. Generated-masters,
from-scratch-masters and from-scratch-pages are informational until Visio's
export mapping for small images is derived, then reopen as required gates.
Corpus check prints their saved acceptance measurements explicitly as such;
the acceptance reporter still measures and prints them. No ceiling changes.

M3.39 authors Page ViewScale=-1 and ViewCenterX/Y at the page centre in internal
inches. The Page-level class "Page view attributes rewritten on save" is counted
separately beside the scratch document-level blank baseline. Only the three
exact Page attributes with finite saved numbers qualify; removals, other paths
and unrelated attributes remain repairs. Witnesses: corpus/blank/blank.vsdx,
the generated D_base Page records and Visio's saved scratch drawings.

## M3.41 acceptance rerun, 2026-10-14

M3.39 and M3.40 close the fifteen prior residual metadata records. Six SHA-changed
inputs alone were rerun: five scratch files and AZURECOMPUTE_M. All 230 inputs
now have complete opened observations, zero residual repairs and zero residual
COM differences; the acceptance reporter exits 0. The 224 unchanged input hashes
and 1664 baseline artifact hashes are unchanged. Dynamic glue has two Connects.
Ten page-size groups pass the frozen ceilings with the one-pixel rule. The three
small-image groups remain informational under the dated ruling above. Both
implementation CI runs pass all eight gates; strict roundtrip is 15080738 of
15080738, and 5548 mutations across 1387 packages produce zero panics. Final
full corpus completion and two clean frozen passes are recorded in M3.42.
