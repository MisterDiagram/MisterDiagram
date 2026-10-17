//! Header dimensions must fail before decoding an oversized pixel allocation.
fn oversized_header() -> Vec<u8> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::RgbaImage::new(1, 1).write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    let mut bytes = bytes.into_inner();
    bytes[16..20].copy_from_slice(&10_000u32.to_be_bytes());
    bytes[20..24].copy_from_slice(&10_000u32.to_be_bytes());
    let mut crc = !0u32;
    for byte in &bytes[12..29] {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    bytes[29..33].copy_from_slice(&(!crc).to_be_bytes());
    bytes
}
#[test]
fn oversized_image_header_precedes_pixel_decode() {
    assert_eq!(misterdiagram_render::raster::decode(&oversized_header()).unwrap_err(), "image pixel limit (header)");
}
#[test]
#[cfg(target_os = "macos")]
fn oversized_image_peak_memory_is_bounded() {
    let output = std::process::Command::new("/usr/bin/time")
        .arg("-l")
        .arg(std::env::current_exe().unwrap())
        .args(["--exact", "oversized_image_header_precedes_pixel_decode", "--nocapture"])
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "{stderr}");
    let rss: u64 = stderr.lines().find(|l| l.contains("maximum resident set size")).unwrap().split_whitespace().next().unwrap().parse().unwrap();
    eprintln!("10000x10000 image header peak RSS: {rss} bytes (limit 100 MiB)");
    assert!(rss < 100 * 1024 * 1024, "{stderr}");
}
