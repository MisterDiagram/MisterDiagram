//! Fixed COM populations prevent background dump additions from moving a gate.
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

pub(crate) fn description() -> String {
    format!(
        "apps/diagramcraft-corpus/tests/cohorts/m3-872-dumps.txt sha256={:x}",
        Sha256::digest(include_bytes!("../tests/cohorts/m3-872-dumps.txt"))
    )
}

pub(crate) struct Population {
    members: BTreeSet<String>,
}
impl Population {
    pub fn parse(text: &str, expected_sha: &str) -> Result<Self, String> {
        if text.len() > 1024 * 1024 || format!("{:x}", Sha256::digest(text.as_bytes())) != expected_sha {
            return Err("COM population membership hash or byte limit".into());
        }
        let mut members = BTreeSet::new();
        for line in text.lines() {
            let path = Path::new(line);
            if line.is_empty()
                || line.len() > 4096
                || line.contains('\\')
                || !path.components().all(|c| matches!(c, Component::Normal(_)))
                || !matches!(line.split('/').next(), Some("generated" | "blank" | "ms-shipped-dumps"))
                || path.extension().is_none_or(|e| e != "tsv")
                || !members.insert(line.to_owned())
                || members.len() > 100_000
            {
                return Err("invalid COM population member".into());
            }
        }
        if members.is_empty() {
            return Err("empty COM population".into());
        }
        Ok(Self { members })
    }
    pub fn frozen() -> Result<Self, String> {
        let floors = include_str!("../../../crates/sheet/tests/floors.toml");
        let section =
            floors.split("[m3_27_gate_population]").nth(1).ok_or("missing pinned COM population")?.split("\n[").next().ok_or("population section")?;
        let value = |key| {
            section
                .lines()
                .find_map(|l| {
                    let (k, v) = l.split_once('=')?;
                    (k.trim() == key).then(|| v.trim().trim_matches('"'))
                })
                .ok_or("missing population key")
        };
        if value("dumps_path")? != "apps/diagramcraft-corpus/tests/cohorts/m3-872-dumps.txt" {
            return Err("compiled COM population path disagrees with floor".into());
        }
        let p = Self::parse(include_str!("../tests/cohorts/m3-872-dumps.txt"), value("dumps_sha256")?)?;
        p.complete(value("dumps_count")?.parse().map_err(|_| "invalid population count")?)?;
        Ok(p)
    }
    pub fn complete(&self, count: usize) -> Result<(), String> {
        if count != self.members.len() {
            return Err("pinned COM population was not fully compared".into());
        }
        Ok(())
    }
    pub fn contains(&self, root: &Path, path: &Path) -> bool {
        path.strip_prefix(root).is_ok_and(|p| self.members.contains(p.to_string_lossy().as_ref()))
    }
    pub fn require_available(&self, root: &Path) -> Result<(), String> {
        for member in &self.members {
            if !root.join(member).is_file() {
                return Err(format!("missing pinned COM dump: {member}"));
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn pin(s: &str) -> Result<Population, String> {
        Population::parse(s, &format!("{:x}", Sha256::digest(s.as_bytes())))
    }
    #[test]
    fn membership_is_exact_and_tampering_is_rejected() {
        let p = pin("generated/a.sheet.tsv\n").unwrap();
        assert!(p.contains(Path::new("corpus"), Path::new("corpus/generated/a.sheet.tsv")));
        assert!(!p.contains(Path::new("corpus"), Path::new("corpus/generated/later.sheet.tsv")));
        assert!(Population::parse("generated/a.sheet.tsv\n", &"0".repeat(64)).is_err());
        for bad in ["", "../a.tsv\n", "generated/../a.tsv\n", "/generated/a.tsv\n", "generated/a.tsv\ngenerated/a.tsv\n"] {
            assert!(pin(bad).is_err());
        }
        assert!(p.require_available(Path::new("target/nonexistent-com-pin-fixture")).is_err());
        assert!(p.complete(0).is_err());
        assert!(p.complete(1).is_ok());
        Population::frozen().unwrap().complete(1085).unwrap();
    }
}
