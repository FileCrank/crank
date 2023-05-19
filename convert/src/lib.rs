mod conversions;
pub mod data;
pub mod error;
pub mod format;

use crate::conversions::graph;
use crate::data::{DataSink, DataSource};
use crate::error::ConversionResult;
use std::fs::File;
use std::io::{BufRead, Write};

pub struct Opts {}

pub fn convert<S: DataSource, D: DataSink>(
    opts: Opts,
    source: &mut S,
    dest: &mut D,
) -> ConversionResult<()> {
    let graph = graph();

    Ok(())
}
