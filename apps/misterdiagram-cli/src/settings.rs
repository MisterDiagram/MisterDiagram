//! CLI retention settings, deliberately separate from all file reader limits.
use misterdiagram_engine::MemoryLimits;
use std::ffi::OsString;

pub fn parse(args: &[OsString], environment: Option<OsString>) -> Result<(Vec<OsString>, MemoryLimits), String> {
    let mut filtered = Vec::new();
    let mut flag = None;
    let mut i = 0;
    while let Some(arg) = args.get(i) {
        if arg == "--stencil-path" {
            filtered.push(arg.clone());
            i += 1;
            filtered.push(args.get(i).cloned().ok_or("--stencil-path needs a folder list")?);
        } else if arg == "--memory-limit" {
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
/// Match the operating system path-list syntax, including Windows quoted paths.
pub fn stencil_path(args: &[OsString], environment: Option<OsString>) -> Result<(Vec<OsString>, Vec<std::path::PathBuf>), String> {
    let mut filtered = Vec::new();
    let mut flag = None;
    let mut i = 0;
    while let Some(arg) = args.get(i) {
        if arg == "--stencil-path" {
            if flag.is_some() {
                return Err("duplicate --stencil-path".into());
            }
            i += 1;
            flag = Some(args.get(i).cloned().ok_or("--stencil-path needs a folder list")?);
        } else {
            filtered.push(arg.clone());
        }
        i += 1;
    }
    let mut folders = Vec::new();
    if let Some(value) = flag.or(environment) {
        if value.as_encoded_bytes().len() > 64 * 1024 || value.as_encoded_bytes().contains(&0) {
            return Err("stencil search path byte limit or NUL".into());
        }
        for folder in std::env::split_paths(&value) {
            if folders.len() >= 256 || folder.as_os_str().as_encoded_bytes().len() > 32768 {
                return Err("stencil search folder count or byte limit".into());
            }
            folders.push(folder);
        }
    }
    Ok((filtered, folders))
}
#[cfg(test)]
mod stencil_tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn platform_separator_and_flag_precedence() {
        let paths = [PathBuf::from("first"), PathBuf::from("second folder")];
        let joined = std::env::join_paths(&paths).unwrap();
        let args = [OsString::from("mcp"), OsString::from("--stencil-path"), joined.clone()];
        let (kept, parsed) = stencil_path(&args, Some("ignored".into())).unwrap();
        assert_eq!(kept, vec![OsString::from("mcp")]);
        assert_eq!(parsed, paths);
        assert_eq!(stencil_path(&["run".into()], Some(joined)).unwrap().1, paths);
        #[cfg(windows)]
        assert_eq!(stencil_path(&[], Some("first;second".into())).unwrap().1, [PathBuf::from("first"), PathBuf::from("second")]);
        #[cfg(not(windows))]
        assert_eq!(stencil_path(&[], Some("first:second".into())).unwrap().1, [PathBuf::from("first"), PathBuf::from("second")]);
        let (kept, _) = parse(&["mcp".into(), "--stencil-path".into(), "--memory-limit".into()], None).unwrap();
        assert_eq!(stencil_path(&kept, None).unwrap().1, vec![PathBuf::from("--memory-limit")]);
        assert_eq!(stencil_path(&[], None).unwrap().1, Vec::<PathBuf>::new());
        assert_eq!(stencil_path(&[], Some("".into())).unwrap().1, vec![PathBuf::new()]);
    }
    #[test]
    fn hostile_path_list_is_rejected_before_collection() {
        for value in ["x".repeat(1024 * 1024 + 1), "x".repeat(32769), "bad\0path".into()] {
            assert!(stencil_path(&[], Some(value.into())).is_err());
        }
        let many = std::env::join_paths(vec!["x"; 257]).unwrap();
        assert!(stencil_path(&[], Some(many)).is_err());
        assert!(stencil_path(&["--stencil-path".into()], None).is_err());
        assert!(stencil_path(&["--stencil-path".into(), "a".into(), "--stencil-path".into(), "b".into()], None).is_err());
    }
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
