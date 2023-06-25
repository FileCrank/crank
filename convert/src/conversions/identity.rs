use crate::error::ConversionResult;
use crate::format::Source;
use std::io::{copy, Write};

pub fn identity_conversion(source: &mut dyn Source, sink: &mut dyn Write) -> ConversionResult<()> {
    copy(source, sink)?;
    Ok(())
}
