use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::path::Path;

fn compare(path: &Path) {
    let original = diagramcraft_vsdx::read(path).unwrap();
    let mut actual = original.clone();
    let mut host = diagramcraft_route::GlueHost::prepare(&original, &mut actual).unwrap();
    diagramcraft_route::reroute(&original, &mut actual, &mut host, Some((0, None))).unwrap();
    let mut oracle = Evaluator::new(&original).unwrap();
    let mut evaluated = Evaluator::new(&actual).unwrap().with_glue_host(&mut host);
    let page = &original.pages[0];
    let ids = page.connects.iter().map(|c| c.from_sheet).collect::<std::collections::BTreeSet<_>>();
    for id in ids {
        let scope = Scope::PageShape { page: page.id, shape: id };
        let kinds = |e: &Evaluator<'_>| {
            e.resolved_named_cells(scope)
                .into_iter()
                .filter(|n| n.section.is_some_and(|s| s.name == "Geometry") && n.cell.name == "X")
                .filter_map(|n| {
                    n.row.filter(|r| !r.deleted).map(|r| ((n.section.and_then(|s| s.index).unwrap_or(0), r.index.unwrap_or(0)), r.row_type.clone()))
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        assert_eq!(kinds(&oracle), kinds(&evaluated), "{} shape{id} row kinds", path.display());
        let names = oracle
            .resolved_named_cells(scope)
            .into_iter()
            .chain(evaluated.resolved_named_cells(scope))
            .map(|c| c.name)
            .filter(|n| {
                n.starts_with("Geometry")
                    || matches!(
                        n.as_str(),
                        "BeginX"
                            | "BeginY"
                            | "EndX"
                            | "EndY"
                            | "PinX"
                            | "PinY"
                            | "Width"
                            | "Height"
                            | "LocPinX"
                            | "LocPinY"
                            | "Angle"
                            | "FlipX"
                            | "FlipY"
                    )
            })
            .collect::<std::collections::BTreeSet<_>>();
        for name in names {
            let a = evaluated.evaluate(scope, &name);
            let b = oracle.evaluate(scope, &name);
            let eq = equivalent(&a, &b);
            assert!(eq, "{} shape{id} {name}: actual={a:?} Visio={b:?}", path.display());
        }
    }
}
#[test]
fn observed_orthogonal_geometry_and_frames() {
    let root = std::env::var_os("DIAGRAMCRAFT_CORPUS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus"));
    if !root.exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    for name in [
        "offset-right-above",
        "offset-right-below",
        "offset-above",
        "offset-below",
        "offset-left",
        "obstacle-below-centre",
        "obstacle-stacked",
        "obstacle-touching",
        "obstacle-permeable",
        "obstacle-objtype-zero",
        "style-1",
        "style-2",
        "style-16",
        "style-override",
        "preference-line-to-node",
        "preference-block-size",
        "preference-avoid-page-breaks",
    ] {
        compare(&root.join(format!("generated/route/{name}.vsdx")));
    }
    for name in ["obstacle", "dynamic-glue", "move-before", "move-after", "static-glue"] {
        compare(&root.join(format!("generated/connector/{name}.vsdx")));
    }
}

fn equivalent(a: &diagramcraft_sheet::Value, b: &diagramcraft_sheet::Value) -> bool {
    use diagramcraft_sheet::{Expr, Value};
    if let (Value::Text(a), Value::Text(b)) = (a, b) {
        if a.starts_with("POLYLINE(") && b.starts_with("POLYLINE(") {
            let scalar = |v: &Expr| match v {
                Expr::Number(n, None) => Some(*n),
                Expr::Unary(diagramcraft_sheet::parser::UnaryOp::Minus, e) => {
                    if let Expr::Number(n, None) = &**e {
                        Some(-n)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let (Ok(Expr::Call { name: an, args: aa }), Ok(Expr::Call { name: bn, args: ba })) =
                (diagramcraft_sheet::parse(a), diagramcraft_sheet::parse(b))
            {
                return an == bn
                    && aa.len() == ba.len()
                    && aa.iter().zip(&ba).all(|(a, b)| scalar(a).zip(scalar(b)).is_some_and(|(a, b)| (a - b).abs() <= 1e-6));
            }
        }
        return a == b;
    }
    match (a.numeric(), b.numeric()) {
        (Ok(a), Ok(b)) => (a - b).abs() <= 1e-6,
        _ => a == b,
    }
}

#[test]
fn observed_line_jump_rows_and_signed_arcs() {
    let root = std::env::var_os("DIAGRAMCRAFT_CORPUS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus"));
    if !root.exists() {
        eprintln!("skip: private corpus absent");
        return;
    }
    for name in ["jump-code-2", "jump-factor-x", "jump-gap", "jump-square", "jump-two-sides"] {
        compare(&root.join(format!("generated/route/{name}.vsdx")));
    }
    compare(&root.join("generated/connector/line-jumps.vsdx"));
}
