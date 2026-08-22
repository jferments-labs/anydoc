use anydoc::model::{Block, SourceCoordinates, SourceUnitKind};
use anydoc::{Format, to_document, to_document_with_locations};

const PPTX: &[u8] = include_bytes!("source-location-fixtures/two-slides.pptx");
const EPUB: &[u8] = include_bytes!("source-location-fixtures/two-chapters.epub");
const ODP: &[u8] = include_bytes!("source-location-fixtures/two-slides.odp");
const ODT: &[u8] = include_bytes!("source-location-fixtures/outline.odt");
const DOCX: &[u8] = include_bytes!("fixtures/docx/handmade-outline.docx");

#[test]
fn existing_document_struct_literal_remains_source_compatible() {
    let _: anydoc::model::Document = anydoc::model::Document {
        blocks: Vec::new(),
        notes: Vec::new(),
        assets: Vec::new(),
    };
}

#[test]
fn existing_to_document_shape_is_unchanged() {
    let document = to_document(PPTX, Format::Pptx).unwrap();
    assert_eq!(document.blocks.len(), 2);
    assert!(document.blocks.iter().all(|block| matches!(block, Block::Paragraph(_))));
}

#[test]
fn pptx_maps_each_slide_to_its_own_block_span() {
    let located = to_document_with_locations(PPTX, Format::Pptx).unwrap();
    assert_eq!(located.source_map.units.len(), 2);
    assert_eq!(located.source_map.spans.len(), 2);
    for (index, unit) in located.source_map.units.iter().enumerate() {
        assert_eq!(unit.kind, SourceUnitKind::Slide);
        assert_eq!(unit.index, index);
        let expected_part = format!("ppt/slides/slide{}.xml", index + 1);
        assert_eq!(unit.origin_part.as_deref(), Some(expected_part.as_str()));
    }
    assert_eq!((located.source_map.spans[0].block_start, located.source_map.spans[0].block_end), (0, 1));
    assert_eq!((located.source_map.spans[1].block_start, located.source_map.spans[1].block_end), (1, 2));
}

#[test]
fn epub_maps_original_spine_items() {
    let located = to_document_with_locations(EPUB, Format::Epub).unwrap();
    assert_eq!(located.source_map.units.len(), 2);
    assert_eq!(located.source_map.spans.len(), 2);
    assert_eq!(located.source_map.units[0].kind, SourceUnitKind::SpineItem);
    assert_eq!(located.source_map.units[0].index, 0);
    assert_eq!(located.source_map.units[0].origin_part.as_deref(), Some("chapter1.xhtml"));
    assert_eq!(located.source_map.units[1].kind, SourceUnitKind::SpineItem);
    assert_eq!(located.source_map.units[1].index, 1);
    assert_eq!(located.source_map.units[1].origin_part.as_deref(), Some("chapter2.xhtml"));
    assert_eq!((located.source_map.spans[0].block_start, located.source_map.spans[0].block_end), (0, 2));
    assert_eq!((located.source_map.spans[1].block_start, located.source_map.spans[1].block_end), (2, 4));
}

#[test]
fn odp_maps_draw_pages_as_slides() {
    let located = to_document_with_locations(ODP, Format::Odp).unwrap();
    assert_eq!(located.source_map.units.len(), 2);
    assert_eq!(located.source_map.spans.len(), 2);
    assert_eq!(located.source_map.units[0].kind, SourceUnitKind::Slide);
    assert_eq!(located.source_map.units[0].name.as_deref(), Some("Intro"));
    assert_eq!(located.source_map.units[0].origin_part.as_deref(), Some("content.xml"));
    assert_eq!(located.source_map.units[1].kind, SourceUnitKind::Slide);
    assert_eq!(located.source_map.units[1].name.as_deref(), Some("Details"));
    assert_eq!(located.source_map.units[1].origin_part.as_deref(), Some("content.xml"));
    assert_eq!((located.source_map.spans[0].block_start, located.source_map.spans[0].block_end), (0, 2));
    assert_eq!((located.source_map.spans[1].block_start, located.source_map.spans[1].block_end), (2, 4));
}

#[test]
fn odt_outline_headings_become_nested_source_sections() {
    let located = to_document_with_locations(ODT, Format::Odt).unwrap();
    assert_eq!(located.source_map.units.len(), 3);
    assert_eq!(located.source_map.units[0].kind, SourceUnitKind::OutlineSection);
    assert_eq!(located.source_map.units[0].name.as_deref(), Some("Alpha"));
    assert_eq!(located.source_map.units[0].origin_part.as_deref(), Some("content.xml"));
    assert_eq!(located.source_map.units[1].name.as_deref(), Some("Beta"));
    assert_eq!(located.source_map.units[2].name.as_deref(), Some("Gamma"));
    assert_eq!(
        located.source_map.spans[0].coordinates,
        Some(SourceCoordinates::OutlineLevel { level: 1 })
    );
    assert_eq!(
        located.source_map.spans[1].coordinates,
        Some(SourceCoordinates::OutlineLevel { level: 2 })
    );
    assert_eq!((located.source_map.spans[0].block_start, located.source_map.spans[0].block_end), (0, 4));
    assert_eq!((located.source_map.spans[1].block_start, located.source_map.spans[1].block_end), (2, 4));
    assert_eq!((located.source_map.spans[2].block_start, located.source_map.spans[2].block_end), (4, 6));
}

#[test]
fn docx_retains_source_outline_sections_without_changing_document_api() {
    let plain = to_document(DOCX, Format::Docx).unwrap();
    let located = to_document_with_locations(DOCX, Format::Docx).unwrap();
    assert_eq!(plain.blocks.len(), located.document.blocks.len());
    assert!(located.source_map.units.iter().any(|unit| unit.kind == SourceUnitKind::OutlineSection));
    // The DOCX frontend can resolve a non-conventional main-part path, but the
    // generic located wrapper does not receive that path yet. Leave it unknown
    // rather than incorrectly claiming every DOCX came from word/document.xml.
    assert!(located.source_map.units.iter().all(|unit| unit.origin_part.is_none()));
    assert!(located.source_map.spans.iter().any(|span| {
        matches!(span.coordinates, Some(SourceCoordinates::OutlineLevel { .. }))
    }));
}
