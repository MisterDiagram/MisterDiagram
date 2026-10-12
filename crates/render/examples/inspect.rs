//! Host diagnostics for export bounds and pixel profiles, without changing oracle files.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use diagramcraft_render::{raster, scene};
use kurbo::Shape as _;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut files = vec![];
    if args.is_empty() {
        for group in std::fs::read_dir("corpus/generated")? {
            let group = group?.path();
            if !group.is_dir() {
                continue;
            }
            for file in std::fs::read_dir(group)? {
                let p = file?.path();
                if p.extension().is_some_and(|x| x == "vsdx") {
                    files.push(p);
                }
            }
        }
    } else {
        files.extend(args.iter().map(std::path::PathBuf::from));
    }
    files.sort();
    for file in files {
        let doc = diagramcraft_vsdx::read(&file)?;
        let s = scene::page(&doc, 0, scene::Options::default())?;
        let expected = raster::decode(&std::fs::read(file.with_extension("png"))?)?;
        println!(
            "{}\t{:?}\t{:?}\t{:?}\t{:.9}\t{:.9}",
            file.display(),
            expected.dimensions(),
            s.page_size,
            s.bounds,
            s.bounds.width() * 96.,
            s.bounds.height() * 96.
        );
        if !args.is_empty() {
            println!("  warnings {:?}", s.warnings);
            for op in &s.ops {
                match op {
                    scene::Op::Bounds(b) => println!("  logical {b:?}"),
                    scene::Op::Path { path, .. } => println!("  path {:?}", path.bounding_box()),
                    scene::Op::Text { layout, transform } => {
                        println!("  text transform {transform:?}");
                        for line in &layout.lines {
                            println!(
                                "    line para={} start={} end={} extent={:?} top={} baseline={} bottom={}",
                                line.para, line.start, line.end, line.extent, line.top, line.baseline, line.bottom
                            );
                        }
                        for r in &layout.runs {
                            println!("    text {:?} font {} size {} glyphs {:?}", r.cells, r.face.family, r.size, r.glyphs);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
