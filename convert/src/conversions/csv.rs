use crate::error::ConversionResult;
use crate::format::Source;
use std::io::Write;

pub fn csv_to_txt(source: &mut dyn Source, dest: &mut dyn Write) -> ConversionResult<()> {
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

pub fn csv_to_xlsx(source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    todo!();
}
