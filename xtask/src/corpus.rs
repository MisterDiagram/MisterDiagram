//! Pinned private corpus hydration. No shipped Microsoft content is fetched here.
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Component, Path};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::corpus_pins::CORPUS_COMMIT;

const MAX_FILES: usize = 10_000;
const MAX_MANIFEST: u64 = 4 * 1024 * 1024;
const MAX_FILE: u64 = 512 * 1024 * 1024;
type Manifest = BTreeMap<String, String>;

fn safe_name(name: &str) -> bool {
    let mut parts = name.split('/');
    matches!(parts.next(), Some("generated" | "blank"))
        && name.contains('/')
        && name.len() <= 1024
        && parts.all(|p| !p.is_empty() && p != "." && p != ".." && p.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b)))
}

fn parse_manifest(text: &str) -> Result<Manifest, String> {
    if text.len() as u64 > MAX_MANIFEST {
        return Err("manifest exceeds size cap".into());
    }
    let mut result = Manifest::new();
    for line in text.lines() {
        let (hash, name) = line.split_once("  ").ok_or("invalid checksum manifest line")?;
        if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) || !safe_name(name) {
            return Err("invalid checksum or unsafe corpus path".into());
        }
        if result.insert(name.to_owned(), hash.to_ascii_lowercase()).is_some() {
            return Err("duplicate corpus path".into());
        }
        if result.len() > MAX_FILES {
            return Err("too many corpus files".into());
        }
    }
    if result.is_empty() {
        return Err("empty corpus manifest".into());
    }
    Ok(result)
}

fn digest(path: &Path) -> Result<String, String> {
    let meta = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !meta.is_file() || meta.len() > MAX_FILE {
        return Err(format!("not a bounded regular file: {}", path.display()));
    }
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?.take(MAX_FILE + 1);
    let mut hash = Sha256::new();
    let mut buf = [0u8; 65536];
    let mut total = 0u64;
    loop {
        let n = file.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        total = total.checked_add(n as u64).ok_or("file size overflow")?;
        if total > MAX_FILE {
            return Err("file exceeds size cap".into());
        }
        hash.update(buf.get(..n).ok_or("read exceeded buffer")?);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn walk(root: &Path, relative: &Path, depth: usize, visited: &mut usize, result: &mut Manifest) -> Result<(), String> {
    *visited = visited.checked_add(1).ok_or("directory entry overflow")?;
    if *visited > MAX_FILES * 2 {
        return Err("too many corpus directory entries".into());
    }
    if depth > 16 {
        return Err("corpus directory nesting exceeds cap".into());
    }
    let path = root.join(relative);
    let meta = fs::symlink_metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    if meta.file_type().is_symlink() {
        return Err(format!("symlink in corpus: {}", path.display()));
    }
    if meta.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            walk(root, &relative.join(entry.file_name()), depth + 1, visited, result)?;
        }
    } else {
        // Manifest keys use forward slashes on every platform; the walk joins with the native separator.
        let name = relative.to_str().ok_or("non-UTF-8 corpus path")?.replace('\\', "/");
        let name = name.as_str();
        if !safe_name(name) || result.len() >= MAX_FILES {
            return Err("unsafe path or corpus file cap exceeded".into());
        }
        result.insert(name.to_owned(), digest(&path)?);
    }
    Ok(())
}

fn inventory(root: &Path) -> Result<Manifest, String> {
    let mut result = Manifest::new();
    let mut visited = 0;
    for name in ["generated", "blank"] {
        walk(root, Path::new(name), 0, &mut visited, &mut result)?;
    }
    Ok(result)
}

