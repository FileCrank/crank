use std::io::Write;

pub struct DocxWriter<'a> {
    sink: &'a mut dyn Write,
}

impl<'a> DocxWriter<'a> {
    pub fn new(sink: &'a mut dyn Write) -> Self {
        Self { sink }
    }
}
