use misterdiagram_model::{Cell, Document, Master, Part, Shape, skeleton};
use misterdiagram_sheet::Value;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

fn set(shape: &mut Shape, name: &str, value: &str) {
    shape.sheet.cells.retain(|cell| cell.name != name);
    shape.sheet.cells.push(Cell { name: name.into(), value: Some(value.into()), ..Default::default() });
}
fn document() -> Document {
    let mut document = skeleton::document(false);
    // Contributor-original numerical theme with visibly distinct shape and connector styles.
    let xml = br#"<theme><clrScheme><dk1><srgbClr val="14283C"/></dk1><lt1><srgbClr val="F0E0D0"/></lt1><schemeID schemeEnum="123"/></clrScheme><fontScheme><minorFont><latin typeface="Contributor Font"/></minorFont><schemeID schemeEnum="123"/></fontScheme><fontStyles><fontProps><color><srgbClr val="E1D2C3"/></color></fontProps></fontStyles><connectorFontStyles><fontProps><color><schemeClr val="phClr"/></color></fontProps></connectorFontStyles><fillStyles><fillProps pattern="7"/></fillStyles><fmtScheme><fillStyleLst><solidFill><schemeClr val="phClr"/></solidFill></fillStyleLst><lnStyleLst><ln w="28575"><solidFill><srgbClr val="314253"/></solidFill></ln></lnStyleLst></fmtScheme><fmtSchemeEx><schemeID schemeEnum="123"/></fmtSchemeEx><fmtConnectorScheme><fillStyleLst><solidFill><schemeClr val="lt1"/></solidFill></fillStyleLst><lnStyleLst><ln w="19050"><solidFill><schemeClr val="phClr"/></solidFill></ln></lnStyleLst></fmtConnectorScheme><fmtConnectorSchemeEx><schemeID schemeEnum="123"/></fmtConnectorSchemeEx></theme>"#;
    document.parts.insert("visio/theme/theme1.xml".into(), Part { bytes: xml.to_vec().into(), ..Default::default() });
    let mut shape = skeleton::rectangle(1, 1., 1., 2., 1., "Label").unwrap();
    for (name, value) in [
        ("ColorSchemeIndex", "123"),
        ("EffectSchemeIndex", "123"),
        ("ConnectorSchemeIndex", "123"),
        ("FontSchemeIndex", "123"),
        ("QuickStyleFontColor", "0"),
        ("QuickStyleLineColor", "0"),
        ("QuickStyleFontMatrix", "1"),
        ("QuickStyleLineMatrix", "1"),
        ("QuickStyleType", "0"),
        ("QuickStyleFillMatrix", "1"),
        ("QuickStyleFillColor", "0"),
        ("LineColor", "Themed"),
        ("LineWeight", "Themed"),
        ("FillPattern", "Themed"),
    ] {
        set(&mut shape, name, value);
    }
    document.pages[0].shapes.push(shape);
    document
}
fn rgb(value: Value) -> [u8; 3] {
    match value {
        Value::Color { rgb, .. } => rgb,
        other => panic!("expected colour, got {other:?}"),
    }
}

/// QuickStyleType takes precedence; the inherited endpoint section supplies OneD.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.oned
/// https://learn.microsoft.com/en-us/office/client-developer/visio/quickstyletype-cell-quick-style-section
#[test]
fn connector_style_selection_uses_shape_property_and_explicit_precedence() {
    for (endpoints, quick, shape_type, connector) in [
        (false, 0, "Shape", false),
        (true, 0, "Shape", true),
        (true, 1, "Shape", true),
        (false, 1, "Shape", true),
        (true, 2, "Shape", false),
        (false, 3, "Shape", true),
        (false, 2, "Shape", false),
        (true, 0, "Guide", false),
        (true, 0, "Foreign", false),
        (true, 3, "Foreign", true),
    ] {
        for inherited in [false, true] {
            let mut document = document();
            let shape = &mut document.pages[0].shapes[0];
            set(shape, "QuickStyleType", &quick.to_string());
            shape.shape_type = Some(shape_type.into());
            if endpoints {
                if inherited {
                    let mut source = Shape { id: 7, ..Default::default() };
                    set(&mut source, "BeginX", "0");
                    shape.master = Some(17);
                    shape.master_shape = Some(7);
                    document.masters.push(Master { id: 17, shapes: vec![source].into(), ..Default::default() });
                } else {
                    set(shape, "BeginX", "0");
                }
            }
            let themes = misterdiagram_theme::Themes::from_document(&document).unwrap();
            let mut evaluator = Evaluator::new(&document).unwrap().with_theme_host(&themes);
            let scope = Scope::PageShape { page: 0, shape: 1 };
            assert_eq!(rgb(evaluator.evaluate(scope, "Char.Color")), if connector { [20, 40, 60] } else { [225, 210, 195] });
            assert_eq!(rgb(evaluator.evaluate(scope, "LineColor")), if connector { [20, 40, 60] } else { [49, 66, 83] });
            assert_eq!(evaluator.evaluate(scope, "LineWeight").numeric(), Ok(if connector { 19050. / 914400. } else { 28575. / 914400. }));
            assert_eq!(evaluator.evaluate(scope, "FillPattern").numeric(), Ok(if connector { 1. } else { 7. }));
            assert_eq!(evaluator.evaluate(scope, "FillGradientEnabled").numeric(), Ok(0.));
            assert!((evaluator.evaluate(scope, "FillGradientAngle").numeric().unwrap() - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
        }
    }
}
