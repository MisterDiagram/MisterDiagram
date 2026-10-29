//! Deterministic SVG test fonts: approved bundled bytes and the product's aliases only.
pub fn options() -> resvg::usvg::Options<'static> {
    use resvg::usvg::{FontStyle, fontdb};
    let mut options = resvg::usvg::Options::default();
    for bytes in misterdiagram_fonts::bundled() {
        options.fontdb_mut().load_font_data(bytes.to_vec());
    }
    options.fontdb_mut().set_serif_family("Carlito");
    options.fontdb_mut().set_sans_serif_family("Carlito");
    options.fontdb_mut().set_monospace_family("Cousine");
    options.font_resolver.select_font = Box::new(|font, db| {
        let names: Vec<String> = font.families().iter().map(ToString::to_string).map(|s| s.trim_matches(['\'', '"']).to_owned()).collect();
        let mut families = Vec::new();
        for name in &names {
            families.push(fontdb::Family::Name(name));
            for family in misterdiagram_fonts::substitutes(name) {
                families.push(fontdb::Family::Name(family));
            }
        }
        families.push(fontdb::Family::Name(misterdiagram_fonts::FALLBACK_FAMILY));
        families.push(fontdb::Family::Name("Carlito"));
        db.query(&fontdb::Query {
            families: &families,
            weight: fontdb::Weight(font.weight()),
            stretch: fontdb::Stretch::Normal,
            style: match font.style() {
                FontStyle::Normal => fontdb::Style::Normal,
                FontStyle::Italic => fontdb::Style::Italic,
                FontStyle::Oblique => fontdb::Style::Oblique,
            },
        })
    });
    options
}
