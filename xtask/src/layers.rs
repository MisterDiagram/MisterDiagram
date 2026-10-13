//! Dependency layering rules (plan/architecture.md §3).
//!
//! The rule engine works on a small, metadata-independent model so it can be
//! unit-tested; `from_metadata` builds that model from `cargo metadata`.

use serde_json::Value;

/// Where a workspace crate sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Regular layered crate.
    Layer(u8),
    /// Binaries and build tooling: exempt from the rules.
    Exempt,
}

impl Class {
    fn layer(self) -> Option<u8> {
        match self {
            Class::Layer(l) => Some(l),
            Class::Exempt => None,
        }
    }
}

/// The layering table. Names are package names without the `diagramcraft-`
/// prefix.
pub const TABLE: &[(&str, Class)] = &[
    ("geom", Class::Layer(0)),
    ("color", Class::Layer(0)),
    ("opc", Class::Layer(0)),
    ("sheet", Class::Layer(1)),
    ("model", Class::Layer(1)),
    ("fonts", Class::Layer(1)),
    ("text", Class::Layer(2)),
    ("theme", Class::Layer(2)),
    ("vsdx", Class::Layer(2)),
    ("route", Class::Layer(3)),
    ("render", Class::Layer(3)),
    ("export", Class::Layer(4)),
    ("engine", Class::Layer(5)),
    ("mcp", Class::Layer(6)),
    ("diagramcraft", Class::Exempt),
    ("cli", Class::Exempt),
    ("corpus", Class::Exempt),
    ("web", Class::Exempt),
    ("xtask", Class::Exempt),
];
// Integration-only hosts for the named P3 round-trip acceptance test.
pub const DEV_EXCEPTIONS: &[(&str, &str)] = &[("vsdx", "theme"), ("vsdx", "text"), ("model", "vsdx")];

/// Explicit orderings *within* a layer (earlier may be used by later).
pub const INTRA_LAYER_ORDER: &[&[&str]] = &[];

fn intra_layer_allowed(from: &str, to: &str) -> bool {
    let (from, to) = (short_name(from), short_name(to));
    INTRA_LAYER_ORDER.iter().any(|chain| match (chain.iter().position(|n| *n == from), chain.iter().position(|n| *n == to)) {
        (Some(f), Some(t)) => t < f,
        _ => false,
    })
}

/// External crates that constitute a UI toolkit / windowing dependency.
/// Entries ending in `*` are prefixes.
pub const UI_CRATES: &[&str] = &["egui*", "epaint*", "emath", "ecolor", "eframe", "winit", "rfd", "bevy*"];

/// First layer allowed to use UI crates.
pub const UI_MIN_LAYER: u8 = 7;

pub fn short_name(pkg: &str) -> &str {
    pkg.strip_prefix("diagramcraft-").unwrap_or(pkg)
}

pub fn classify(pkg: &str) -> Option<Class> {
    let s = short_name(pkg);
    TABLE.iter().find(|(n, _)| *n == s).map(|(_, c)| *c)
}

fn is_ui_crate(name: &str) -> bool {
    UI_CRATES.iter().any(|p| match p.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => name == *p,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepKind {
    Normal,
    Dev,
    Build,
}

#[derive(Debug, Clone)]
pub struct Dep {
    pub name: String,
    pub kind: DepKind,
    /// `true` if the dependency is a workspace member.
    pub workspace: bool,
}

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub deps: Vec<Dep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    Unregistered { krate: String },
    Lockfile { detail: String },
    Upward { krate: String, dep: String, from: u8, to: u8, kind: DepKind },
    UiBelowApps { krate: String, dep: String, layer: u8 },
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Lockfile { detail } => write!(f, "Cargo.lock: {detail}"),
            Violation::Unregistered { krate } => {
                write!(f, "{krate}: unknown workspace crate; register it in xtask/src/layers.rs TABLE (see plan/architecture.md §3)")
            }
            Violation::Upward { krate, dep, from, to, kind } => {
                write!(f, "{krate} (L{from}) -> {dep} (L{to}) [{kind:?}]: may only depend on strictly lower layers")
            }
            Violation::UiBelowApps { krate, dep, layer } => {
                write!(f, "{krate} (L{layer}) depends on UI crate `{dep}`; UI crate below apps; UI toolkits are only allowed in apps")
            }
        }
    }
}

