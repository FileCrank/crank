use crate::error::ConversionResult;
use crate::format::{ChunkFn, ConversionFormat};
use std::io::BufRead;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct TxtFormat {}

/// A chunk of a text file is a line of text
impl ConversionFormat<String> for TxtFormat {
    fn read(source: &mut dyn BufRead, recv: &ChunkFn<String>) -> ConversionResult<()> {
        loop {
            let mut buf = String::new();

            match source.read_line(&mut buf)? {
                // If we read 0 bytes, there's nothing left from the source
                0 => return Ok(()),
                _ => recv(&buf)?,
            };
        }
    }
}
