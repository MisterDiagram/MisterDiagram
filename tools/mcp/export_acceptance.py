#!/usr/bin/env python3
"""Build the three M5 witnesses through fresh MCP processes and check PPTX exports.

Contributor-original, MIT OR Apache-2.0. Requires the release CLI, exporter examples,
python-pptx, pypdf and LibreOffice. Never launches PowerPoint or a Windows VM.
"""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import subprocess
import sys
from pypdf import PdfReader
from client import Client
import check_pptx
import check_pptx_text
import check_pptx_glue

ROOT = Path(__file__).resolve().parents[2]
WITNESSES = [
    ('office', 'office-network.py', 'office-network', 18),
    ('san', 'san-topology.py', 'p5a-san-topology', 80),
    ('fabric', 'fabric-page.py', 'fabric-page', 20),
]


def run(command, env=None):
    result = subprocess.run([str(x) for x in command], capture_output=True, text=True, timeout=180, env=env)
    if result.returncode:
        raise RuntimeError(f'{command}: exit {result.returncode}\n{result.stdout}\n{result.stderr}')
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cli', type=Path, default=ROOT/'target/release/misterdiagram')
    parser.add_argument('--examples', type=Path, default=ROOT/'target/release/examples')
    parser.add_argument('--out', type=Path, default=ROOT/'corpus/_state/m5-export')
    args = parser.parse_args()
    cli = args.cli.resolve()
    examples = args.examples.resolve()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    temporary = out/'tmp'
    temporary.mkdir()
    env = dict(os.environ, TMPDIR=str(temporary))
    results = []
    for name, script, stem, expected_ends in WITNESSES:
        destination = out/name
        row = dict(name=name, expected_ends=expected_ends)
        results.append(row)
        try:
            row['build_stdout'] = run([sys.executable, ROOT/'tools/mcp'/script, destination, '--cli', cli], env)
            source = destination/(stem+'.vsdx')
            output = destination/(stem+'.pptx')
            with Client(cli) as client:
                client.command('file.open', path=str(source))
                client.tool('export', path=str(output))
            expected_pages = check_pptx.foreground(source)
            row['slides'] = check_pptx.inspect(output, expected_pages)
            for kind in ('text', 'glue'):
                manifest = destination/(kind+'.tsv')
                run([examples/(kind+'_manifest'), source, manifest], env)
                checker = check_pptx_text if kind == 'text' else check_pptx_glue
                row[kind] = checker.inspect(output, manifest)
            assert row['glue']['glued_ends'] == expected_ends, 'MCP attachment count'
            row['model'] = run([examples/'model_check', source], env)
            summary = re.fullmatch(r'PPTX model: (\d+) files, (\d+) pages, (\d+) identical, (\d+) substitutions excluded \(ceiling \d+\), (\d+) failures', row['model'].splitlines()[0])
            assert summary and tuple(map(int, summary.groups())) == (1, expected_pages, expected_pages, 0, 0), 'these MCP witnesses must stay exact'
            (destination/'model.txt').write_text(row['model'])
            pdf = destination/'pdf'
            pdf.mkdir()
            row['soffice_stdout'] = run(['/opt/homebrew/bin/soffice',
                f'-env:UserInstallation={(out/"libreoffice-profile").as_uri()}',
                '--headless', '--convert-to', 'pdf', '--outdir', pdf, output], env)
            row['pdf_pages'] = len(PdfReader(pdf/(stem+'.pdf')).pages)
            assert row['pdf_pages'] == expected_pages, 'PDF page count'
            row['sha256'] = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in (source, output)}
            row['passed'] = True
        except Exception as error:
            row['passed'] = False
            row['error'] = str(error)
        print(json.dumps(row), flush=True)
    passed = sum(row['passed'] for row in results)
    report = dict(passed=passed, total=len(results), native_acceptance='Pending Claude Code and Charles: PowerPoint and Visio', results=results)
    (out/'report.json').write_text(json.dumps(report, indent=2)+'\n')
    lines = ['# M5 export acceptance', '', f'{passed}/{len(results)} drawings pass the local export gates.', '',
             '| Drawing | Slides/PDF pages | Glued ends | Labels grouped/2-D | Text blocks/paragraphs/runs | Output-model pixels |',
             '|---|---:|---:|---:|---|---|']
    for row in results:
        if row['passed']:
            text = row['text']
            lines.append(f'| {row["name"]} | {row["slides"]}/{row["pdf_pages"]} | {row["glue"]["glued_ends"]} | {row["glue"]["grouped_labels"]}/{row["glue"]["labelled_shapes"]} | {text["blocks"]}/{text["paragraphs"]}/{text["runs"]} | identical |')
        else:
            lines.append(f'| {row["name"]} | FAIL | | | | See report.json |')
    lines += ['', 'Each drawing was built from a fresh MCP process and exported through MCP.',
              'python-pptx checks package, text, attachment indexes and immediate shape/label parents; LibreOffice checks PDF page counts.',
              'PowerPoint open, render and drag acceptance and Visio unrepaired-open acceptance remain pending.',
              'The three drawings are witnesses of the general M5 gate, not its definition.']
    (out/'report.md').write_text('\n'.join(lines)+'\n')
    return 0 if passed == len(results) else 1


if __name__ == '__main__':
    raise SystemExit(main())