/// Check all rules. Returns violations sorted for stable output.
fn check_graph(crates: &[Crate]) -> Vec<Violation> {
    let mut out = Vec::new();
    for c in crates {
        let Some(class) = classify(&c.name) else {
            out.push(Violation::Unregistered { krate: c.name.clone() });
            continue;
        };
        if class == Class::Exempt {
            continue;
        }
        let layer = class.layer().unwrap_or(0);
        for d in &c.deps {
            // Self dev-dependencies (e.g. to enable features in tests) are fine.
            if d.name == c.name {
                continue;
            }
            if d.workspace {
                match classify(&d.name) {
                    // Unregistered deps are reported on their own entry.
                    None => {}
                    Some(dc) => {
                        let to = dc.layer().unwrap_or(u8::MAX);
                        let dev_ok = d.kind == DepKind::Dev && DEV_EXCEPTIONS.contains(&(short_name(&c.name), short_name(&d.name)));
                        if to >= layer && !dev_ok && !(to == layer && intra_layer_allowed(&c.name, &d.name)) {
                            out.push(Violation::Upward {
                                krate: c.name.clone(),
                                dep: d.name.clone(),
                                from: layer,
                                to: if to == u8::MAX { 7 } else { to },
                                kind: d.kind,
                            });
                        }
                    }
                }
            } else if layer < UI_MIN_LAYER && is_ui_crate(&d.name) {
                out.push(Violation::UiBelowApps { krate: c.name.clone(), dep: d.name.clone(), layer });
            }
        }
    }
    out.sort_by_key(|v| v.to_string());
    out.dedup();
    out
}

/// Scan Cargo's generated lockfile syntax, including transitive dependencies.
/// Package names are ASCII registry identifiers; escaped or malformed names fail closed.
pub fn check_lock(text: &str) -> Result<Vec<Violation>, String> {
    if text.len() > 8 * 1024 * 1024 {
        return Err("lockfile exceeds size cap".into());
    }
    let mut out = Vec::new();
    let mut packages = 0usize;
    let mut in_package = false;
    let mut named = false;
    for line in text.lines() {
        if line.len() > 65536 {
            return Err("lockfile line exceeds cap".into());
        }
        let line = line.trim();
        if line == "[[package]]" {
            if in_package && !named {
                return Err("package without a name".into());
            }
            packages += 1;
            if packages > 20_000 {
                return Err("too many locked packages".into());
            }
            in_package = true;
            named = false;
        } else if in_package && let Some((key, value)) = line.split_once('=') {
            if key.trim() != "name" {
                continue;
            }
            if named {
                return Err("duplicate package name field".into());
            }
            let value = value.split('#').next().unwrap_or_default().trim();
            let name = value
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                .ok_or("invalid locked package name")?;
            if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)) {
                return Err("invalid locked package identifier".into());
            }
            named = true;
            if is_ui_crate(name) {
                out.push(Violation::Lockfile { detail: format!("UI crate `{name}` is forbidden in the headless lockfile") });
            }
        }
    }
    if packages == 0 || !named {
        return Err("lockfile has no named package".into());
    }
    Ok(out)
}

/// Enforce the graph rules and reject UI crates anywhere in Cargo.lock.
pub fn check(crates: &[Crate]) -> Vec<Violation> {
    use std::io::Read;
    let mut out = check_graph(crates);
    let lock = (|| -> Result<Vec<Violation>, String> {
        let mut text = String::new();
        std::fs::File::open(crate::root().join("Cargo.lock"))
            .map_err(|e| e.to_string())?
            .take(8 * 1024 * 1024 + 1)
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;
        check_lock(&text)
    })();
    match lock {
        Ok(v) => out.extend(v),
        Err(detail) => out.push(Violation::Lockfile { detail }),
    }
    out
}

