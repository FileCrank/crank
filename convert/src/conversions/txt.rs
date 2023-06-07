use crate::error::ConversionResult;
use crate::format::{ChunkFn, ConversionFormat, Format};
use crate::writers::docx::DocxWriter;
use docx_rs::{Docx, Paragraph, Run};
use std::io::{copy, BufRead, Read, Seek, Write};

/*
TODO: use this
impl ConversionFormat for Format::Txt {
    /// A chunk of a text file is a line of text
    type ChunkType = String;

    fn read(source: &mut dyn BufRead, recv: &ChunkFn<Self::ChunkType>) -> ConversionResult<()> {
        loop {
            let mut buf = String::new();

            match source.read_line(&mut buf)? {
                // If we read 0 bytes, there's nothing left from the source
                0 => return Ok(()),
                _ => recv(&buf)?,
            };
        }
    }
}

 */

pub trait WriteSeek: Write + Seek {}

pub fn txt_to_docx(source: &mut dyn BufRead, sink: &mut dyn WriteSeek) -> ConversionResult<()> {
    // TODO: use DocxWriter
    let mut buf = String::new();
    source.read_to_string(&mut buf)?;
    let document = Docx::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(buf)));
    let built_doc = document.build();
    built_doc.pack(sink)?;

    Ok(())
}
