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
}

pub type ConversionResult<R> = Result<R, ConversionError>;
