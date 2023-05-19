use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {}

pub type ConversionResult<R> = Result<R, ConversionError>