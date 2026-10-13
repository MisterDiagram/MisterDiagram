# Target application parity

Status: pre-alpha, M0 and M1 met; M2 human gate accepted; independent closure verification complete. Target is Microsoft Visio Standard 2021.

Weighted breadth is 23% across 45 Core=Y rows: 1 done, 16 partial, 28 missing.
The renderer emits PNG, SVG and vector PDF. All ten generated render groups
pass the ceilings frozen by Charles on 2026-10-12 at 16:45. Four one-pixel size
differences are tolerated and scored over the common area; differences of two
pixels or more remain hard failures. Render comparison is mandatory in CI again.
Text-metric cells remain promoted, with oracle1 floor 97.4911%. All 1,108 contact
master drawing areas contain ink, with zero blank or failed cells across 24
stencils. The earlier successful-call claim concealed 643 blank cells; the
corrected baseline was 465 ink-bearing cells. Ink coverage is not pixel parity. Remaining text and metafile/compound-line
gaps move to Phase 3.
See file-format-parity.md for measurements and gaps.md for counted follow-ups.

Core=N shell, view, review and platform rows remain separately reported. Authoring,
round-trip writing, routing and automation are not accepted. The human gate
and two consecutive independent closure verification passes
are complete. Phase 3 is in progress; Visio acceptance of writer output is pending.
