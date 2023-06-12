use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Format not found in graph")]
    FormatNotFoundError,
    #[error("No path found")]
    PathNotFoundError,
    #[error("Empty Path")]
    EmptyPathError,
    #[error("No conversion found")]
    ConversionNotFoundError,
    #[error(transparent)]
    DocxReaderError(#[from] docx_rs::ReaderError),
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}

impl Serialize for ConversionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ConversionResult<R> = Result<R, ConversionError>;
