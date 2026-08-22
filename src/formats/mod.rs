//! One frontend per input format; each parses bytes into the document model.

mod csv;
pub mod detect;
mod doc;
mod docx;
mod epub;
mod odf;
pub mod pdf;
mod ppt;
mod pptx;
mod rtf;
mod sheet;

use crate::Format;
use crate::error::ConvertError;
use crate::model::{Document, LocatedDocument, SourceMap};

pub fn parse(bytes: &[u8], format: Format) -> Result<Document, ConvertError> {
    match format {
        Format::Excel => sheet::parse(bytes),
        Format::Csv => csv::parse(bytes),
        Format::Docx => docx::parse(bytes),
        Format::Odt | Format::Ods | Format::Odp => odf::parse(bytes),
        Format::Pptx => pptx::parse(bytes),
        Format::Epub => epub::parse(bytes),
        Format::Rtf => rtf::parse(bytes),
        // RTF files wearing a .doc extension are common in the wild.
        Format::Doc if bytes.starts_with(b"{\\rtf") => rtf::parse(bytes),
        Format::Doc => doc::parse(bytes),
        Format::Ppt => ppt::parse(bytes),
        // pdf-inspector produces Markdown directly; there is no document
        // model for PDFs. `to_markdown_bytes` routes them to `pdf`.
        Format::Pdf => Err(ConvertError::Unsupported(
            "PDF converts directly to Markdown; use to_markdown or to_markdown_bytes".to_string(),
        )),
    }
}

pub fn parse_with_locations(bytes: &[u8], format: Format) -> Result<LocatedDocument, ConvertError> {
    match format {
        Format::Pptx => pptx::parse_with_locations(bytes),
        Format::Epub => epub::parse_with_locations(bytes),
        Format::Odt | Format::Ods | Format::Odp => odf::parse_with_locations(bytes),
        // DOCX discovers its main part through package relationships. Until the
        // frontend returns that resolved path, do not guess the conventional
        // `word/document.xml` name here.
        Format::Docx => Ok(with_outline_locations(docx::parse(bytes)?, None)),
        Format::Rtf => Ok(with_outline_locations(rtf::parse(bytes)?, None)),
        Format::Doc if bytes.starts_with(b"{\\rtf") => {
            Ok(with_outline_locations(rtf::parse(bytes)?, None))
        }
        Format::Doc => Ok(with_outline_locations(doc::parse(bytes)?, None)),
        // These frontends do not yet expose reliable source units. Preserve
        // the existing document exactly and return an empty map rather than
        // inventing boundaries.
        Format::Excel | Format::Csv | Format::Ppt => parse(bytes, format).map(empty_locations),
        Format::Pdf => parse(bytes, format).map(empty_locations),
    }
}

fn with_outline_locations(document: Document, origin_part: Option<&str>) -> LocatedDocument {
    let mut source_map = SourceMap::default();
    source_map.push_outline_sections(&document, origin_part.map(str::to_string));
    LocatedDocument { document, source_map }
}

fn empty_locations(document: Document) -> LocatedDocument {
    LocatedDocument { document, source_map: SourceMap::default() }
}
