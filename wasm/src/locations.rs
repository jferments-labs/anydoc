//! Serializable source-location model for WebAssembly.

use anydoc::model;
use serde::Serialize;

use crate::document::Document;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocatedDocument {
    document: Document,
    source_map: SourceMap,
}

impl From<model::LocatedDocument> for LocatedDocument {
    fn from(located: model::LocatedDocument) -> Self {
        LocatedDocument {
            document: located.document.into(),
            source_map: located.source_map.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceMap {
    units: Vec<SourceUnit>,
    spans: Vec<SourceSpan>,
}

impl From<model::SourceMap> for SourceMap {
    fn from(source_map: model::SourceMap) -> Self {
        SourceMap {
            units: source_map.units.into_iter().map(SourceUnit::from).collect(),
            spans: source_map.spans.into_iter().map(SourceSpan::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceUnit {
    kind: &'static str,
    index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_part: Option<String>,
}

impl From<model::SourceUnit> for SourceUnit {
    fn from(unit: model::SourceUnit) -> Self {
        let kind = match unit.kind {
            model::SourceUnitKind::Slide => "slide",
            model::SourceUnitKind::SpineItem => "spineItem",
            model::SourceUnitKind::OutlineSection => "outlineSection",
            _ => unreachable!("unsupported source-unit kind in this binding version"),
        };
        SourceUnit { kind, index: unit.index, name: unit.name, origin_part: unit.origin_part }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceSpan {
    unit_index: usize,
    block_start: usize,
    block_end: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordinates: Option<SourceCoordinates>,
}

impl From<model::SourceSpan> for SourceSpan {
    fn from(span: model::SourceSpan) -> Self {
        SourceSpan {
            unit_index: span.unit_index,
            block_start: span.block_start,
            block_end: span.block_end,
            coordinates: span.coordinates.map(SourceCoordinates::from),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceCoordinates {
    kind: &'static str,
    level: u8,
}

impl From<model::SourceCoordinates> for SourceCoordinates {
    fn from(coordinates: model::SourceCoordinates) -> Self {
        match coordinates {
            model::SourceCoordinates::OutlineLevel { level } => {
                SourceCoordinates { kind: "outlineLevel", level }
            }
            _ => unreachable!("unsupported source-coordinate kind in this binding version"),
        }
    }
}
