//! Source-location types for the Node.js binding.

use anydoc::model;
use napi_derive::napi;

use crate::document::Document;

#[napi(object)]
pub struct LocatedDocument {
    pub document: Document,
    pub source_map: SourceMap,
}

impl From<model::LocatedDocument> for LocatedDocument {
    fn from(located: model::LocatedDocument) -> Self {
        LocatedDocument {
            document: located.document.into(),
            source_map: located.source_map.into(),
        }
    }
}

#[napi(object)]
pub struct SourceMap {
    pub units: Vec<SourceUnit>,
    pub spans: Vec<SourceSpan>,
}

impl From<model::SourceMap> for SourceMap {
    fn from(source_map: model::SourceMap) -> Self {
        SourceMap {
            units: source_map.units.into_iter().map(SourceUnit::from).collect(),
            spans: source_map.spans.into_iter().map(SourceSpan::from).collect(),
        }
    }
}

#[napi(string_enum)]
#[allow(non_camel_case_types)]
pub enum SourceUnitKind {
    slide,
    spineItem,
    outlineSection,
}

#[napi(object)]
pub struct SourceUnit {
    pub kind: SourceUnitKind,
    /// Zero-based position in the source's own order.
    pub index: u32,
    /// Source-defined name when one exists.
    pub name: Option<String>,
    /// Package part or stream this unit came from when available.
    pub origin_part: Option<String>,
}

impl From<model::SourceUnit> for SourceUnit {
    fn from(unit: model::SourceUnit) -> Self {
        let kind = match unit.kind {
            model::SourceUnitKind::Slide => SourceUnitKind::slide,
            model::SourceUnitKind::SpineItem => SourceUnitKind::spineItem,
            model::SourceUnitKind::OutlineSection => SourceUnitKind::outlineSection,
            _ => unreachable!("unsupported source-unit kind in this binding version"),
        };
        SourceUnit {
            kind,
            index: unit.index.min(u32::MAX as usize) as u32,
            name: unit.name,
            origin_part: unit.origin_part,
        }
    }
}

#[napi(object)]
pub struct SourceSpan {
    /// Index into `sourceMap.units`.
    pub unit_index: u32,
    /// Half-open range in `document.blocks`.
    pub block_start: u32,
    pub block_end: u32,
    /// Finer format-native coordinates when available.
    pub coordinates: Option<SourceCoordinates>,
}

impl From<model::SourceSpan> for SourceSpan {
    fn from(span: model::SourceSpan) -> Self {
        SourceSpan {
            unit_index: span.unit_index.min(u32::MAX as usize) as u32,
            block_start: span.block_start.min(u32::MAX as usize) as u32,
            block_end: span.block_end.min(u32::MAX as usize) as u32,
            coordinates: span.coordinates.map(SourceCoordinates::from),
        }
    }
}

#[napi(string_enum)]
#[allow(non_camel_case_types)]
pub enum SourceCoordinatesKind {
    outlineLevel,
}

#[napi(object)]
pub struct SourceCoordinates {
    pub kind: SourceCoordinatesKind,
    /// Source heading/outline depth, one-based.
    pub level: u32,
}

impl From<model::SourceCoordinates> for SourceCoordinates {
    fn from(coordinates: model::SourceCoordinates) -> Self {
        match coordinates {
            model::SourceCoordinates::OutlineLevel { level } => SourceCoordinates {
                kind: SourceCoordinatesKind::outlineLevel,
                level: level.into(),
            },
            _ => unreachable!("unsupported source-coordinate kind in this binding version"),
        }
    }
}
