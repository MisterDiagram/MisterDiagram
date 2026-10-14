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
    pub text: Rate,
    failures: BTreeMap<String, (u64, String)>,
    omitted_observations: u64,
}
#[derive(Debug)]
pub struct Coverage {
    old: BTreeSet<String>,
    seen: BTreeSet<String>,
    shipped_members: BTreeSet<String>,
    combined_members: Option<BTreeSet<String>>,
    pub populations: [Population; 3],
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
        Ok(Self { old, seen: BTreeSet::new(), shipped_members: BTreeSet::new(), combined_members: None, populations: Default::default() })
    }
    /// Freeze a diagnostic rebase cohort without changing the full oracle population.
    pub fn with_combined(mut self, text: &str) -> Result<Self, String> {
        let members = Self::from_list(text)?.old;
        if !self.old.is_subset(&members) {
            return Err("old cohort is not contained in combined cohort".into());
        }
        self.combined_members = Some(members);
        Ok(self)
    }
    pub(crate) fn dump(&mut self, root: &Path, source: &Path, shipped: bool) -> Result<usize, String> {
        let index = if shipped {
            let key = source.strip_prefix(root.join("ms-shipped")).map_err(|e| e.to_string())?.to_string_lossy().to_lowercase();
            self.shipped_members.insert(key.clone());
            if self.old.contains(&key) {
                self.seen.insert(key);
                0
            } else if self.combined_members.as_ref().is_some_and(|members| !members.contains(&key)) {
                2
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
            if calls.iter().any(|n| diagramcraft_sheet::is_text_metric(n)) {
                p.text.record(matched);
            }
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
        if self.combined_members.as_ref().is_some_and(|members| !members.is_subset(&self.shipped_members)) {
            return Err("missing pinned combined dumps".into());
        }
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
        for (name, p) in ["old", "new", "outside"].iter().zip(&self.populations) {
            writeln!(
                f,
                "COM {name}: dumps={} passed={}/{} rate={:.9}% excluded={}",
                p.dumps,
                p.rate.matched,
                p.rate.total,
                p.rate.percent(),
                p.rate.excluded
            )?;
            writeln!(f, "COM text {name}: passed={}/{} rate={:.9}%", p.text.matched, p.text.total, p.text.percent())?;
            writeln!(f, "COM {name}: omitted_function_observations={}", p.omitted_observations)?;
            if *name != "outside" {
                combined.matched += p.rate.matched;
                combined.total += p.rate.total;
            }
            let mut failures: Vec<_> = p.failures.iter().collect();
            failures.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
            for (function, (count, witness)) in failures.into_iter().take(20) {
                writeln!(f, "COM {name} failure {function}: {count} witness={witness}")?;
            }
        }
        writeln!(f, "COM combined: passed={}/{} rate={:.9}%", combined.matched, combined.total, combined.percent())?;
        for member in &self.shipped_members {
            let label = if self.combined_members.as_ref().is_some_and(|members| !members.contains(member)) { "outside" } else { "shipped" };
            writeln!(f, "COM {label} member: {member}")?;
        }
        Ok(())
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
    if (total, matched, excluded, dumps)
        != (report.com_directory.total, report.com_directory.matched, report.com_directory.excluded, report.dump_files)
    {
        return Err("COM partition does not reconcile".into());
    }
    let text_total: u64 = coverage.populations.iter().map(|p| p.text.total).sum();
    let text_matched: u64 = coverage.populations.iter().map(|p| p.text.matched).sum();
    if (text_total, text_matched) != (report.com_directory_hosts[1].total, report.com_directory_hosts[1].matched) {
        return Err("COM text partition does not reconcile".into());
    }
    Ok(coverage)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_coverage_is_measured_but_cannot_enter_the_pinned_rebase() {
        let mut c = Coverage::from_list("1033/a.vssx").unwrap().with_combined("1033/a.vssx\n1033/b.vssx").unwrap();
        assert!(c.complete().is_err());
        for (file, index) in [("a", 0), ("b", 1), ("c", 2)] {
            assert_eq!(c.dump(Path::new("corpus"), &Path::new("corpus/ms-shipped/1033").join(format!("{file}.vssx")), true).unwrap(), index);
            c.record(index, true, &BTreeSet::from(["TEXTWIDTH".into()]), None);
        }
        assert!(c.complete().is_ok());
        assert_eq!(c.populations.iter().map(|p| p.rate.total).sum::<u64>(), 3);
        assert!(c.to_string().contains("COM combined: passed=2/2"));
        assert!(c.to_string().contains("COM outside member: 1033/c.vssx"));
        assert!(Coverage::from_list("a.vssx").unwrap().with_combined("b.vssx").is_err());
    }
    #[test]
    fn text_partition_counts_only_text_calls_in_each_cohort() {
        let mut c = Coverage::from_list("a.vssx").unwrap();
        c.record(0, true, &BTreeSet::from(["TEXTWIDTH".into()]), None);
        c.record(0, false, &BTreeSet::from(["TEXTHEIGHT".into()]), None);
        c.record(0, false, &BTreeSet::from(["SUM".into()]), None);
        c.record(1, true, &BTreeSet::from(["SHAPETEXT".into(), "TEXTWIDTH".into()]), None);
        assert_eq!((c.populations[0].text.matched, c.populations[0].text.total), (1, 2));
        assert_eq!((c.populations[1].text.matched, c.populations[1].text.total), (1, 1));
        assert_eq!((c.populations[0].rate.matched, c.populations[0].rate.total), (1, 3));
    }
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
        assert_eq!(c.shipped_members, BTreeSet::from(["1033/a.vssx".into(), "1033/b.vssx".into()]));
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
