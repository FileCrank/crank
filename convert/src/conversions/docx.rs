use crate::error::ConversionResult;
use std::fmt::Write;
use std::io::BufRead;

pub fn docx_to_md(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    Ok(())
}
