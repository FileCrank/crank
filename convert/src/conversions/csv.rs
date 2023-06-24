use crate::error::ConversionResult;
use std::io::{BufRead, Write};

pub fn csv_to_txt(source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    let mut rdr = csv::Reader::from_reader(source);
    for result in rdr.records() {
        let record = result?;

        for column in record.iter() {
            dest.write(column.as_bytes())?;
            dest.write(b" ")?;
        }
        dest.write(b"\n")?;
    }
    Ok(())
}
