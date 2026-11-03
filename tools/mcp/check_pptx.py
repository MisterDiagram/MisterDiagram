#!/usr/bin/env python3
"""Independent package checks for PPTX exports; all outputs stay in a fresh folder.

Requires python-pptx 1.0.2, pypdf and LibreOffice. Does not launch PowerPoint.
This checks package structure and page counts, not visual or glue parity.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import xml.etree.ElementTree as ET
import zipfile

from pptx import Presentation
from pypdf import PdfReader


def inspect(path, expected):
    presentation = Presentation(str(path))
    assert len(presentation.slides) == expected, "foreground slide count"
    assert len(presentation.slide_masters) == 1, "one master"
    assert len(presentation.slide_layouts) == 1, "one layout"
    assert not presentation.slide_layouts[0].placeholders, "blank layout"
    for slide in presentation.slides:
        master = slide.slide_layout.slide_master
        assert not master.placeholders, "blank master"
        assert any(rel.reltype.endswith("/theme") for rel in master.part.rels.values()), "master theme"
        assert not slide._element.xpath(".//p:style"), "explicit properties"
        assert not slide._element.xpath(".//a:schemeClr"), "resolved colours"
    return len(presentation.slides)


def foreground(path):
    with zipfile.ZipFile(path) as package:
        pages = ET.fromstring(package.read("visio/pages/pages.xml"))
    return sum(page.get("Background") not in ("1", "true") for page in pages.findall("{*}Page"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=Path, default=Path("target/release/misterdiagram"))
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--inspect", type=Path)
    parser.add_argument("--expected", type=int)
    parser.add_argument("inputs", nargs="*", type=Path)
    args = parser.parse_args()
    if args.inspect:
        assert args.expected is not None
        print(inspect(args.inspect, args.expected))
        return
    args.out.mkdir(parents=True, exist_ok=False)
    outputs = args.out.resolve()
    temporary = outputs / "tmp"
    temporary.mkdir()
    pdf_dir = outputs / "pdf"
    pdf_dir.mkdir()
    env = dict(os.environ, TMPDIR=str(temporary))
    paths = sorted(path for item in args.inputs for path in (item.glob("*/*.vsdx") if item.is_dir() else [item]))
    assert paths, "empty witness set"
    rows = []
    for number, source in enumerate(paths):
        output = outputs / f"witness-{number:04}.pptx"
        result = subprocess.run([str(args.cli.resolve()), "render", str(source), str(output)], capture_output=True, timeout=120, env=env)
        row = dict(source=str(source), output=str(output), export_exit=result.returncode, warnings=result.stderr.decode(errors="replace"))
        rows.append(row)
        if result.returncode == 0:
            try:
                row["expected"] = foreground(source)
                row["slides"] = inspect(output, row["expected"])
            except Exception as error:
                row["error"] = str(error)
        print(json.dumps({key: value for key, value in row.items() if key != "warnings"}), flush=True)
    eligible = [row for row in rows if row["export_exit"] == 0 and "error" not in row]
    profile = (outputs / "libreoffice-profile").as_uri()
    for start in range(0, len(eligible), 12):
        batch = eligible[start:start + 12]
        command = ["/opt/homebrew/bin/soffice", f"-env:UserInstallation={profile}", "--headless", "--convert-to", "pdf", "--outdir", str(pdf_dir)]
        command.extend(row["output"] for row in batch)
        result = subprocess.run(command, capture_output=True, timeout=180, env=env)
        for row in batch:
            row["soffice_exit"] = result.returncode
            pdf = pdf_dir / (Path(row["output"]).stem + ".pdf")
            try:
                assert result.returncode == 0, result.stderr.decode(errors="replace")
                assert pdf.is_file() and pdf.stat().st_size > 0, "new nonempty PDF required"
                row["pdf_pages"] = len(PdfReader(pdf).pages)
                assert row["pdf_pages"] == row["expected"], "PDF page count"
            except Exception as error:
                row["error"] = str(error)
    passed = sum(row.get("slides") == row.get("pdf_pages") == row.get("expected") and "expected" in row and "error" not in row for row in rows)
    report = dict(total=len(rows), passed=passed, failed=len(rows) - passed, rows=rows)
    (outputs / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "rows"}), flush=True)
    raise SystemExit(0 if passed == len(rows) else 1)


if __name__ == "__main__":
    main()
