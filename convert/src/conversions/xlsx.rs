use crate::error::ConversionResult;
use crate::format::Source;
use calamine::{open_workbook_from_rs, Error, RangeDeserializerBuilder, Reader, XlsOptions, Xlsx};
use std::io::{Cursor, Write};

pub fn xlsx_to_csv(source: &mut dyn Source, dest: &mut dyn Write) -> ConversionResult<()> {
    let seeker = Cursor::new(source);
    let workbook: Xlsx<_> = open_workbook_from_rs(seeker)?;

    Ok(())
}
