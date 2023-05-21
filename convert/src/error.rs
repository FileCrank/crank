use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Format not found in graph")]
    FormatNotFoundError,
    #[error("No path found")]
    NoPathFoundError,
}

pub type ConversionResult<R> = Result<R, ConversionError>;
