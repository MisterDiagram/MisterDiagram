//! CLI retention settings, deliberately separate from all file reader limits.
use misterdiagram_engine::MemoryLimits;
use std::ffi::OsString;

pub fn parse(args: &[OsString], environment: Option<OsString>) -> Result<(Vec<OsString>, MemoryLimits), String> {
    let mut filtered = Vec::new();
    let mut flag = None;
    let mut i = 0;
    while let Some(arg) = args.get(i) {
        if arg == "--memory-limit" {
            if flag.is_some() {
                return Err("duplicate --memory-limit".into());
            }
            i += 1;
            flag = Some(args.get(i).cloned().ok_or("--memory-limit needs positive whole MiB")?);
        } else {
            filtered.push(arg.clone());
        }
        i += 1;
    }
    let limits = match flag.or(environment) {
        None => MemoryLimits::default(),
        Some(value) => {
            let text = value
                .to_str()
                .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
                .ok_or("--memory-limit / MISTERDIAGRAM_MEMORY_LIMIT must be positive whole MiB")?;
            let mib = text.parse::<usize>().map_err(|_| "memory limit overflow")?;
            MemoryLimits::from_mib(mib).map_err(|e| e.to_string())?
        }
    };
    Ok((filtered, limits))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_precedence_and_hostile_values() {
        let args = ["mcp", "--memory-limit", "2048"].map(Into::into);
        assert_eq!(parse(&args, Some("bad".into())).unwrap().1.mib(), 2048);
        assert_eq!(parse(&["mcp".into()], Some("4096".into())).unwrap().1.mib(), 4096);
        assert_eq!(parse(&["mcp".into()], None).unwrap().1.mib(), 2048);
        for bad in ["", "0", "-1", "NaN", "inf", "1.5", "999999999999999999999999"] {
            assert!(parse(&["mcp".into()], Some(bad.into())).is_err(), "{bad}");
        }
        assert!(parse(&["--memory-limit".into()], None).is_err());
        assert!(parse(&["--memory-limit".into(), "1".into(), "--memory-limit".into(), "2".into()], None).is_err());
    }
}
