//! Private human-gate contact sheets: each master beside its Visio export.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use std::path::Path;
fn run() -> Result<(), String> {
    let root = Path::new("corpus");
    for source in std::fs::read_to_string(root.join("_state/m2-contact-inputs.txt")).map_err(|e| e.to_string())?.lines() {
        let source = Path::new(source);
        let name = source.file_stem().and_then(|s| s.to_str()).ok_or("invalid stencil name")?;
        let doc = misterdiagram_vsdx::read(source).map_err(|e| e.to_string())?;
        let own_path = root.join("_state/contact-sheets").join(format!("{name}-ours.png"));
        let own = misterdiagram_render::raster::decode(&std::fs::read(own_path).map_err(|e| e.to_string())?)?;
        let height = doc.masters.len().div_ceil(3).checked_mul(200).ok_or("pair height overflow")?;
        if height.checked_mul(1440).is_none_or(|n| n > misterdiagram_render::scene::MAX_PIXELS) {
            return Err("paired contact pixel limit".into());
        }
        let mut output = image::RgbaImage::from_pixel(1440, height as u32, image::Rgba([255, 255, 255, 255]));
        let mut missing = 0usize;
        for (index, master) in doc.masters.iter().enumerate() {
            let x = (index % 3) * 480;
            let y = (index / 3) * 200;
            let sx = (index % 6) * 240;
            let sy = (index / 6) * 200;
            if sx + 240 > own.width() as usize || sy + 200 > own.height() as usize {
                return Err("source contact tile outside image".into());
            }
            let tile = image::imageops::crop_imm(&own, sx as u32, sy as u32, 240, 200).to_image();
            image::imageops::overlay(&mut output, &tile, x as i64, y as i64);
            let reference = root.join("_vm/out/export-masters").join(name).join(format!("{}.png", master.id));
            match std::fs::read(&reference).map_err(|e| e.to_string()).and_then(|bytes| misterdiagram_render::raster::decode(&bytes)) {
                Ok(img) => {
                    let scale = (216. / f64::from(img.width()).max(1.)).min(160. / f64::from(img.height()).max(1.)).min(2.);
                    let w = (f64::from(img.width()) * scale).round().max(1.) as u32;
                    let h = (f64::from(img.height()) * scale).round().max(1.) as u32;
                    let img = image::imageops::resize(&img, w, h, image::imageops::FilterType::Lanczos3);
                    image::imageops::overlay(&mut output, &img, (x + 240 + (240 - w as usize) / 2) as i64, (y + 8) as i64);
                    let label = image::imageops::crop_imm(&tile, 0, 172, 240, 28).to_image();
                    image::imageops::overlay(&mut output, &label, (x + 240) as i64, (y + 172) as i64);
                }
                Err(e) => {
                    missing += 1;
                    eprintln!("missing Visio {}: {e}", reference.display());
                }
            }
        }
        output.save(root.join("_state/contact-sheets").join(format!("{name}-paired.png"))).map_err(|e| e.to_string())?;
        println!("{name}: masters={} missing_Visio={missing} layout=ours-left,Visio-right", doc.masters.len());
    }
    Ok(())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
