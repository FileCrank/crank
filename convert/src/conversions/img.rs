use crate::error::ConversionResult;
use crate::format::ConversionFn;
use image::io::Reader;
use image::{ImageEncoder, ImageFormat, ImageOutputFormat};
use std::io::{BufRead, Cursor, Read, Write};

// TODO: once I make ConversionFn a Fn trait instead of a fn pointer, all of the image conversions can be one function which returns a closure
#[inline]
fn image_to_image_inner(
    source: &mut dyn BufRead,
    dest: &mut dyn Write,
    source_format: ImageFormat,
    dest_format: ImageFormat,
) -> ConversionResult<()> {
    // TODO: ideally we'd do this in place, without the read/write buf, but for now we just want to get everything hooked up

    let mut read_buf: Vec<u8> = Vec::new();
    source.read_to_end(&mut read_buf)?;
    let img = Reader::with_format(Cursor::new(read_buf), source_format).decode()?;

    let mut write_buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    img.write_to(&mut write_buf, dest_format)?;
    dest.write_all(write_buf.into_inner().as_slice())?;
    Ok(())
}

pub fn jpg_to_png(source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    image_to_image_inner(source, dest, ImageFormat::Jpeg, ImageFormat::Png)
}

pub fn png_to_jpg(source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    image_to_image_inner(source, dest, ImageFormat::Png, ImageFormat::Jpeg)
}
