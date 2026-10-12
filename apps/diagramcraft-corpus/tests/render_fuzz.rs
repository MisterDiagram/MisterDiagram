//! Renderer mutation harness, extended from the attributed vsdx malformed-input harness.
//! Original: deckcraft crates/pptx/tests/malformed.rs, be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
//! Every local corpus package participates in deterministic XML and ZIP mutations.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use diagramcraft_vsdx::import;
use std::{
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};
fn zip_of(es: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut z = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (n, d) in es {
        z.start_file(n, zip::write::SimpleFileOptions::default()).expect("start");
        z.write_all(d).expect("write");
    }
    z.finish().expect("finish").into_inner()
}
fn entries(b: &[u8]) -> Vec<(String, Vec<u8>)> {
    let mut z = zip::ZipArchive::new(Cursor::new(b)).expect("zip");
    (0..z.len())
        .map(|i| {
            let mut f = z.by_index(i).expect("entry");
            let n = f.name().to_string();
            let mut d = vec![];
            f.read_to_end(&mut d).expect("read");
            (n, d)
        })
        .collect()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    if !p.exists() {
        return;
    }
    for entry in std::fs::read_dir(p).expect("dir") {
        let entry = entry.expect("entry");
        let ty = entry.file_type().expect("type");
        assert!(!ty.is_symlink());
        let p = entry.path();
        if ty.is_dir() {
            walk(&p, out)
        } else if p.extension().and_then(|s| s.to_str()).is_some_and(|s| matches!(s.to_ascii_lowercase().as_str(), "vsdx" | "vssx" | "vstx" | "vsdm"))
        {
            out.push(p);
        }
    }
}
fn corpus() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let mut files = vec![];
    for dir in ["generated", "blank", "ms-shipped"] {
        walk(&root.join(dir), &mut files)
    }
    files.sort();
    files
}
fn exercise(bytes: &[u8]) {
    if let Ok(doc) = import(bytes) {
        let options = diagramcraft_render::scene::Options::default();
        let scene = if doc.pages.is_empty() {
            diagramcraft_render::scene::master(&doc, 0, options)
        } else {
            diagramcraft_render::scene::page(&doc, 0, options)
        };
        if let Ok(scene) = scene {
            let _ = diagramcraft_render::raster::render(&scene, options);
        }
    }
}
#[test]
fn corpus_renderer_and_seeded_mutations_never_panic() {
    let files = corpus();
    if !Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus").exists() {
        eprintln!("local corpus absent: corpus mutation pass skipped");
        return;
    }
    assert!(!files.is_empty(), "existing corpus must contain mutation inputs");
    let mut seed = 0x5eed_u64;
    let mut rnd = |n: usize| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n.max(1) as u64) as usize
    };
    let mut mutations = 0;
    for file in &files {
        let bytes = std::fs::read(file).expect("read");
        exercise(&bytes);
        for cut in [bytes.len() / 2, bytes.len().saturating_sub(1)] {
            exercise(&bytes[..cut]);
            mutations += 1;
        }
        let mut es = entries(&bytes);
        let candidates: Vec<usize> =
            es.iter().enumerate().filter(|(_, e)| e.0.ends_with(".xml") && e.0.starts_with("visio/")).map(|(i, _)| i).collect();
        let k = candidates[rnd(candidates.len())];
        let d = &mut es[k].1;
        for _ in 0..1 + rnd(8) {
            if d.is_empty() {
                break;
            }
            let at = rnd(d.len());
            match rnd(4) {
                0 => d[at] = rnd(256) as u8,
                1 => d.truncate(at),
                2 => d.insert(at, b"<>\"&="[rnd(5)]),
                _ => {
                    let end = (at + rnd(64)).min(d.len());
                    d.drain(at..end);
                }
            }
        }
        exercise(&zip_of(&es));
        mutations += 1;
        let mut b = bytes.clone();
        for _ in 0..1 + rnd(16) {
            let at = rnd(b.len());
            b[at] = rnd(256) as u8;
        }
        exercise(&b);
        mutations += 1;
    }
    eprintln!("renderer malformed: {} corpus files parsed; {mutations} seeded mutations; zero panics", files.len());
}
