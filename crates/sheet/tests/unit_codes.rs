//! Keep the unit vocabulary complete against local, unmodified Visio corpora.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use quick_xml::{Reader, events::Event};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

fn vocabulary(text: &str) -> Result<BTreeSet<String>, String> {
    if text.len() > 65536 {
        return Err("unit list too large".into());
    }
    let mut codes = BTreeSet::new();
    for line in text.lines() {
        let code = line.split('#').next().unwrap_or_default().trim();
        if code.is_empty() {
            continue;
        }
        if code.len() > 32 || !code.bytes().all(|b| b.is_ascii_uppercase() || b == b'_') || !codes.insert(code.to_owned()) {
            return Err(format!("invalid or duplicate unit code: {code}"));
        }
    }
    if codes.is_empty() {
        return Err("empty unit list".into());
    }
    Ok(codes)
}

fn packages(path: &Path, depth: usize, result: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    if depth > 16 || result.len() > 20000 {
        return Err("corpus traversal cap exceeded".into());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            return Err("corpus symlink is not allowed".into());
        }
        if kind.is_dir() {
            packages(&entry.path(), depth + 1, result)?;
        } else if entry
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|s| ["vsdx", "vssx", "vstx", "vsdm"].contains(&s.to_ascii_lowercase().as_str()))
        {
            result.push(entry.path());
        }
    }
    Ok(())
}

#[test]
fn local_corpus_unit_codes_are_listed() -> Result<(), Box<dyn std::error::Error>> {
    let known = vocabulary(include_str!("units.txt"))?;
    let corpus = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus");
    let mut files = Vec::new();
    for group in ["generated", "blank", "ms-shipped"] {
        let path = corpus.join(group);
        if path.exists() {
            packages(&path, 0, &mut files)?;
        }
    }
    if files.is_empty() {
        eprintln!("No local corpora: hydrate with cargo xtask corpus --local to run the unit coverage scan.");
        return Ok(());
    }
    let mut observed = BTreeSet::new();
    let mut unknown = BTreeMap::new();
    let mut parts = 0usize;
    for path in &files {
        let file = fs::File::open(path)?;
        if file.metadata()?.len() > 2 * 1024 * 1024 * 1024 {
            return Err("package exceeds cap".into());
        }
        let mut zip = zip::ZipArchive::new(file)?;
        if zip.len() > 50000 {
            return Err("ZIP entry count exceeds cap".into());
        }
        for index in 0..zip.len() {
            let entry = zip.by_index(index)?;
            if !entry.name().ends_with(".xml") {
                continue;
            }
            if entry.size() > 512 * 1024 * 1024 {
                return Err("XML part exceeds cap".into());
            }
            let name = entry.name().to_owned();
            let mut xml = Reader::from_reader(BufReader::new(entry.take(512 * 1024 * 1024 + 1)));
            let mut buffer = Vec::new();
            loop {
                match xml.read_event_into(&mut buffer)? {
                    Event::Start(e) | Event::Empty(e) => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() != b"U" {
                                continue;
                            }
                            let code = attr.decode_and_unescape_value(xml.decoder())?.into_owned();
                            if !known.contains(&code) {
                                unknown.entry(code.clone()).or_insert_with(|| format!("{}#{name}", path.display()));
                            }
                            observed.insert(code);
                        }
                    }
                    Event::Eof => break,
                    _ => {}
                }
                buffer.clear();
            }
            parts += 1;
        }
    }
    assert!(unknown.is_empty(), "unlisted Visio unit codes and witnesses: {unknown:?}");
    println!("Unit coverage: {} packages, {parts} XML parts, {} observed codes, {} listed, 0 unlisted.", files.len(), observed.len(), known.len());
    Ok(())
}

#[test]
fn hostile_unit_lists_are_rejected() {
    for text in ["", "NUM\nNUM", "../NUM", "NUM value", "NURBS\0", "num"] {
        assert!(vocabulary(text).is_err());
    }
    assert!(vocabulary(&"A".repeat(65537)).is_err());
    assert_eq!(vocabulary("NUM # witness\nGUID\n").map(|c| c.len()), Ok(2));
}
