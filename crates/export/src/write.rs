//! Presentation packaging retargeted from storytold/deckcraft
//! crates/pptx/src/write.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use crate::model::Page;
use misterdiagram_color::Rgba;
use misterdiagram_opc::{
    NS_A, NS_P, NS_R, PackageOut, RelsOut,
    xml::{A, W},
};
use misterdiagram_render::scene::{EvaluatedPage, Options, Scene};

mod content;
mod dml;
mod media;
mod shapes;
mod theme;

const CT_PRES: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml";
const CT_SLIDE: &str = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
const CT_LAYOUT: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
const CT_MASTER: &str = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
const CT_THEME: &str = "application/vnd.openxmlformats-officedocument.theme+xml";
const CT_CORE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const CT_APP: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
// Fixed theme/master/layout/properties plus per-slide relationship XML, OPC
// metadata, presentation IDs, paths and warnings. The per-slide allowance is
// deliberately larger than the generated fixed-schema metadata.
const FIXED_STORAGE: usize = 64 * 1024;
const SLIDE_METADATA: usize = 16 * 1024;
// String/Vec spare capacity and ZIP output capacity can each approach twice the
// used bytes. Reserve both while PackageOut retains the source buffers.
const XML_STORAGE_FACTOR: usize = 4;

pub struct Export {
    pub bytes: Vec<u8>,
    pub pages: usize,
    pub warnings: Vec<String>,
}

