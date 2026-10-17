// Review 2a EVALCELL regression, adapted with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use misterdiagram_opc::{PackageOut, RelsOut};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::{
    evaluate::{Evaluator, Scope},
    import,
};

/// A doubling chain User.L0 = 1, User.Lk = User.L(k-1) + User.L(k-1), so User.Lk = 2^k.
fn chain(depth: usize, extra: &str) -> Vec<u8> {
    chain_with_base(depth, "1", extra)
}
fn chain_with_base(depth: usize, base: &str, extra: &str) -> Vec<u8> {
    let mut rows = format!("<Row N='L0'><Cell N='Value' V='1' F='{base}'/></Row>");
    for i in 1..=depth {
        rows.push_str(&format!("<Row N='L{i}'><Cell N='Value' V='0' F='User.L{p}+User.L{p}'/></Row>", p = i - 1));
    }
    rows.push_str(extra);
    let mut package = PackageOut::default();
    let mut rels = RelsOut::default();
    rels.add("http://schemas.microsoft.com/visio/2010/relationships/document", "visio/document.xml", false);
    package.rels("", &rels);
    package.xml_part(
        "visio/document.xml",
        "application/vnd.ms-visio.drawing.main+xml",
        format!(
            "<VisioDocument xmlns='http://schemas.microsoft.com/office/visio/2012/main'><DocumentSheet><Section N='User'>{rows}</Section></DocumentSheet></VisioDocument>"
        )
        .into_bytes(),
    );
    package.finish().unwrap()
}

/// evalcell-function: EVALCELL returns the calculated result of the referenced cell's
/// formula for the given arguments. With no arguments and no ARG() use, that is the cell's
/// own value: User.L15 = 2^15 = 32768 by hand.
#[test]
fn review_2a_evalcell_of_argument_free_chain_equals_direct_value() {
    let bytes = chain(15, "<Row N='T'><Cell N='Value' V='0' F='EVALCELL(User.L15)'/></Row>");
    let doc = import(&bytes).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "User.L15"), Value::Number(32768.0));
    assert_eq!(e.evaluate(Scope::Document, "User.T"), Value::Number(32768.0));
}

#[test]
fn argument_dependent_chain_uses_each_frame_and_never_an_outside_cache() {
    let bytes = chain(
        4,
        "<Row N='Arg'><Cell N='Value' V='0' F='ARG(&quot;A&quot;,1)'/></Row><Row N='Mid'><Cell N='Value' V='0' F='User.Arg+User.L4'/></Row><Row N='Outer'><Cell N='Value' V='0' F='User.Mid+User.Mid'/></Row>",
    );
    let doc = import(&bytes).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "User.Outer"), Value::Number(34.));
    for arg in [7., 3., 11.] {
        assert_eq!(e.eval_cell(Scope::Document, "User.Outer", &[("A".into(), Value::Number(arg))]), Value::Number((arg + 16.) * 2.));
    }
    assert_eq!(e.evaluate(Scope::Document, "User.Outer"), Value::Number(34.));
}

#[test]
fn cold_argument_free_chain_is_cached_inside_the_frame() {
    let bytes = chain(15, "<Row N='T'><Cell N='Value' V='0' F='EVALCELL(User.L15)'/></Row>");
    let doc = import(&bytes).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "User.T"), Value::Number(32768.));
    assert_eq!(e.evaluate(Scope::Document, "User.L15"), Value::Number(32768.));
}

#[test]
fn clamped_argument_free_cycle_is_the_same_in_cold_and_warm_frames() {
    let bytes = chain(0, "<Row N='C'><Cell N='Value' V='0' F='MAX(.75,MIN(User.C/2,.25))'/></Row>");
    let doc = import(&bytes).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.eval_cell(Scope::Document, "User.C", &[]), Value::Number(0.75));
    assert_eq!(e.evaluate(Scope::Document, "User.C"), Value::Number(0.75));
    assert_eq!(e.eval_cell(Scope::Document, "User.C", &[]), Value::Number(0.75));
}

#[test]
fn frame_depth_refusal_does_not_poison_later_argument_free_values() {
    let mut rows = String::from(
        "<Row N='Helper'><Cell N='Value' V='0' F='IFERROR(EVALCELL(User.L0),99)'/></Row><Row N='N0'><Cell N='Value' V='0' F='User.Helper'/></Row>",
    );
    for i in 1..=63 {
        rows.push_str(&format!("<Row N='N{i}'><Cell N='Value' V='0' F='EVALCELL(User.N{})'/></Row>", i - 1));
    }
    let doc = import(&chain(0, &rows)).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert!(matches!(e.eval_cell(Scope::Document, "User.N63", &[]), Value::Error(_)));
    assert_eq!(e.evaluate(Scope::Document, "User.Helper"), Value::Number(1.));
}

#[test]
fn outside_argument_and_cycle_chains_keep_their_cache() {
    for (base, extra) in [("ARG(&quot;A&quot;,1)", ""), ("IFERROR(User.C,1)", "<Row N='C'><Cell N='Value' V='0' F='User.C+1'/></Row>")] {
        let doc = import(&chain_with_base(20, base, extra)).unwrap();
        let mut e = Evaluator::new(&doc).unwrap();
        for _ in 0..2 {
            assert_eq!(e.evaluate(Scope::Document, "User.L20"), Value::Number(1048576.), "outside chain base {base}");
        }
    }
}

#[test]
fn outside_contextual_cache_hit_keeps_a_cold_parent_frame_dependent() {
    let doc = import(&chain_with_base(4, "ARG(&quot;A&quot;,1)", "")).unwrap();
    let mut e = Evaluator::new(&doc).unwrap();
    assert_eq!(e.evaluate(Scope::Document, "User.L0"), Value::Number(1.));
    assert_eq!(e.evaluate(Scope::Document, "User.L4"), Value::Number(16.));
    assert_eq!(e.eval_cell(Scope::Document, "User.L4", &[("A".into(), Value::Number(7.))]), Value::Number(112.));
    assert_eq!(e.evaluate(Scope::Document, "User.L4"), Value::Number(16.));
}
