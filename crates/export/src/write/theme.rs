//! Theme skeleton retargeted from storytold/deckcraft
//! crates/pptx/src/write/theme.rs, 90058395b78be26bb7ea7b4914a9f113c28d979d.
//! Copyright (c) 2026 ArtCraft Team and the DeckCraft contributors.
//! SPDX-License-Identifier: MIT OR Apache-2.0
use misterdiagram_color::Rgba;
use misterdiagram_opc::{
    NS_A,
    xml::{A, W},
};

pub(super) fn write(colors: &[Rgba; 12]) -> Vec<u8> {
    let mut w = W::new();
    w.open("a:theme", A::new().a("xmlns:a", NS_A).a("name", "Resolved drawing colours"));
    w.open0("a:themeElements");
    w.open("a:clrScheme", A::new().a("name", "Resolved drawing colours"));
    for (slot, color) in
        ["dk1", "lt1", "dk2", "lt2", "accent1", "accent2", "accent3", "accent4", "accent5", "accent6", "hlink", "folHlink"].iter().zip(colors)
    {
        let tag = format!("a:{slot}");
        w.open0(&tag);
        w.val("a:srgbClr", color.hex());
        w.close(&tag);
    }
    w.close("a:clrScheme");
    w.open("a:fontScheme", A::new().a("name", "MisterDiagram"));
    for tag in ["a:majorFont", "a:minorFont"] {
        w.open0(tag);
        w.empty("a:latin", A::new().a("typeface", "Liberation Sans"));
        w.empty("a:ea", A::new().a("typeface", ""));
        w.empty("a:cs", A::new().a("typeface", ""));
        w.close(tag);
    }
    w.close("a:fontScheme");
    w.open("a:fmtScheme", A::new().a("name", "Explicit drawing properties"));
    for list in ["a:fillStyleLst", "a:lnStyleLst", "a:effectStyleLst", "a:bgFillStyleLst"] {
        w.open0(list);
        for _ in 0..3 {
            match list {
                "a:lnStyleLst" => {
                    w.open("a:ln", A::new().a("w", 0));
                    w.empty0("a:noFill");
                    w.close("a:ln");
                }
                "a:effectStyleLst" => {
                    w.open0("a:effectStyle");
                    w.empty0("a:effectLst");
                    w.close("a:effectStyle");
                }
                _ => w.empty0("a:noFill"),
            }
        }
        w.close(list);
    }
    w.close("a:fmtScheme");
    w.close("a:themeElements");
    w.close("a:theme");
    w.finish()
}
