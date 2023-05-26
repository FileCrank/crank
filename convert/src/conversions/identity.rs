use crate::error::ConversionResult;
use std::io::{copy, BufRead, Write};

pub fn identity_conversion(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    copy(source, sink)?;
    Ok(())
}
