#!/usr/bin/env python3
"""Generate deterministic fixtures used by tests/source_locations.rs."""

from __future__ import annotations

from pathlib import Path
from zipfile import ZIP_DEFLATED, ZIP_STORED, ZipFile, ZipInfo

OUT = Path(__file__).resolve().parent / "source-location-fixtures"
STAMP = (1980, 1, 1, 0, 0, 0)


def write_zip(path: Path, entries: list[tuple[str, str, int]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with ZipFile(path, "w") as archive:
        for name, body, compression in entries:
            info = ZipInfo(name, STAMP)
            info.compress_type = compression
            info.external_attr = 0o100644 << 16
            archive.writestr(info, body.encode())


def slide(text: str) -> str:
    return f"""<?xml version="1.0" encoding="UTF-8"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <p:cSld><p:spTree>
    <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
    <p:grpSpPr/>
    <p:sp>
      <p:nvSpPr><p:cNvPr id="2" name="Text"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
      <p:spPr/>
      <p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>{text}</a:t></a:r></a:p></p:txBody>
    </p:sp>
  </p:spTree></p:cSld>
</p:sld>"""


def pptx() -> None:
    write_zip(
        OUT / "two-slides.pptx",
        [
            (
                "[Content_Types].xml",
                """<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
  <Override PartName="/ppt/slides/slide2.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>""",
                ZIP_DEFLATED,
            ),
            (
                "_rels/.rels",
                """<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>""",
                ZIP_DEFLATED,
            ),
            (
                "ppt/presentation.xml",
                """<?xml version="1.0" encoding="UTF-8"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId1"/>
    <p:sldId id="257" r:id="rId2"/>
  </p:sldIdLst>
  <p:defaultTextStyle/>
</p:presentation>""",
                ZIP_DEFLATED,
            ),
            (
                "ppt/_rels/presentation.xml.rels",
                """<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide2.xml"/>
</Relationships>""",
                ZIP_DEFLATED,
            ),
            ("ppt/slides/slide1.xml", slide("First slide"), ZIP_DEFLATED),
            ("ppt/slides/slide2.xml", slide("Second slide"), ZIP_DEFLATED),
        ],
    )


def epub() -> None:
    write_zip(
        OUT / "two-chapters.epub",
        [
            ("mimetype", "application/epub+zip", ZIP_STORED),
            (
                "META-INF/container.xml",
                """<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="book.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>""",
                ZIP_DEFLATED,
            ),
            (
                "book.opf",
                """<?xml version="1.0" encoding="UTF-8"?>
<package version="3.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata/>
  <manifest>
    <item id="one" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
    <item id="two" href="chapter2.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine><itemref idref="one"/><itemref idref="two"/></spine>
</package>""",
                ZIP_DEFLATED,
            ),
            (
                "chapter1.xhtml",
                """<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body><p>First chapter</p></body></html>""",
                ZIP_DEFLATED,
            ),
            (
                "chapter2.xhtml",
                """<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml"><body><p>Second chapter</p></body></html>""",
                ZIP_DEFLATED,
            ),
        ],
    )


def odp() -> None:
    write_zip(
        OUT / "two-slides.odp",
        [
            ("mimetype", "application/vnd.oasis.opendocument.presentation", ZIP_STORED),
            (
                "content.xml",
                """<?xml version="1.0" encoding="UTF-8"?>
<office:document-content
 xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
 xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
 xmlns:presentation="urn:oasis:names:tc:opendocument:xmlns:presentation:1.0"
 xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">
 <office:body><office:presentation>
  <draw:page draw:name="Intro">
   <draw:frame presentation:class="title"><draw:text-box><text:p>First slide</text:p></draw:text-box></draw:frame>
   <draw:frame presentation:class="outline"><draw:text-box><text:p>First body</text:p></draw:text-box></draw:frame>
  </draw:page>
  <draw:page draw:name="Details">
   <draw:frame presentation:class="title"><draw:text-box><text:p>Second slide</text:p></draw:text-box></draw:frame>
   <draw:frame presentation:class="outline"><draw:text-box><text:p>Second body</text:p></draw:text-box></draw:frame>
  </draw:page>
 </office:presentation></office:body>
</office:document-content>""",
                ZIP_DEFLATED,
            ),
        ],
    )


def odt() -> None:
    write_zip(
        OUT / "outline.odt",
        [
            ("mimetype", "application/vnd.oasis.opendocument.text", ZIP_STORED),
            (
                "content.xml",
                """<?xml version="1.0" encoding="UTF-8"?>
<office:document-content
 xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
 xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">
 <office:body><office:text>
  <text:h text:outline-level="1">Alpha</text:h><text:p>Alpha body</text:p>
  <text:h text:outline-level="2">Beta</text:h><text:p>Beta body</text:p>
  <text:h text:outline-level="1">Gamma</text:h><text:p>Gamma body</text:p>
 </office:text></office:body>
</office:document-content>""",
                ZIP_DEFLATED,
            ),
        ],
    )


if __name__ == "__main__":
    pptx()
    epub()
    odp()
    odt()
