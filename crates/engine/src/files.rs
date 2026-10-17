//! File commands reuse the bounded model reader, writer and renderers.
use crate::{Result, Selection, Session, operation};
use misterdiagram_vsdx::SaveKind;
use serde_json::{Value, json};
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

pub(crate) fn path<'a>(p: &'a Value, key: &str) -> Result<&'a Path> {
    let value = p
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 32768 && !s.contains('\0'))
        .ok_or_else(|| operation(format!("missing or invalid {key}")))?;
    Ok(Path::new(value))
}
fn extension(path: &Path) -> Result<String> {
    path.extension().and_then(|s| s.to_str()).map(str::to_ascii_lowercase).ok_or_else(|| operation("output needs a supported extension"))
}
/// Documents.Add's initial custom palette, before first-save RGB entries are appended.
/// Witness: generated/basic/0001-probe.vsdx. The low-level skeleton retains its
/// M3 post-save palette contract for existing programmatic callers.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
pub(crate) fn blank_document(metric: bool) -> misterdiagram_model::Document {
    let mut document = misterdiagram_model::skeleton::document(metric);
    document.colors = [(24, "#7F7F7F".into()), (25, "#FFFFFF".into())].into_iter().collect();
    document
}
/// Newly authored RGB literals receive the custom palette entries Visio writes.
/// This runs only after an edit, never merely while opening an existing file.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
pub(crate) fn update_palette(doc: &mut misterdiagram_model::Document) -> Result<()> {
    let mut colors = std::collections::BTreeSet::new();
    let mut cells = 0usize;
    let mut collect = |sheet: &misterdiagram_model::Sheet| -> Result<()> {
        for cell in sheet.cells.iter().chain(sheet.sections.iter().flat_map(|s| s.cells.iter().chain(s.rows.iter().flat_map(|r| &r.cells)))) {
            cells += 1;
            if cells > 2_000_000 {
                return Err(operation("palette cell traversal limit"));
            }
            if let Some(value) =
                cell.value.as_deref().filter(|v| v.len() == 7 && v.starts_with('#') && v.bytes().skip(1).all(|b| b.is_ascii_hexdigit()))
            {
                colors.insert(value.to_ascii_uppercase());
                if colors.len() > 65536 {
                    return Err(operation("palette count limit"));
                }
            }
        }
        Ok(())
    };
    collect(&doc.document_sheet)?;
    for page in &doc.pages {
        collect(&page.sheet)?;
    }
    let mut pending = doc.pages.iter().flat_map(|p| &p.shapes).map(|s| (s, 0)).collect::<Vec<_>>();
    let mut count = 0usize;
    while let Some((shape, depth)) = pending.pop() {
        count += 1;
        if count > 100000 || depth > 128 {
            return Err(operation("palette shape traversal limit"));
        }
        collect(&shape.sheet)?;
        pending.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    for color in colors {
        if !doc.colors.values().any(|v| v.eq_ignore_ascii_case(&color)) {
            let index = doc
                .colors
                .keys()
                .copied()
                .max()
                .unwrap_or(23)
                .checked_add(1)
                .filter(|v| *v < 65536)
                .ok_or_else(|| operation("palette count limit"))?;
            doc.colors.insert(index, color);
        }
    }
    Ok(())
}
/// Create an empty drawing, corresponding to Documents.Add("").
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.add
pub(crate) fn new(s: &mut Session, p: &Value) -> Result<Value> {
    let metric = match p.get("metric") {
        None => false,
        Some(Value::Bool(b)) => *b,
        _ => return Err(operation("metric must be boolean")),
    };
    s.document = Arc::new(blank_document(metric));
    s.selection = Selection::default();
    s.path = None;
    s.undo.clear();
    s.redo.clear();
    Ok(json!({"pages":1,"metric":metric}))
}
/// Open an existing package without changing its imported local caches.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.open
pub(crate) fn open(s: &mut Session, p: &Value) -> Result<Value> {
    let path = path(p, "path")?;
    let document = misterdiagram_vsdx::read_with_open_nodes(path, s.retained_xml_nodes()).map_err(operation)?;
    let pages = document.pages.len();
    s.document = Arc::new(document);
    s.selection = Selection::default();
    s.path = Some(path.to_owned());
    s.undo.clear();
    s.redo.clear();
    Ok(json!({"pages":pages,"macros_removed":s.document.macros_removed}))
}
/// Save as a drawing, stencil or template according to the output extension.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.saveas
pub(crate) fn save(s: &mut Session, p: &Value) -> Result<Value> {
    let path = path(p, "path")?;
    let kind = match extension(path)?.as_str() {
        "vsdx" => SaveKind::Drawing,
        "vssx" => SaveKind::Stencil,
        "vstx" => SaveKind::Template,
        _ => return Err(operation("save extension must be vsdx, vssx or vstx")),
    };
    let theme = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let written = misterdiagram_vsdx::write_as_with_hosts(&s.document, kind, Some(&theme), Some(&misterdiagram_text::Host)).map_err(operation)?;
    atomic_write(path, &written.bytes)?;
    s.path = Some(path.to_owned());
    Ok(json!({"bytes":written.bytes.len(),"warnings":written.warnings,"warning_count":written.warning_count}))
}
/// Export a page through the established renderer. PDF without a page includes
/// every foreground page, matching the existing CLI's PDF export surface.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.export
/// https://learn.microsoft.com/en-us/office/vba/api/visio.document.exportasfixedformat
pub(crate) fn export(s: &mut Session, p: &Value) -> Result<Value> {
    use misterdiagram_render::{pdf, raster, scene, svg};
    let path = path(p, "path")?;
    let ext = extension(path)?;
    if !matches!(ext.as_str(), "png" | "svg" | "pdf") {
        return Err(operation("export extension must be png, svg or pdf"));
    }
    let scale = match p.get("scale") {
        None => 1.,
        Some(v) => v.as_f64().filter(|v| v.is_finite() && *v > 0. && *v <= 100.).ok_or_else(|| operation("scale must be in (0,100]"))?,
    };
    let page = p
        .get("page")
        .map(|v| {
            v.as_u64().and_then(|v| usize::try_from(v).ok()).and_then(|v| v.checked_sub(1)).ok_or_else(|| operation("page is a one-based ordinal"))
        })
        .transpose()?;
    let options = scene::Options { scale, visio_export: true, ..Default::default() };
    let mut budget = scene::SceneBudget::default();
    let scenes = if ext == "pdf" && page.is_none() {
        let mut scenes = Vec::new();
        for (i, page) in s.document.pages.iter().enumerate() {
            if page.attributes.get("Background").is_some_and(|v| v == "1" || v == "true") {
                continue;
            }
            if scenes.len() >= 4096 {
                return Err(operation("export page limit"));
            }
            let scene = scene::page_budgeted(&s.document, i, options, &mut budget).map_err(operation)?;
            scenes.push(scene);
        }
        scenes
    } else {
        vec![scene::page_budgeted(&s.document, page.unwrap_or(s.selection.page), options, &mut budget).map_err(operation)?]
    };
    let first = scenes.first().ok_or_else(|| operation("no foreground page"))?;
    let bytes = match ext.as_str() {
        "png" => {
            let image = raster::render(first, options).map_err(operation)?;
            let mut buffer = std::io::Cursor::new(Vec::new());
            image.write_to(&mut buffer, image::ImageFormat::Png).map_err(operation)?;
            buffer.into_inner()
        }
        "svg" => svg::render(first, options).map_err(operation)?.into_bytes(),
        "pdf" => pdf::render(&scenes, options).map_err(operation)?,
        _ => return Err(operation("unsupported export extension")),
    };
    atomic_write(path, &bytes)?;
    let warnings: Vec<_> = scenes.iter().flat_map(|s| s.warnings.iter()).take(1000).collect();
    Ok(json!({"bytes":bytes.len(),"pages":scenes.len(),"warnings":warnings}))
}
struct Temporary(PathBuf);
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
/// Publish a complete output on the same filesystem as the destination.
/// No destination is truncated before the writer or renderer succeeds.
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    if path.file_name().is_none() {
        return Err(operation("missing output file name"));
    }
    let permissions = match std::fs::metadata(path) {
        Ok(metadata) => Some(metadata.permissions()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(operation(e)),
    };
    for _ in 0..16 {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temp = parent.join(format!(".misterdiagram-{}-{n}.tmp", std::process::id()));
        let mut open = std::fs::OpenOptions::new();
        open.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            open.mode(0o600);
        }
        let mut file = match open.open(&temp) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(operation(e)),
        };
        let temporary = Temporary(temp);
        file.write_all(bytes).map_err(operation)?;
        if let Some(permissions) = &permissions {
            file.set_permissions(permissions.clone()).map_err(operation)?;
        }
        file.sync_all().map_err(operation)?;
        drop(file);
        std::fs::rename(&temporary.0, path).map_err(operation)?;
        return Ok(());
    }
    Err(operation("could not reserve an output temporary file"))
}
