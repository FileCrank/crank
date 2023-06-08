use crate::error::ConversionResult;
use comrak::nodes::NodeValue::Document;
use docx_rs::{Docx, Paragraph, Run};
use std::io::{Cursor, Seek, Write};

pub struct DocxWriter<'a> {
    sink: &'a mut dyn Write,
    document: Box<Docx>,
}

impl<'a> DocxWriter<'a> {
    pub fn new(sink: &'a mut dyn Write) -> Self {
        Self {
            sink,
            document: Box::new(Docx::new()),
        }
    }

    pub fn write_text(&mut self, text: String) {
        self.document
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text(text)));
    }

    pub fn build(self) -> ConversionResult<()> {
        // TODO: should be able to do this without the intermediate buf by wrapping self.sink in a Cursor
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let built_doc = self.document.build();
        built_doc.pack(&mut buf)?;
        self.sink.write_all(buf.into_inner().as_slice())?;
        Ok(())
    }
}
