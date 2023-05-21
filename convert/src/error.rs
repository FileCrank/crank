use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Format not found in graph")]
    FormatNotFoundError,
}

pub type ConversionResult<R> = Result<R, ConversionError>;