/// Package already evaluated foreground scenes in their input order.
/// Backgrounds, visibility and cached fallbacks are resolved by the renderer.
/// ECMA-376 Part 1, 19.2.1.26 (presentation) and 19.3.1.38 (sld).
pub fn render(scenes: Vec<EvaluatedPage>, colors: &[Rgba; 12], options: Options) -> Result<Export, String> {
    let mut pixel_budget = misterdiagram_render::scene::SceneBudget::default();
    let pages = scenes
        .into_iter()
        .map(|source| {
            let view = canvas(&source.scene, options)?;
            let pixels = raster_options(&source.scene, options)?;
            let mut page = Page::with_options(source, pixels, &mut pixel_budget)?;
            page.presentation_canvas = Some(view);
            Ok(page)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut output = render_limited(&pages, colors, options, 256 * 1024 * 1024)?;
    for page in pages {
        for warning in page.canvas.warnings {
            if output.warnings.len() < 1000 {
                output.warnings.push(warning);
            }
        }
    }
    Ok(output)
}
fn render_limited(scenes: &[Page], colors: &[Rgba; 12], options: Options, limit: usize) -> Result<Export, String> {
    if scenes.is_empty() || scenes.len() > 4096 {
        return Err("PPTX requires 1 to 4096 foreground pages".into());
    }
    let first = scenes.first().ok_or("no foreground page")?;
    let (original_width, original_height, _) = output_canvas(first, options)?;
    let (width, height) = slide_size(original_width, original_height)?;
    let mut pkg = PackageOut::default();
    let mut media = media::Media::default();
    let mut warnings = vec!["PPTX: connector presets and glue await connector export".into()];
    if original_width.max(original_height) / original_width.min(original_height) > 56. {
        warnings.push("PPTX: page aspect ratio exceeds 56:1; short canvas side padded to one inch".into());
    }
    let mut rels = RelsOut::default();
    let master = rels.add("slideMaster", "slideMasters/slideMaster1.xml", false);
    let mut slides = Vec::new();
    let mut retained = FIXED_STORAGE;
    for (index, scene) in scenes.iter().enumerate() {
        retained = retained.checked_add(SLIDE_METADATA).ok_or("PPTX package limit")?;
        let remaining = limit.checked_sub(retained).ok_or("PPTX package limit")? / XML_STORAGE_FACTOR;
        let remaining = remaining.min(32 * 1024 * 1024);
        let (w, h, view) = output_canvas(scene, options)?;
        for substitution in &scene.substitutions {
            content::warn(&mut warnings, &format!("PPTX page {}: {}", index + 1, substitution.name()));
        }
        let scale = (width / w).min(height / h);
        if (w - width).abs() > 1e-9 || (h - height).abs() > 1e-9 {
            warnings.push(format!("PPTX page {}: scaled by {scale:.9} to fit the presentation canvas (DrawingML sides 1 to 56 inches)", index + 1));
        }
        let view = kurbo::Affine::translate(((width - w * scale) / 2., (height - h * scale) / 2.)) * kurbo::Affine::scale(scale) * view;
        let mut w = W::new();
        root_open(&mut w, "p:sld", A::new());
        tree_open(&mut w);
        let mut slide_rels = RelsOut::default();
        slide_rels.add("slideLayout", "../slideLayouts/slideLayout1.xml", false);
        let mut media_remaining = limit.checked_sub(retained).ok_or("PPTX package limit")?;
        let media_before = media_remaining;
        shapes::write(
            &mut w,
            scene,
            view,
            &mut warnings,
            remaining,
            &mut shapes::Pictures { media: &mut media, package: &mut pkg, rels: &mut slide_rels, remaining: &mut media_remaining },
        )?;
        retained = retained.checked_add(media_before - media_remaining).ok_or("PPTX package limit")?;
        tree_close(&mut w);
        color_override(&mut w);
        w.close("p:sld");
        let bytes = w.finish();
        let charge = bytes.len().checked_mul(XML_STORAGE_FACTOR).ok_or("PPTX package limit")?;
        retained = retained.checked_add(charge).ok_or("PPTX package limit")?;
        if retained > limit {
            return Err("PPTX package limit".into());
        }
        let part = format!("ppt/slides/slide{}.xml", index + 1);
        pkg.xml_part(&part, CT_SLIDE, bytes);
        pkg.rels(&part, &slide_rels);
        slides.push(rels.add("slide", &format!("slides/slide{}.xml", index + 1), false));
    }
    let mut w = W::new();
    root_open(&mut w, "p:presentation", A::new());
    w.open0("p:sldMasterIdLst");
    w.empty("p:sldMasterId", A::new().a("id", 2147483648u32).a("r:id", master));
    w.close("p:sldMasterIdLst");
    w.open0("p:sldIdLst");
    for (i, id) in slides.iter().enumerate() {
        w.empty("p:sldId", A::new().a("id", i + 256).a("r:id", id));
    }
    w.close("p:sldIdLst");
    w.empty("p:sldSz", A::new().a("cx", emu(width)?).a("cy", emu(height)?));
    w.empty("p:notesSz", A::new().a("cx", emu(width)?).a("cy", emu(height)?));
    w.close("p:presentation");
    pkg.xml_part("ppt/presentation.xml", CT_PRES, w.finish());
    pkg.rels("ppt/presentation.xml", &rels);
    templates(&mut pkg);
    pkg.xml_part("ppt/theme/theme1.xml", CT_THEME, theme::write(colors));
    properties(&mut pkg, scenes.len());
    let mut root = RelsOut::default();
    root.add("officeDocument", "ppt/presentation.xml", false);
    root.add(misterdiagram_opc::RT_CORE, "docProps/core.xml", false);
    root.add("extended-properties", "docProps/app.xml", false);
    pkg.rels("", &root);
    warnings.truncate(1000);
    Ok(Export { pages: scenes.len(), bytes: pkg.finish().map_err(|e| e.to_string())?, warnings })
}

fn output_canvas(page: &Page, options: Options) -> Result<(f64, f64, kurbo::Affine), String> {
    page.presentation_canvas.map(Ok).unwrap_or_else(|| canvas(&page.canvas, options))
}

fn canvas(scene: &Scene, options: Options) -> Result<(f64, f64, kurbo::Affine), String> {
    if options.fit {
        let (w, h, view) = scene.fit_canvas(options)?;
        let dpi = 96. * options.scale;
        Ok((w / dpi, h / dpi, kurbo::Affine::scale(1. / dpi) * view))
    } else {
        scene.canvas(raster_options(scene, options)?)?;
        let (w, h) = scene.page_size;
        if !w.is_finite() || !h.is_finite() || w <= 0. || h <= 0. || w > 1e6 || h > 1e6 {
            return Err("PPTX invalid page size".into());
        }
        Ok((w, h, page_flip(h)))
    }
}

/// A1 scales oversized canvases before any foreign-pixel allocation. Validation
/// still uses the renderer's scene guards, at the resulting presentation density.
fn raster_options(scene: &Scene, options: Options) -> Result<Options, String> {
    if !options.scale.is_finite() || options.scale <= 0. || options.scale > 100. {
        return Err("render scale outside (0,100]".into());
    }
    if options.fit {
        return Ok(options);
    }
    match scene.canvas(options) {
        Ok(_) => return Ok(options),
        Err(error) if error == "render pixel limit" => {}
        Err(error) => return Err(error),
    }
    let longest = scene.bounds.width().max(scene.bounds.height());
    let factor = if longest > 56. { 56. / longest } else { 1. };
    Ok(Options { scale: options.scale * factor, ..options })
}

/// ST_SlideSizeCoordinate, ECMA-376 Part 1 section 19.7.17. Keep legal
/// pages exact; otherwise uniformly fit the nearest legal canvas. Ratios above
/// 56:1 need empty padding on the shorter side because no uniform scale can put
/// both sides in range. The content is never stretched or silently cropped.
fn slide_size(width: f64, height: f64) -> Result<(f64, f64), String> {
    if !width.is_finite() || !height.is_finite() || width <= 0. || height <= 0. {
        return Err("PPTX invalid page size".into());
    }
    let minimum = 1. / width.min(height);
    let maximum = 56. / width.max(height);
    let scale = if minimum <= maximum { 1f64.clamp(minimum, maximum) } else { maximum };
    Ok(((width * scale).clamp(1., 56.), (height * scale).clamp(1., 56.)))
}

/// The only page-space Y flip: Visio inches to top-left DrawingML inches.
fn page_flip(height: f64) -> kurbo::Affine {
    kurbo::Affine::new([1., 0., 0., -1., 0., height])
}

fn emu(inches: f64) -> Result<i64, String> {
    let n = inches * 914400.;
    if !n.is_finite() || n.abs() > 2.0e9 {
        return Err("PPTX coordinate limit".into());
    }
    Ok(n.round() as i64)
}

fn root_open(w: &mut W, tag: &str, mut attrs: A) {
    attrs = attrs.a("xmlns:a", NS_A).a("xmlns:r", NS_R).a("xmlns:p", NS_P);
    w.open(tag, attrs);
}

fn tree_open(w: &mut W) {
    w.open0("p:cSld");
    w.open0("p:spTree");
    w.open0("p:nvGrpSpPr");
    w.empty("p:cNvPr", A::new().a("id", 1).a("name", ""));
    w.empty0("p:cNvGrpSpPr");
    w.empty0("p:nvPr");
    w.close("p:nvGrpSpPr");
    w.open0("p:grpSpPr");
    w.open0("a:xfrm");
    for tag in ["a:off", "a:chOff"] {
        // Children follow ext below in the required schema order.
        if tag == "a:off" {
            w.empty(tag, A::new().a("x", 0).a("y", 0));
            w.empty("a:ext", A::new().a("cx", 0).a("cy", 0));
        } else {
            w.empty(tag, A::new().a("x", 0).a("y", 0));
            w.empty("a:chExt", A::new().a("cx", 0).a("cy", 0));
        }
    }
    w.close("a:xfrm");
    w.close("p:grpSpPr");
}
fn tree_close(w: &mut W) {
    w.close("p:spTree");
    w.close("p:cSld");
}
fn color_override(w: &mut W) {
    w.open0("p:clrMapOvr");
    w.empty0("a:masterClrMapping");
    w.close("p:clrMapOvr");
}
fn templates(pkg: &mut PackageOut) {
    let mut w = W::new();
    root_open(&mut w, "p:sldLayout", A::new().a("type", "blank").a("preserve", 1));
    tree_open(&mut w);
    tree_close(&mut w);
    color_override(&mut w);
    w.close("p:sldLayout");
    pkg.xml_part("ppt/slideLayouts/slideLayout1.xml", CT_LAYOUT, w.finish());
    let mut rels = RelsOut::default();
    rels.add("slideMaster", "../slideMasters/slideMaster1.xml", false);
    pkg.rels("ppt/slideLayouts/slideLayout1.xml", &rels);
    let mut rels = RelsOut::default();
    let layout = rels.add("slideLayout", "../slideLayouts/slideLayout1.xml", false);
    rels.add("theme", "../theme/theme1.xml", false);
    pkg.rels("ppt/slideMasters/slideMaster1.xml", &rels);
    let mut w = W::new();
    root_open(&mut w, "p:sldMaster", A::new());
    tree_open(&mut w);
    tree_close(&mut w);
    let mut map = A::new().a("bg1", "lt1").a("tx1", "dk1").a("bg2", "lt2").a("tx2", "dk2");
    for slot in ["accent1", "accent2", "accent3", "accent4", "accent5", "accent6", "hlink", "folHlink"] {
        map = map.a(slot, slot);
    }
    w.empty("p:clrMap", map);
    w.open0("p:sldLayoutIdLst");
    w.empty("p:sldLayoutId", A::new().a("id", 2147483649u32).a("r:id", layout));
    w.close("p:sldLayoutIdLst");
    w.close("p:sldMaster");
    pkg.xml_part("ppt/slideMasters/slideMaster1.xml", CT_MASTER, w.finish());
}
fn properties(pkg: &mut PackageOut, slides: usize) {
    let app = format!("MisterDiagram {}", env!("CARGO_PKG_VERSION"));
    let mut w = W::new();
    w.open(
        "cp:coreProperties",
        A::new()
            .a("xmlns:cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties")
            .a("xmlns:dc", "http://purl.org/dc/elements/1.1/"),
    );
    w.elt("dc:creator", &app);
    w.elt("cp:lastModifiedBy", &app);
    w.close("cp:coreProperties");
    pkg.xml_part("docProps/core.xml", CT_CORE, w.finish());
    let mut w = W::new();
    w.open("Properties", A::new().a("xmlns", "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"));
    w.elt("Application", &app);
    w.elt("Slides", &slides.to_string());
    w.close("Properties");
    pkg.xml_part("docProps/app.xml", CT_APP, w.finish());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aggregate_xml_budget_is_shared_between_slides() {
        let doc = misterdiagram_model::skeleton::document(false);
        let opts = Options::default();
        let scene = Page::from_evaluated(misterdiagram_render::scene::evaluated::page(&doc, 0, opts, &mut Default::default()).unwrap()).unwrap();
        let colors = [Rgba::BLACK; 12];
        let one = render_limited(std::slice::from_ref(&scene), &colors, opts, 256 * 1024).unwrap();
        let package = misterdiagram_opc::Package::open(&one.bytes).unwrap();
        let slide = package.get("ppt/slides/slide1.xml").unwrap().len();
        let limit = FIXED_STORAGE + SLIDE_METADATA + XML_STORAGE_FACTOR * slide;
        assert!(render_limited(std::slice::from_ref(&scene), &colors, opts, limit).is_ok());
        let second = Page::from_evaluated(misterdiagram_render::scene::evaluated::page(&doc, 0, opts, &mut Default::default()).unwrap()).unwrap();
        assert!(render_limited(&[scene, second], &colors, opts, limit).is_err());
    }
    #[test]
    fn legal_slide_bounds_keep_uniform_scaling() {
        assert_eq!(slide_size(8.5, 11.).unwrap(), (8.5, 11.));
        let (w, h) = slide_size(0.085, 0.11).unwrap();
        assert_eq!(w, 1.);
        assert!((h - 0.11 / 0.085).abs() < 1e-12);
        assert_eq!(slide_size(112., 28.).unwrap(), (56., 14.));
        assert_eq!(slide_size(1000., 1.).unwrap(), (56., 1.));
        for (w, h) in [(0., 1.), (f64::NAN, 1.), (1., f64::INFINITY)] {
            assert!(slide_size(w, h).is_err());
        }
    }
    #[test]
    fn page_corners_flip_once_and_convert_to_emu() {
        let flip = page_flip(7.);
        for (from, to) in [((0., 0.), (0., 7.)), ((10., 0.), (10., 7.)), ((0., 7.), (0., 0.)), ((10., 7.), (10., 0.))] {
            let p = flip * kurbo::Point::from(from);
            assert_eq!(p, kurbo::Point::from(to));
            assert_eq!(emu(p.x).unwrap(), (to.0 * 914400.) as i64);
            assert_eq!(emu(p.y).unwrap(), (to.1 * 914400.) as i64);
        }
        for bad in [f64::NAN, f64::INFINITY, -f64::INFINITY, 1e100] {
            assert!(emu(bad).is_err());
        }
    }
}
