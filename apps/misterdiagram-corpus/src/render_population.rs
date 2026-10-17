//! Charles's 2026-10-15 ruling pins the pixel ratchet to M3.0 membership.
//! https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};
pub(crate) const MEMBERS: &str = include_str!("../tests/cohorts/m3-0-render.txt");
const SHA: &str = "110f12b555cf9afb250f7deaffbc1df21cea6368b647a974264fceed12958267";
const COUNT: usize = 204;

pub(crate) fn frozen() -> Result<BTreeSet<&'static str>, String> {
    parse(MEMBERS, SHA)
}
fn parse<'a>(text: &'a str, sha: &str) -> Result<BTreeSet<&'a str>, String> {
    if text.len() > 1024 * 1024 || format!("{:x}", Sha256::digest(text.as_bytes())) != sha {
        return Err("render population hash or byte limit".into());
    }
    let mut members = BTreeSet::new();
    for line in text.lines() {
        let path = Path::new(line);
        if line.len() > 4096
            || line.contains('\\')
            || !path.components().all(|c| matches!(c, Component::Normal(_)))
            || !line.starts_with("generated/")
            || path.extension().is_none_or(|e| e != "vsdx")
            || !members.insert(line)
        {
            return Err("invalid render population member".into());
        }
    }
    if members.len() != COUNT {
        return Err("render population count".into());
    }
    Ok(members)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pinned_render_population_rejects_added_member() {
        let members = frozen().unwrap();
        assert_eq!(members.len(), 204);
        assert!(members.contains("generated/line/compound-1.vsdx"));
        for later in [
            "generated/route/offset-left.vsdx",
            "generated/data/property-rows.vsdx",
            "generated/master/from-shape.vsdx",
            "generated/theme/builtin.vsdx",
        ] {
            assert!(!members.contains(later));
        }
        assert_eq!(include_str!("../tests/cohorts/m3-0-render.sha256"), format!("{SHA}  m3-0-render.txt\n"));
        assert!(parse(&format!("{MEMBERS}generated/route/offset-left.vsdx\n"), SHA).is_err());
    }
    #[test]
    fn hostile_render_population_is_rejected() {
        for bad in ["", "../escape.vsdx\n", "/generated/file.vsdx\n", "generated/../escape.vsdx\n", "generated/a.vsdx\ngenerated/a.vsdx\n"] {
            assert!(parse(bad, &format!("{:x}", Sha256::digest(bad.as_bytes()))).is_err());
        }
        assert!(parse(&"a".repeat(1024 * 1024 + 1), SHA).is_err());
    }
}
