use crate::error::ConversionResult;
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader;
use std::io::{BufRead, Write};

pub fn jpg_to_png(source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    let img = Reader::new(source);
    let mut encoder = JpegEncoder::new(dest);
    Ok(())
}
