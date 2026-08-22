//! Source locations retained alongside a parsed document.

use super::{Block, Document, inlines_to_plain_text};

/// A parsed document together with optional source-defined structural locations.
///
/// This is intentionally separate from [`Document`] so callers that use the
/// existing document model are not required to change when provenance is
/// added or expanded.
#[derive(Debug, Clone, Default)]
pub struct LocatedDocument {
    /// The existing parsed document model.
    pub document: Document,
    /// Source units and their mappings to top-level document blocks.
    pub source_map: SourceMap,
}

/// Source-defined structural units and their mappings to top-level blocks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceMap {
    /// Source-defined units that contributed structured locations.
    pub units: Vec<SourceUnit>,
    /// Half-open ranges into [`Document::blocks`](super::Document::blocks).
    pub spans: Vec<SourceSpan>,
}

/// One source-defined unit that contributed located document content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceUnit {
    /// The source structure this unit represents.
    pub kind: SourceUnitKind,
    /// Zero-based position in the source's own order. Gaps are possible when
    /// earlier source units were duplicate, empty, or unusable.
    pub index: usize,
    /// Source-defined name when one exists, such as a slide or section name.
    pub name: Option<String>,
    /// Package part or stream this unit came from when the format exposes one.
    pub origin_part: Option<String>,
}

/// Kind of source-defined unit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceUnitKind {
    /// One presentation slide/page.
    Slide,
    /// One EPUB spine item in publication reading order.
    SpineItem,
    /// A section defined by the source document's outline/heading structure.
    OutlineSection,
}

/// A source unit's contribution to the top-level document block stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    /// Index into [`SourceMap::units`].
    pub unit_index: usize,
    /// First top-level block in the span.
    pub block_start: usize,
    /// First top-level block after the span.
    pub block_end: usize,
    /// Finer source coordinates when the format exposes them.
    pub coordinates: Option<SourceCoordinates>,
}

/// Format-native coordinates within a source unit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCoordinates {
    /// Source-assigned outline level for a document heading, one-based.
    OutlineLevel {
        /// Source heading/outline depth.
        level: u8,
    },
}

impl SourceMap {
    pub(crate) fn push_unit(
        &mut self,
        kind: SourceUnitKind,
        index: usize,
        name: Option<String>,
        origin_part: Option<String>,
    ) -> usize {
        let unit_index = self.units.len();
        self.units.push(SourceUnit { kind, index, name, origin_part });
        unit_index
    }

    pub(crate) fn push_span(
        &mut self,
        unit_index: usize,
        block_start: usize,
        block_end: usize,
        coordinates: Option<SourceCoordinates>,
    ) {
        debug_assert!(unit_index < self.units.len());
        if block_start >= block_end {
            return;
        }
        self.spans.push(SourceSpan {
            unit_index,
            block_start,
            block_end,
            coordinates,
        });
    }

    /// Retain source-defined outline headings as nested section ranges.
    ///
    /// Each heading owns content through the next heading at the same or a
    /// shallower source outline level. Nested outline sections therefore have
    /// intentionally overlapping block spans. This uses heading semantics the
    /// format frontend already resolved from the source; it does not infer
    /// pagination or synthesize section boundaries from layout.
    pub(crate) fn push_outline_sections(
        &mut self,
        document: &Document,
        origin_part: Option<String>,
    ) {
        let headings: Vec<(usize, u8, String)> = document
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(block_index, block)| match block {
                Block::Heading { level, content, .. } => Some((
                    block_index,
                    *level,
                    inlines_to_plain_text(content).trim().to_string(),
                )),
                _ => None,
            })
            .collect();

        for (source_index, (block_start, level, name)) in headings.iter().enumerate() {
            let block_end = headings
                .iter()
                .skip(source_index + 1)
                .find(|(_, next_level, _)| *next_level <= *level)
                .map(|(index, _, _)| *index)
                .unwrap_or(document.blocks.len());
            let unit_index = self.push_unit(
                SourceUnitKind::OutlineSection,
                source_index,
                (!name.is_empty()).then(|| name.clone()),
                origin_part.clone(),
            );
            self.push_span(
                unit_index,
                *block_start,
                block_end,
                Some(SourceCoordinates::OutlineLevel { level: *level }),
            );
        }
    }
}