fn git(source: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git").arg("-C").arg(source).args(args).output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!("corpus git failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(meta) if meta.is_dir() && !meta.file_type().is_symlink() => Ok(()),
        Ok(_) => Err(format!("not a real directory: {}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => fs::create_dir(path).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn run(root: &Path, args: &[String]) -> Result<(), String> {
    if !args.iter().any(|a| a == "--local") || args.iter().any(|a| a != "--local" && a != "--update-manifest") {
        return Err("usage: cargo xtask corpus --local [--update-manifest]".into());
    }
    let source = root.parent().ok_or("workspace has no parent")?.join("diagramcraft-corpus");
    if git(&source, &["rev-parse", "HEAD"])?.trim() != CORPUS_COMMIT {
        return Err(format!("corpus checkout must be pinned commit {CORPUS_COMMIT}"));
    }
    if !git(&source, &["status", "--porcelain", "--untracked-files=all"])?.trim().is_empty() {
        return Err("corpus checkout must be clean".into());
    }
    let observed = inventory(&source)?;
    let manifest_path = root.join("xtask/corpus.sha256");
    if args.iter().any(|a| a == "--update-manifest") {
        let text: String = observed.iter().map(|(name, hash)| format!("{hash}  {name}\n")).collect();
        fs::write(&manifest_path, text).map_err(|e| e.to_string())?;
    }
    let mut text = String::new();
    fs::File::open(&manifest_path).map_err(|e| e.to_string())?.take(MAX_MANIFEST + 1).read_to_string(&mut text).map_err(|e| e.to_string())?;
    let expected = parse_manifest(&text)?;
    if observed != expected {
        return Err("source corpus differs from checksum manifest".into());
    }
    let destination = root.join("corpus");
    ensure_directory(&destination)?;
    for name in expected.keys() {
        let relative = Path::new(name);
        let parent = relative.parent().ok_or("missing corpus parent")?;
        let mut dir = destination.clone();
        for component in parent.components() {
            let Component::Normal(part) = component else {
                return Err("unsafe corpus component".into());
            };
            dir.push(part);
            ensure_directory(&dir)?;
        }
        let target = destination.join(relative);
        if let Ok(meta) = fs::symlink_metadata(&target)
            && (!meta.is_file() || meta.file_type().is_symlink())
        {
            return Err("destination must be a regular file".into());
        }
        fs::copy(source.join(relative), &target).map_err(|e| e.to_string())?;
    }
    if inventory(&destination)? != expected {
        return Err("copied corpus differs from checksum manifest".into());
    }
    println!("Corpus {CORPUS_COMMIT}: {} files copied and SHA256 verified (generated + blank).", expected.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostile_manifests_are_rejected() {
        let hash = "a".repeat(64);
        for name in ["/generated/x", "generated/../x", "generated//x", "generated/./x", "generated/x\\y", "other/x", "generated/", "generated/x y"] {
            assert!(parse_manifest(&format!("{hash}  {name}\n")).is_err(), "{name}");
        }
        assert!(parse_manifest("").is_err());
        assert!(parse_manifest("bad  generated/x").is_err());
        assert!(parse_manifest(&"x".repeat(MAX_MANIFEST as usize + 1)).is_err());
        assert!(parse_manifest(&format!("{hash}  generated/x\n{hash}  generated/x\n")).is_err());
    }
    #[test]
    fn manifests_preserve_checksums_and_sorted_paths() -> Result<(), String> {
        let a = "a".repeat(64);
        let b = "b".repeat(64);
        let manifest = parse_manifest(&format!("{a}  generated/a.vsdx\n{b}  blank/blank.vsdx\n"))?;
        assert_eq!(manifest.len(), 2);
        assert_eq!(manifest.get("generated/a.vsdx"), Some(&a));
        assert_eq!(manifest.keys().next().map(String::as_str), Some("blank/blank.vsdx"));
        Ok(())
    }
    #[test]
    fn inventory_detects_tampering_and_rejects_symlinks() -> Result<(), String> {
        let root = crate::root().join("target").join(format!("corpus-test-{}", std::process::id()));
        fs::create_dir_all(root.join("generated")).map_err(|e| e.to_string())?;
        fs::create_dir_all(root.join("blank")).map_err(|e| e.to_string())?;
        let file = root.join("generated/probe.vsdx");
        fs::write(&file, b"abc").map_err(|e| e.to_string())?;
        let before = inventory(&root)?;
        assert_eq!(before.get("generated/probe.vsdx").map(String::as_str), Some("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"));
        fs::write(&file, b"changed").map_err(|e| e.to_string())?;
        assert_ne!(inventory(&root)?, before);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&file, root.join("blank/link")).map_err(|e| e.to_string())?;
            assert!(inventory(&root).is_err());
        }
        fs::remove_dir_all(root).map_err(|e| e.to_string())?;
        Ok(())
    }
}
