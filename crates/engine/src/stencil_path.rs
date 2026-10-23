//! Ordered, non-recursive stencil discovery. Admission remains in stencils::open.
use crate::{Result, operation};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub(super) fn bare(path: &Path) -> bool {
    let Some(name) = path.to_str() else { return false };
    !name.is_empty()
        && name != "."
        && !name.contains("..")
        && !name.contains(['/', '\\', '\0'])
        && !(name.as_bytes().first().is_some_and(u8::is_ascii_alphabetic) && name.as_bytes().get(1) == Some(&b':'))
}
fn folders(configured: &[PathBuf]) -> Vec<PathBuf> {
    if configured.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        configured.iter().map(|p| if p.as_os_str().is_empty() { PathBuf::from(".") } else { p.clone() }).collect()
    }
}
pub(crate) fn resolve(configured: &[PathBuf], path: &Path) -> Result<PathBuf> {
    if !bare(path) {
        return path.canonicalize().map_err(operation);
    }
    let folders = folders(configured);
    for folder in &folders {
        let first = folder.join(path);
        let mut candidates = vec![first];
        if !path.extension().and_then(|e| e.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("vssx")) {
            let mut name = path.as_os_str().to_os_string();
            name.push(".vssx");
            candidates.push(folder.join(name));
        }
        for candidate in candidates {
            match candidate.metadata() {
                Ok(metadata) if metadata.is_file() => return candidate.canonicalize().map_err(operation),
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(operation(format!("stencil search {}: {error}", candidate.display()))),
            }
        }
    }
    Err(operation(format!(
        "stencil {} not found; searched folders: {}",
        path.display(),
        folders.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
    )))
}
pub(crate) fn available(configured: &[PathBuf]) -> Result<Value> {
    let folders = folders(configured);
    let mut files = vec![];
    let mut bytes = 0usize;
    let mut visited = 0usize;
    for folder in &folders {
        let entries = match std::fs::read_dir(folder) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(operation(format!("stencil folder {}: {error}", folder.display()))),
        };
        let absolute = folder.canonicalize().map_err(operation)?;
        let mut rows = vec![];
        for entry in entries {
            visited = visited.saturating_add(1);
            if visited > 100_000 {
                return Err(operation("stencil directory entry limit"));
            }
            let entry = entry.map_err(operation)?;
            let path = entry.path();
            if !path.extension().and_then(|e| e.to_str()).is_some_and(|ext| ext.eq_ignore_ascii_case("vssx")) {
                continue;
            }
            let metadata = std::fs::metadata(&path).map_err(operation)?;
            if !metadata.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            bytes = bytes.saturating_add(name.len()).saturating_add(absolute.as_os_str().as_encoded_bytes().len());
            if files.len() + rows.len() >= 10_000 || bytes > 8 * 1024 * 1024 {
                return Err(operation("stencil available result limit"));
            }
            rows.push((name, metadata.len()));
        }
        rows.sort();
        files.extend(rows.into_iter().map(|(name, size)| json!({"name":name,"folder":absolute.to_string_lossy(),"size":size})));
    }
    Ok(json!({"files":files,"folders":folders.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>()}))
}

impl crate::Session {
    /// Configure ordered search folders; empty means the process working directory.
    /// The CLI splits MISTERDIAGRAM_STENCIL_PATH with std::env::split_paths.
    pub fn set_stencil_path(&mut self, folders: Vec<PathBuf>) -> Result<()> {
        let mut bytes = 0usize;
        if folders.len() > 256 {
            return Err(operation("stencil search folder count limit (256)"));
        }
        for folder in &folders {
            let raw = folder.as_os_str().as_encoded_bytes();
            bytes = bytes.saturating_add(raw.len());
            if raw.len() > 32768 || raw.contains(&0) || bytes > 64 * 1024 {
                return Err(operation("stencil search path byte limit or NUL"));
            }
        }
        let previous = std::mem::replace(&mut self.stencil_path, folders);
        if let Err(error) = self.reserve(None) {
            self.stencil_path = previous;
            return Err(error);
        }
        Ok(())
    }
}
