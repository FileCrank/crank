use crate::error::ConversionResult;
use crate::format::{ConversionFn, Source};
use image::io::Reader;
use image::{ImageEncoder, ImageFormat, ImageOutputFormat};
use std::io::{Cursor, Read, Write};

pub fn image_to_image_inner(
    source: &mut dyn Source,
    dest: &mut dyn Write,
    from: ImageFormat,
    to: ImageFormat,
) -> ConversionResult<()> {
    // TODO: ideally we'd do this in place, without the read/write buf, but for now we just want to get everything hooked up

    let mut read_buf: Vec<u8> = Vec::new();
    source.read_to_end(&mut read_buf)?;
    let img = Reader::with_format(Cursor::new(read_buf), from).decode()?;

    let mut write_buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    img.write_to(&mut write_buf, to)?;
    dest.write_all(write_buf.into_inner().as_slice())?;
    Ok(())
}

#[macro_export]
macro_rules! image_to_image {
    ($from: expr, $to: expr) => {
        Box::new(move |source: &mut dyn Source, dest: &mut dyn Write| {
            crate::conversions::img::image_to_image_inner(source, dest, $from, $to)
        })
    };
}
