use crate::conversions::txt::TxtFormat;
use crate::error::ConversionResult;
use std::io::{BufRead, Read};

pub type ChunkFn<'a, T> = dyn Fn(&'a T) -> ConversionResult<()>;

pub trait ConversionFormat<T> {
    fn read(source: &mut dyn BufRead, recv: &ChunkFn<T>) -> ConversionResult<()>;
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Format {
    Txt(TxtFormat),
    Md,
    Docx,
}
