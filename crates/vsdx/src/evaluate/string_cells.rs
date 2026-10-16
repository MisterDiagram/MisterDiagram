//! Declared text cells, independent of the optional serialized U attribute.
use diagramcraft_model::Cell;

/// Classify by section and column: Field.Format and Property.Value are not string columns.
/// Font and language identities also keep their existing numeric interpretation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/label-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/prompt-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/format-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sortkey-cell-shape-data-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-layer-sectionvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-hyperlink-rowvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-controls-rowvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-user-defined-cells-sectionvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bulletstring-cell-paragraph-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/comment-cell-miscellaneous-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/comment-cell-annotation-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/name-cell-reviewer-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/initials-cell-reviewer-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/menu-cell-actions-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/prompt-cell-actions-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sortkey-cell-actions-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/tagname-cell-actions-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/buttonface-cell-actions-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-element-action-tag-sectionvisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/buttonface-cell-action-tags-section
pub(super) fn is_string(name: &str, cell: &Cell) -> bool {
    // Evaluator canonical names are lowercase; column spelling in XML is case sensitive.
    let column = cell.name.rsplit('.').next().unwrap_or(&cell.name).split('[').next().unwrap_or("").to_ascii_lowercase();
    let (section, _) = name.split_once('.').unwrap_or(("", name));
    match section {
        "" => matches!(column.as_str(), "comment" | "copyright" | "helptopic" | "shapekeywords" | "layermember"),
        "prop" => matches!(column.as_str(), "label" | "prompt" | "format" | "sortkey"),
        "layers" => matches!(column.as_str(), "name" | "nameuniv" | "password"),
        "user" | "controls" | "connections" => column == "prompt",
        "para" => column == "bulletstr",
        "hyperlink" => matches!(column.as_str(), "description" | "address" | "subaddress" | "extrainfo" | "frame"),
        "actions" => matches!(column.as_str(), "menu" | "prompt" | "tagname" | "buttonface" | "sortkey"),
        "actiontag" | "smarttag" | "smarttags" => matches!(column.as_str(), "tagname" | "description" | "buttonface"),
        "reviewer" => matches!(column.as_str(), "name" | "initials"),
        "annotation" => column == "comment",
        _ => false,
    }
}
