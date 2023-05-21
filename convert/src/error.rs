use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub type ConversionResult<R> = Result<R, ConversionError>;