/// Build the model from `cargo metadata --format-version 1 --no-deps`.
pub fn from_metadata(meta: &Value) -> Result<Vec<Crate>, String> {
    let pkgs = meta["packages"].as_array().ok_or("metadata: no packages array")?;
    let members: Vec<&str> = pkgs.iter().filter_map(|p| p["name"].as_str()).collect();
    let mut out = Vec::new();
    for p in pkgs {
        let name = p["name"].as_str().ok_or("package without name")?.to_owned();
        let mut deps = Vec::new();
        for d in p["dependencies"].as_array().into_iter().flatten() {
            let dname = d["name"].as_str().unwrap_or_default().to_owned();
            let kind = match d["kind"].as_str() {
                Some("dev") => DepKind::Dev,
                Some("build") => DepKind::Build,
                _ => DepKind::Normal,
            };
            let workspace = members.contains(&dname.as_str());
            deps.push(Dep { name: dname, kind, workspace });
        }
        out.push(Crate { name, deps });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn describe(class: Option<Class>) -> String {
    match class {
        Some(Class::Layer(l)) => format!("L{l}"),
        Some(Class::Exempt) => "exempt".into(),
        None => "UNREGISTERED".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(name: &str, deps: &[(&str, DepKind, bool)]) -> Crate {
        Crate { name: name.into(), deps: deps.iter().map(|(n, k, w)| Dep { name: (*n).into(), kind: *k, workspace: *w }).collect() }
    }
    use DepKind::*;

    #[test]
    fn clean_downward_graph_passes() {
        let g = [
            c("diagramcraft-geom", &[("kurbo", Normal, false)]),
            c("diagramcraft-model", &[("diagramcraft-geom", Normal, true)]),
            c("diagramcraft-engine", &[("diagramcraft-model", Normal, true)]),
            c("diagramcraft-mcp", &[("diagramcraft-engine", Normal, true)]),
            c("diagramcraft-cli", &[("diagramcraft-mcp", Normal, true)]),
        ];
        assert!(check_graph(&g).is_empty(), "{:?}", check_graph(&g));
    }

    #[test]
    fn upward_dependency_flagged() {
        let v = check_graph(&[c("diagramcraft-model", &[("diagramcraft-engine", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 1, to: 5, .. }]));
    }

    #[test]
    fn sideways_dependency_flagged() {
        let v = check_graph(&[c("diagramcraft-text", &[("diagramcraft-theme", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 2, to: 2, .. }]));
    }

    #[test]
    fn l0_foundations_are_independent() {
        let v = check_graph(&[c("diagramcraft-geom", &[("diagramcraft-color", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 0, to: 0, .. }]));
    }

    #[test]
    fn self_dev_dependency_ignored() {
        assert!(check_graph(&[c("diagramcraft-model", &[("diagramcraft-model", Dev, true)])]).is_empty());
    }

    #[test]
    fn upward_dev_dependency_flagged() {
        let v = check_graph(&[c("diagramcraft-geom", &[("diagramcraft-model", Dev, true)])]);
        assert!(matches!(v[..], [Violation::Upward { kind: Dev, .. }]));
    }

    #[test]
    fn ui_crates_below_apps_flagged() {
        for dep in ["egui", "eframe", "winit", "egui_kittest", "rfd", "bevy_ecs", "bevy"] {
            let v = check_graph(&[c("diagramcraft-engine", &[(dep, Normal, false)])]);
            assert!(matches!(v[..], [Violation::UiBelowApps { layer: 5, .. }]), "{dep}");
        }
        assert!(!check_graph(&[c("diagramcraft-engine", &[("egui_extras", Normal, false)])]).is_empty());
        assert!(!check_graph(&[c("diagramcraft-mcp", &[("winit", Normal, false)])]).is_empty());
    }

    #[test]
    fn unregistered_crate_is_error() {
        let v = check_graph(&[c("diagramcraft-mystery", &[])]);
        assert!(matches!(&v[..], [Violation::Unregistered { krate }] if krate == "diagramcraft-mystery"));
        assert!(v[0].to_string().contains("register"));
    }

    #[test]
    fn apps_and_xtask_exempt() {
        for app in ["diagramcraft", "diagramcraft-cli", "diagramcraft-web", "xtask"] {
            assert!(check_graph(&[c(app, &[("egui", Normal, false), ("diagramcraft-mcp", Normal, true)])]).is_empty());
        }
    }

    #[test]
    fn metadata_parsing() {
        let meta: Value = serde_json::from_str(
            r#"{"packages":[
                {"name":"diagramcraft-model","dependencies":[
                    {"name":"diagramcraft-geom","kind":null,"path":"/x/crates/geom"},
                    {"name":"serde","kind":null},
                    {"name":"proptest","kind":"dev"}]},
                {"name":"diagramcraft-geom","dependencies":[]}
            ]}"#,
        )
        .unwrap();
        let g = from_metadata(&meta).unwrap();
        assert_eq!(g.len(), 2);
        let doc = g.iter().find(|c| c.name == "diagramcraft-model").unwrap();
        assert!(doc.deps[0].workspace && !doc.deps[1].workspace);
        assert_eq!(doc.deps[2].kind, Dev);
        assert!(check_graph(&g).is_empty());
    }
    #[test]
    fn external_path_ui_and_transitive_lock_are_rejected() {
        // Metadata for the seeded L5 manifest dependency: egui = { path = "../x" }.
        let meta = serde_json::json!({"packages":[{"name":"diagramcraft-engine",
            "dependencies":[{"name":"egui","path":"../x","kind":null}]}]});
        let graph = from_metadata(&meta).unwrap();
        assert!(!graph[0].deps[0].workspace);
        assert!(matches!(&check_graph(&graph)[..], [Violation::UiBelowApps { layer: 5, .. }]));
        let lock =
            "version = 4\n[[package]]\nname = \"innocent-wrapper\"\nversion = \"1.0.0\"\n[[package]]\nname = \"epaint\"\nversion = \"1.0.0\"\n";
        assert!(matches!(&check_lock(lock).unwrap()[..], [Violation::Lockfile { .. }]));
    }
    #[test]
    fn hostile_lockfiles_fail_closed() {
        for lock in ["", "[[package]]", "[[package]]\nname = egui", "[[package]]\nname = \"../egui\"", "[[package]]\nname=\"ok\"\nname=\"egui\""] {
            assert!(check_lock(lock).is_err(), "{lock}");
        }
        assert!(check_lock(&"x".repeat(8 * 1024 * 1024 + 1)).is_err());
        assert!(check_lock("[[package]]\nname='serde'\nversion='1'").unwrap().is_empty());
    }
}
