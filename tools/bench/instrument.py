#!/usr/bin/env python3
"""Add timing-only probes to a disposable checkout of the strict engine.

Never use on the working source tree. Baseline contract: the 13897b0 edit path.
The benchmark deliberately does not change the CLI's command limits.
"""
from pathlib import Path
import shutil
import hashlib
import sys
root = Path(sys.argv[1]).resolve()
repo = Path(__file__).resolve().parents[2]
if root == repo or repo / 'target' not in root.parents:
    raise SystemExit('instrument only a disposable checkout under this repo/target')
source = root / 'crates/engine/src/lib.rs'
original = source.read_text()
if hashlib.sha256(original.encode()).hexdigest() != '72245b92578288aef45d48bcf960d2a37e4f97a6b54e8c6c153b28b5bb1a67d4':
    raise SystemExit('unexpected or already instrumented engine')
bench = root / 'tools/bench'
bench.mkdir(parents=True, exist_ok=True)
for name in ('authoring.rs','roundtrip_probe.rs'):
    shutil.copyfile(Path(__file__).parent / name, bench / name)
manifest = root / 'crates/engine/Cargo.toml'
temporary = manifest.with_name('Cargo.bench.tmp')
temporary.write_text(manifest.read_text() + '\n[[example]]\nname = "authoring-bench"\npath = "../../tools/bench/authoring.rs"\n\n[[example]]\nname = "roundtrip-probe"\npath = "../../tools/bench/roundtrip_probe.rs"\n')
temporary.replace(manifest)
p=root/'crates/engine/src/lib.rs';s=p.read_text()
s=s.replace('        let before = self.clone();','        let clock = std::time::Instant::now();\n        let mut times = [0.0_f64; 5];\n        let before = self.clone();\n        times[0] = clock.elapsed().as_secs_f64();')
s=s.replace('                let mut glue = misterdiagram_route::GlueHost::prepare', '                let clock = std::time::Instant::now();\n                let mut glue = misterdiagram_route::GlueHost::prepare')
s=s.replace('                misterdiagram_route::reroute(&before.document', '                times[1] = clock.elapsed().as_secs_f64();\n                let clock = std::time::Instant::now();\n                misterdiagram_route::reroute(&before.document')
s=s.replace('                let theme = misterdiagram_theme::Themes', '                times[2] = clock.elapsed().as_secs_f64();\n                let clock = std::time::Instant::now();\n                let theme = misterdiagram_theme::Themes')
s=s.replace('                self.document = Arc::new(misterdiagram_vsdx::import', '                times[3] = clock.elapsed().as_secs_f64();\n                let clock = std::time::Instant::now();\n                self.document = Arc::new(misterdiagram_vsdx::import')
s=s.replace('                files::update_palette', '                times[4] = clock.elapsed().as_secs_f64();\n                files::update_palette')
s=s.replace('        match result {', '        eprintln!("{}", serde_json::json!({"kind":"stages", "clone":times[0], "prepare":times[1], "reroute":times[2], "write":times[3], "import":times[4]}));\n        match result {',1)
s=s.replace('        f(Arc::make_mut(&mut self.document), &mut self.selection)', '        let clock = std::time::Instant::now();\n        let doc = Arc::make_mut(&mut self.document);\n        eprintln!("{}", serde_json::json!({"kind":"cow_clone","seconds":clock.elapsed().as_secs_f64()}));\n        f(doc, &mut self.selection)')
p.write_text(s)
