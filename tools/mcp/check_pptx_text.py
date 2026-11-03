#!/usr/bin/env python3
"""Compare evaluator TSV with python-pptx's native paragraphs and run elements.

Breaks are included as logical runs (a:br); python-pptx's paragraph.runs excludes
those, so its public runs are also checked independently against the a:r entries.
Fields export as evaluated a:r text. No PowerPoint or native oracle is used.
"""
import argparse
import csv
import json
from pathlib import Path
import subprocess

from pptx import Presentation
from pptx.oxml.ns import qn


def text_hex(text):
    return text.encode("utf-8").hex()


def blocks(shapes):
    for shape in shapes:
        if shape._element.tag == qn("p:grpSp"):
            yield from blocks(shape.shapes)
        elif shape._element.find(qn("p:txBody")) is not None:
            yield shape.text_frame


def readback(path):
    presentation = Presentation(str(path))
    rows = [["S", str(len(presentation.slides))]]
    for page, slide in enumerate(presentation.slides, 1):
        for block, frame in enumerate(blocks(slide.shapes), 1):
            key = [str(page), str(block)]
            rows.append(["B", *key, str(len(frame.paragraphs))])
            for pi, paragraph in enumerate(frame.paragraphs):
                elements = [e for e in paragraph._p if e.tag in (qn("a:r"), qn("a:br"))]
                rows.append(["P", *key, str(pi), str(len(elements))])
                actual_runs = list(paragraph.runs)
                ordinary = [e for e in elements if e.tag == qn("a:r")]
                assert len(actual_runs) == len(ordinary), "python-pptx run count"
                for run, element in zip(actual_runs, ordinary):
                    assert run.text == (element.find(qn("a:t")).text or ""), "python-pptx run text"
                    assert run.font.bold is not None and run.font.italic is not None, "explicit style flags"
                    assert run.font.name == element.find(qn("a:rPr")).find(qn("a:latin")).get("typeface"), "resolved face name"
                for ri, element in enumerate(elements):
                    kind = "br" if element.tag == qn("a:br") else "r"
                    properties = element.find(qn("a:rPr"))
                    assert properties is not None, "explicit character properties"
                    face = properties.find(qn("a:latin"))
                    assert face is not None, "explicit font family"
                    text = "" if kind == "br" else (element.find(qn("a:t")).text or "")
                    rows.append(["R", *key, str(pi), str(ri), kind, text_hex(text), properties.get("b"), properties.get("i"), text_hex(face.get("typeface"))])
    return rows


def inspect(path, manifest):
    with manifest.open(newline="") as stream:
        expected = list(csv.reader(stream, delimiter="\t"))
    actual = readback(path)
    if expected != actual:
        for index, (left, right) in enumerate(zip(expected, actual)):
            if left != right:
                raise AssertionError(f"text row {index}: expected {left}, got {right}")
        raise AssertionError(f"text row count: expected {len(expected)}, got {len(actual)}")
    return {"blocks": sum(r[0] == "B" for r in actual), "paragraphs": sum(r[0] == "P" for r in actual), "runs": sum(r[0] == "R" for r in actual)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=Path, default=Path("target/release/misterdiagram"))
    parser.add_argument("--manifest-binary", type=Path, default=Path("target/debug/examples/text_manifest"))
    parser.add_argument("--out", type=Path)
    parser.add_argument("--inspect", type=Path)
    parser.add_argument("--expected", type=Path)
    parser.add_argument("inputs", nargs="*", type=Path)
    args = parser.parse_args()
    if args.inspect:
        assert args.expected is not None
        print(json.dumps(inspect(args.inspect, args.expected)))
        return
    assert args.out is not None
    args.out.mkdir(parents=True, exist_ok=False)
    paths = sorted(path for item in args.inputs for path in (item.glob("*/*.vsdx") if item.is_dir() else [item]))
    assert paths, "empty text witness set"
    rows = []
    totals = dict(blocks=0, paragraphs=0, runs=0)
    for index, source in enumerate(paths):
        output = args.out / f"text-{index:04}.pptx"
        manifest = args.out / f"text-{index:04}.tsv"
        row = dict(source=str(source))
        rows.append(row)
        try:
            for command in ([str(args.manifest_binary), str(source), str(manifest)], [str(args.cli), "render", str(source), str(output)]):
                result = subprocess.run(command, capture_output=True, timeout=120)
                assert result.returncode == 0, result.stderr.decode(errors="replace")
            counts = inspect(output, manifest)
            row.update(counts)
            for key, count in counts.items():
                totals[key] += count
        except Exception as error:
            row["error"] = str(error)
        print(json.dumps(row), flush=True)
    failed = sum("error" in row for row in rows)
    report = dict(total=len(rows), passed=len(rows)-failed, failed=failed, **totals, rows=rows)
    (args.out / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({key: value for key, value in report.items() if key != "rows"}))
    raise SystemExit(1 if failed else 0)


if __name__ == "__main__":
    main()
