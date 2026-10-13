//! Diagnostic partition of the unchanged COM oracle by completed-dump provenance.
use crate::{Rate, Witness};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    path::Path,
};
#[derive(Debug, Default)]
pub struct Population {
    pub dumps: usize,
    pub rate: Rate,
    failures: BTreeMap<String, (u64, String)>,
    omitted_observations: u64,
}
#[derive(Debug)]
pub struct Coverage {
    old: BTreeSet<String>,
    seen: BTreeSet<String>,
    pub populations: [Population; 2],
}
impl Coverage {
    pub fn from_list(text: &str) -> Result<Self, String> {
        if text.len() > 1024 * 1024 {
            return Err("coverage list byte limit".into());
        }
        let mut old = BTreeSet::new();
        for name in text.lines() {
            if name.is_empty()
                || name.len() > 4096
                || name != name.to_lowercase()
                || name.contains('\\')
                || name.split('/').any(|p| p.is_empty() || p == "." || p == "..")
                || !crate::package(Path::new(name))
                || old.len() >= 100_000
                || !old.insert(name.to_owned())
            {
                return Err("invalid or duplicate coverage member".into());
            }
        }
        if old.is_empty() {
            return Err("empty coverage membership".into());
        }
        Ok(Self { old, seen: BTreeSet::new(), populations: Default::default() })
    }
    pub(crate) fn dump(&mut self, root: &Path, source: &Path, shipped: bool) -> Result<usize, String> {
        let index = if shipped {
            let key = source.strip_prefix(root.join("ms-shipped")).map_err(|e| e.to_string())?.to_string_lossy().to_lowercase();
            if self.old.contains(&key) {
                self.seen.insert(key);
                0
            } else {
                1
            }
        } else {
            0
        };
        if let Some(p) = self.populations.get_mut(index) {
            p.dumps += 1;
        }
        Ok(index)
    }
    pub(crate) fn record(&mut self, index: usize, matched: bool, calls: &BTreeSet<String>, witness: Option<&Witness>) {
        if let Some(p) = self.populations.get_mut(index) {
            p.rate.record(matched);
            if let Some(witness) = witness.filter(|_| !matched) {
                let functions = if calls.is_empty() { vec!["<no function>"] } else { calls.iter().map(String::as_str).collect() };
                for function in functions {
                    let key = crate::short(function);
                    if !p.failures.contains_key(&key) && p.failures.len() >= 1000 {
                        p.omitted_observations += 1;
                        continue;
                    }
                    let entry = p.failures.entry(key).or_insert_with(|| {
                        (
                            0,
                            format!(
                                "{} {} {}: {} expected={} actual={}",
                                crate::short(&witness.file),
                                crate::short(&witness.scope),
                                crate::short(&witness.cell),
                                crate::short(&witness.formula),
                                crate::short(&witness.expected),
                                crate::short(&witness.actual)
                            ),
                        )
                    });
                    entry.0 += 1;
                }
            }
        }
    }
    pub(crate) fn excluded(&mut self, index: usize) {
        if let Some(p) = self.populations.get_mut(index) {
            p.rate.excluded += 1;
        }
    }
    fn complete(&self) -> Result<(), String> {
        if self.old != self.seen {
            return Err(format!("missing old dumps: {:?}", self.old.difference(&self.seen).collect::<Vec<_>>()));
        }
        Ok(())
    }
}
impl fmt::Display for Coverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "coverage old_shipped={} found={}", self.old.len(), self.seen.len())?;
        let mut combined = Rate::default();
        for (name, p) in ["old", "new"].iter().zip(&self.populations) {
            writeln!(
                f,
                "COM {name}: dumps={} passed={}/{} rate={:.9}% excluded={}",
                p.dumps,
                p.rate.matched,
                p.rate.total,
                p.rate.percent(),
                p.rate.excluded
            )?;
            writeln!(f, "COM {name}: omitted_function_observations={}", p.omitted_observations)?;
            combined.matched += p.rate.matched;
            combined.total += p.rate.total;
            let mut failures: Vec<_> = p.failures.iter().collect();
            failures.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
            for (function, (count, witness)) in failures.into_iter().take(20) {
                writeln!(f, "COM {name} failure {function}: {count} witness={witness}")?;
            }
        }
        writeln!(f, "COM combined: passed={}/{} rate={:.9}%", combined.matched, combined.total, combined.percent())
    }
}
/// Reuses exactly the main oracle's association, exclusion, evaluation and comparison path.
/// Generated and blank dumps belong to the old population; only shipped coverage grows.
pub fn compare(root: &Path, mut coverage: Coverage) -> Result<Coverage, String> {
    let report = crate::run_pass_with_coverage(root, false, false, Some(&mut coverage))?;
    coverage.complete()?;
    let total: u64 = coverage.populations.iter().map(|p| p.rate.total).sum();
    let matched: u64 = coverage.populations.iter().map(|p| p.rate.matched).sum();
    let excluded: u64 = coverage.populations.iter().map(|p| p.rate.excluded).sum();
    let dumps: usize = coverage.populations.iter().map(|p| p.dumps).sum();
    if (total, matched, excluded, dumps) != (report.com.total, report.com.matched, report.com.excluded, report.dump_files) {
        return Err("COM partition does not reconcile".into());
    }
    Ok(coverage)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn membership_is_bounded_and_must_be_complete() {
        for text in ["", "../a.vssx", "/a.vssx", "1033/A.vssx", "a.vssx\na.vssx", "a.txt"] {
            assert!(Coverage::from_list(text).is_err());
        }
        assert!(Coverage::from_list(&"a".repeat(1024 * 1024 + 1)).is_err());
        let mut c = Coverage::from_list("1033/a.vssx\n").unwrap();
        assert!(c.complete().is_err());
        assert_eq!(c.dump(Path::new("corpus"), Path::new("corpus/generated/a.vsdx"), false).unwrap(), 0);
        assert_eq!(c.dump(Path::new("corpus"), Path::new("corpus/ms-shipped/1033/A.vssx"), true).unwrap(), 0);
        assert_eq!(c.dump(Path::new("corpus"), Path::new("corpus/ms-shipped/1033/b.vssx"), true).unwrap(), 1);
        assert!(c.complete().is_ok());
        assert_eq!((c.populations[0].dumps, c.populations[1].dumps), (2, 1));
    }
    #[test]
    fn diagnostic_storage_is_bounded_for_long_multi_function_rows() {
        let mut c = Coverage::from_list("a.vssx").unwrap();
        let witness = Witness {
            oracle: "COM",
            category: String::new(),
            file: "x".repeat(1_000_000),
            scope: "s".repeat(1_000_000),
            cell: "c".repeat(1_000_000),
            formula: String::new(),
            expected: String::new(),
            actual: String::new(),
        };
        let calls = (0..2000).map(|n| format!("FN{n}")).collect();
        c.record(1, false, &calls, Some(&witness));
        let p = &c.populations[1];
        assert_eq!(p.failures.len(), 1000);
        assert_eq!(p.omitted_observations, 1000);
        assert!(p.failures.values().map(|(_, w)| w.len()).sum::<usize>() < 2_000_000);
        assert_eq!((p.rate.matched, p.rate.total), (0, 1));
    }
}
