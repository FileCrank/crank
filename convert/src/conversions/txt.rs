use crate::error::ConversionResult;
use crate::format::Source;
use crate::writers::docx::DocxWriter;
use std::io::Write;

pub fn txt_to_docx(source: &mut dyn Source, sink: &mut dyn Write) -> ConversionResult<()> {
    let writer = DocxWriter::new(sink);

    let mut buf = String::new();
    source.read_to_string(&mut buf)?;

    writer.write_text(buf).build()?;

    Ok(())
}
