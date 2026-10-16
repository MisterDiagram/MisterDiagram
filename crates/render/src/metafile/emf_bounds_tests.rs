// Review 3b EMF regressions, scaled with Charles Hood's permission.
// MIT OR Apache-2.0. Source recorded in ATTRIBUTION.md.
use super::*;

fn rec(kind: u32, body: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&kind.to_le_bytes());
    bytes.extend_from_slice(&(8 + 4 * body.len() as u32).to_le_bytes());
    for value in body {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}
fn player() -> Player {
    let mut player = Player::new(Kind::Emf, Affine::IDENTITY, 25.4 / 96., Rect::new(0., 0., 100., 100.));
    player.operation_limit = 32;
    player
}
fn text(n: u32) -> Vec<u8> {
    let mut bytes = rec(84, &[0, 0, 100, 100, 1, 0, 0, 10, 10, n, 76, 0, 0, 0, 100, 100, 0]);
    bytes.extend(std::iter::repeat_n([b'A', 0], n as usize).flatten());
    let len = bytes.len() as u32;
    bytes[4..8].copy_from_slice(&len.to_le_bytes());
    bytes
}
fn dotted(player: &mut Player, points: usize, length: u32) {
    record(player, 95, &rec(95, &[1, 0, 0, 0, 0, 2, 0, 0, 0xff, 0, 0])).unwrap();
    record(player, 37, &rec(37, &[1])).unwrap();
    let mut body = vec![0, 0, 100, 100, points as u32];
    for i in 0..points {
        body.extend([if i % 2 == 0 { 0 } else { length }, i as u32 * 10]);
    }
    record(player, 4, &rec(4, &body)).unwrap();
}
#[test]
fn emf_dotted_long_polyline_memory() {
    let mut p = player();
    dotted(&mut p, 40, 100);
    assert!(p.failure.as_deref().is_some_and(|s| s.contains("metafile operation limit")), "dashes must be refused before outline construction");
    assert!(p.out.is_empty(), "dash outline must not be retained before refusal");
}
#[test]
fn emf_long_text_record_memory() {
    let mut p = player();
    record(&mut p, 84, &text(64)).unwrap();
    assert!(p.failure.as_deref().is_some_and(|s| s.contains("metafile operation limit")), "text must be refused before shaping");
    assert!(p.out.is_empty(), "text must be refused before glyph outlines are retained");
}
#[test]
fn small_text_and_dashes_remain_drawable() {
    let mut p = player();
    record(&mut p, 84, &text(8)).unwrap();
    assert!(p.failure.is_none());
    assert_eq!(p.out.len(), 8);
    let mut p = player();
    dotted(&mut p, 2, 10);
    assert!(p.failure.is_none());
    assert_eq!(p.out.len(), 1);
}
