//! Source-location types for the Python binding.

use anydoc::model;
use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::document;

#[pyclass(frozen, get_all, module = "anydoc")]
pub struct LocatedDocument {
    document: Py<document::Document>,
    source_map: Py<SourceMap>,
}

pub fn located_document(
    py: Python<'_>,
    located: model::LocatedDocument,
) -> PyResult<LocatedDocument> {
    Ok(LocatedDocument {
        document: Py::new(py, document::document(py, located.document)?)?,
        source_map: Py::new(py, source_map(py, located.source_map)?)?,
    })
}

#[pyclass(frozen, get_all, module = "anydoc")]
pub struct SourceMap {
    units: Py<PyList>,
    spans: Py<PyList>,
}

fn source_map(py: Python<'_>, source_map: model::SourceMap) -> PyResult<SourceMap> {
    let units: Vec<Py<SourceUnit>> = source_map
        .units
        .into_iter()
        .map(|unit| Py::new(py, SourceUnit::from(unit)))
        .collect::<PyResult<_>>()?;
    let spans: Vec<Py<SourceSpan>> = source_map
        .spans
        .into_iter()
        .map(|span| Py::new(py, source_span(py, span)?))
        .collect::<PyResult<_>>()?;
    Ok(SourceMap {
        units: PyList::new(py, units)?.unbind(),
        spans: PyList::new(py, spans)?.unbind(),
    })
}

#[pyclass(frozen, get_all, module = "anydoc")]
pub struct SourceUnit {
    /// slide, spine_item, or outline_section.
    kind: &'static str,
    /// Zero-based position in the source's own order.
    index: usize,
    /// Source-defined name when one exists.
    name: Option<String>,
    /// Package part or stream this unit came from when available.
    origin_part: Option<String>,
}

impl From<model::SourceUnit> for SourceUnit {
    fn from(unit: model::SourceUnit) -> Self {
        let kind = match unit.kind {
            model::SourceUnitKind::Slide => "slide",
            model::SourceUnitKind::SpineItem => "spine_item",
            model::SourceUnitKind::OutlineSection => "outline_section",
            _ => unreachable!("unsupported source-unit kind in this binding version"),
        };
        SourceUnit { kind, index: unit.index, name: unit.name, origin_part: unit.origin_part }
    }
}

#[pyclass(frozen, get_all, module = "anydoc")]
pub struct SourceSpan {
    /// Index into `source_map.units`.
    unit_index: usize,
    /// Half-open range in `document.blocks`.
    block_start: usize,
    block_end: usize,
    coordinates: Option<Py<SourceCoordinates>>,
}

fn source_span(py: Python<'_>, span: model::SourceSpan) -> PyResult<SourceSpan> {
    Ok(SourceSpan {
        unit_index: span.unit_index,
        block_start: span.block_start,
        block_end: span.block_end,
        coordinates: span
            .coordinates
            .map(|coordinates| Py::new(py, SourceCoordinates::from(coordinates)))
            .transpose()?,
    })
}

#[pyclass(frozen, get_all, module = "anydoc")]
pub struct SourceCoordinates {
    /// outline_level.
    kind: &'static str,
    /// Source heading/outline depth, one-based.
    level: u8,
}

impl From<model::SourceCoordinates> for SourceCoordinates {
    fn from(coordinates: model::SourceCoordinates) -> Self {
        match coordinates {
            model::SourceCoordinates::OutlineLevel { level } => {
                SourceCoordinates { kind: "outline_level", level }
            }
            _ => unreachable!("unsupported source-coordinate kind in this binding version"),
        }
    }
}
