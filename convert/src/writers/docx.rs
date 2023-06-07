use comrak::nodes::NodeValue::Document;
use docx_rs::{Docx, Paragraph, Run};
use std::io::Write;

pub struct DocxWriter<'a> {
    sink: &'a mut dyn Write,
    document: Docx,
}

impl<'a> DocxWriter<'a> {
    pub fn new(sink: &'a mut dyn Write) -> Self {
        Self {
            sink,
            document: Docx::new(),
        }
    }
}
