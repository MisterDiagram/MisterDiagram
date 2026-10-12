//! `cargo xtask parity`: recompute the feature-parity summary in docs/parity-checklist.md from its row
//! table (each Visio feature scored D / P / M, weighted by priority).

use std::path::Path;

const START: &str = "<!-- SUMMARY -->";
const ROWS: &str = "\n## Rows";

/// (area, feature, priority 0 to 2, value 0 / 0.5 / 1, Core).
type Row = (String, String, u8, f64, bool);
pub fn rows(text: &str) -> Result<Vec<Row>, String> {
    let mut out = Vec::new();
    for l in text.lines().map(str::trim).filter(|l| l.starts_with('|')) {
        let cells: Vec<&str> = l.trim_matches('|').split('|').map(str::trim).collect();
        if cells.first().is_some_and(|c| *c == "Area" || c.starts_with("---")) {
            continue;
        }
        let [area, feature, pri, core, status] = cells.as_slice() else {
            return Err(format!("invalid checklist row: {l}"));
        };
        let pri = match *pri {
            "P0" => 0,
            "P1" => 1,
            "P2" => 2,
            _ => return Err(format!("invalid priority: {l}")),
        };
        let core = match *core {
            "Y" => true,
            "N" => false,
            _ => return Err(format!("invalid Core: {l}")),
        };
        let value = match *status {
            "D" => 1.0,
            "P" => 0.5,
            "M" => 0.0,
            _ => return Err(format!("invalid status: {l}")),
        };
        out.push(((*area).into(), (*feature).into(), pri, value, core));
    }
    if out.is_empty() {
        return Err("no checklist rows".into());
    }
    Ok(out)
}
fn weight(p: u8) -> f64 {
    match p {
        0 => 3.0,
        1 => 2.0,
        _ => 1.0,
    }
}

pub fn summary(input: &[Row]) -> String {
    let rows: Vec<Row> = input.iter().filter(|r| r.4).cloned().collect();
    let pct = |rs: &[&Row]| {
        let tot: f64 = rs.iter().map(|r| weight(r.2)).sum();
        if tot == 0.0 { 0.0 } else { 100.0 * rs.iter().map(|r| weight(r.2) * r.3).sum::<f64>() / tot }
    };
    let all: Vec<_> = rows.iter().collect();
    let mut s = format!(
        "**Weighted breadth parity: {:.0}%** over {} Core=Y features.\n\n| Priority | Features | Done | Partial | Missing | Parity |\n|---|---|---|---|---|---|\n",
        pct(&all),
        rows.len()
    );
    for p in 0..3u8 {
        let rr: Vec<_> = rows.iter().filter(|r| r.2 == p).collect();
        let n = |v: f64| rr.iter().filter(|r| r.3 == v).count();
        let unweighted = if rr.is_empty() { 0.0 } else { 100.0 * rr.iter().map(|r| r.3).sum::<f64>() / rr.len() as f64 };
        s += &format!("| P{p} | {} | {} | {} | {} | {unweighted:.0}% |\n", rr.len(), n(1.0), n(0.5), n(0.0));
    }
    s += "\n| Area | Parity |\n|---|---|\n";
    let mut areas: Vec<&str> = Vec::new();
    for r in &rows {
        if !areas.contains(&r.0.as_str()) {
            areas.push(&r.0);
        }
    }
    for a in areas {
        let rr: Vec<_> = rows.iter().filter(|r| r.0 == a).collect();
        s += &format!("| {a} | {:.0}% |\n", pct(&rr));
    }
    let open: Vec<&str> = rows.iter().filter(|r| r.2 == 0 && r.3 < 1.0).map(|r| r.1.as_str()).collect();
    s += &format!("\nOpen P0 items: {}.\n", open.join(", "));
    s += "\nCore=N rows (excluded from milestone scoring):\n\n| Priority | Features | Done | Partial | Missing |\n|---|---|---|---|---|\n";
    for p in 0..3u8 {
        let rs: Vec<_> = input.iter().filter(|r| !r.4 && r.2 == p).collect();
        let n = |v| rs.iter().filter(|r| r.3 == v).count();
        s += &format!("| P{p} | {} | {} | {} | {} |\n", rs.len(), n(1.0), n(0.5), n(0.0));
    }
    s
}

pub fn run(root: &Path) -> Result<(), String> {
    let path = root.join("docs/parity-checklist.md");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let (Some(a), Some(b)) = (text.find(START), text.find(ROWS)) else {
        return Err("docs/parity-checklist.md needs a `<!-- SUMMARY -->` marker before `## Rows`".into());
    };
    if a >= b {
        return Err("summary marker must precede rows".into());
    }
    let rows = rows(text.get(b..).ok_or("invalid rows boundary")?)?;
    let s = summary(&rows);
    let new = format!("{}{START}\n{s}{}", &text[..a], &text[b..]);
    std::fs::write(&path, new).map_err(|e| e.to_string())?;
    print!("{s}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn core_scoring_preserves_weights_and_excludes_shell() {
        let r = rows("| A | x | P0 | Y | D |\n| A | y | P2 | Y | M |\n| B | z | P1 | Y | P |\n| UI | shell | P0 | N | M |\n").unwrap();
        assert_eq!(r.len(), 4);
        let s = summary(&r);
        assert!(s.contains("**Weighted breadth parity: 67%** over 3 Core=Y features."), "{s}");
        assert!(s.contains("| P0 | 1 | 0 | 0 | 1 |"));
        assert!(!s.contains("shell"));
    }
    #[test]
    fn malformed_rows_do_not_silently_change_denominator() {
        for t in ["", "| A | x | P9 | Y | D |", "| A | x | P0 | ? | D |", "| A | x | P0 | Y | ? |", "| A | x | P0 | D |"] {
            assert!(rows(t).is_err());
        }
    }
}
