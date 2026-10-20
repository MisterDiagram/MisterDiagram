#!/usr/bin/env python3
"""Timing-only probes for a disposable current strict checkout. MIT OR Apache-2.0."""
from pathlib import Path
import shutil
import sys

repo = Path(__file__).resolve().parents[2]
root = Path(sys.argv[1]).resolve()
if root == repo or repo / 'target' not in root.parents:
    raise SystemExit('instrument only a disposable checkout below repo/target')
source = root / 'crates/engine/src/lib.rs'
s = source.read_text()


def replace(old, new):
    global s
    if s.count(old) != 1:
        raise SystemExit(f'expected exactly one instrumentation site: {old!r}')
    s = s.replace(old, new)


replace('        let mut before = self.clone();',
        '        let clock = std::time::Instant::now();\n'
        '        let mut times = [0.0_f64; 5];\n'
        '        let mut before = self.clone();\n'
        '        times[0] = clock.elapsed().as_secs_f64();')
replace('                let mut glue = misterdiagram_route::GlueHost::prepare',
        '                let clock = std::time::Instant::now();\n'
        '                let mut glue = misterdiagram_route::GlueHost::prepare')
replace('                misterdiagram_route::reroute(&before.document',
        '                times[1] = clock.elapsed().as_secs_f64();\n'
        '                let clock = std::time::Instant::now();\n'
        '                misterdiagram_route::reroute(&before.document')
replace('                let theme = misterdiagram_theme::Themes::from_document(&self.document)',
        '                times[2] = clock.elapsed().as_secs_f64();\n'
        '                let clock = std::time::Instant::now();\n'
        '                let theme = misterdiagram_theme::Themes::from_document(&self.document)')
replace('                let held_nodes = self.retained_xml_nodes();',
        '                times[3] = clock.elapsed().as_secs_f64();\n'
        '                let clock = std::time::Instant::now();\n'
        '                let held_nodes = self.retained_xml_nodes();')
replace('                files::update_palette(Arc::make_mut(&mut self.document))?;',
        '                times[4] = clock.elapsed().as_secs_f64();\n'
        '                files::update_palette(Arc::make_mut(&mut self.document))?;')
replace('        match result {\n            Ok(Ok(value))',
        '        eprintln!("{}", serde_json::json!({"kind":"stages", "clone":times[0], "prepare":times[1], '
        '"reroute":times[2], "write":times[3], "import":times[4]}));\n'
        '        match result {\n            Ok(Ok(value))')
replace('        f(Arc::make_mut(&mut self.document), &mut self.selection)',
        '        let clock = std::time::Instant::now();\n'
        '        let doc = Arc::make_mut(&mut self.document);\n'
        '        eprintln!("{}", serde_json::json!({"kind":"cow_clone","seconds":clock.elapsed().as_secs_f64()}));\n'
        '        f(doc, &mut self.selection)')
source.write_text(s)
bench = root / 'tools/bench'
bench.mkdir(exist_ok=True)
shutil.copyfile(repo / 'tools/bench/authoring.rs', bench / 'authoring.rs')
manifest = root / 'crates/engine/Cargo.toml'
temporary = manifest.with_suffix('.bench.tmp')
temporary.write_text(manifest.read_text() + '\n[[example]]\nname="authoring-bench"\npath="../../tools/bench/authoring.rs"\n')
temporary.replace(manifest)
