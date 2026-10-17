//! Read-only pixel profiles for private raster witnesses.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for file in std::env::args().skip(1) {
        let img = misterdiagram_render::raster::decode(&std::fs::read(&file)?)?;
        let (mut x0, mut y0, mut x1, mut y1) = (img.width(), img.height(), 0, 0);
        for (x, y, p) in img.enumerate_pixels() {
            if p.0.iter().take(3).any(|c| *c < 250) {
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
        }
        println!("{file} {:?} ink=({x0},{y0})-({x1},{y1})", img.dimensions());
        for y in [0, 1, 2, 3, 4, 10, 30, img.height() / 2, img.height().saturating_sub(2)] {
            if y >= img.height() {
                continue;
            }
            let p: Vec<_> = (0..20.min(img.width())).map(|x| img.get_pixel(x, y).0).collect();
            println!("y={y}: {p:?}");
        }
    }
    Ok(())
}
