mod conversions;
pub mod data;
pub mod error;
pub mod format;

use crate::conversions::graph;
use crate::data::{DataSink, DataSource};
use crate::error::ConversionResult;
use crate::format::Format;
use petgraph::algo::dijkstra;
use std::fs::File;
use std::io::{BufRead, Write};

pub struct Opts {
    pub source_format: Format,
    pub dest_format: Format,
}

pub fn convert<S: DataSource, D: DataSink>(
    opts: Opts,
    source: &mut S,
    dest: &mut D,
) -> ConversionResult<()> {
    let (format_indices, built_graph) = graph();
    let source_index = format_indices.get(&opts.source_format)?;
    let dest_index = format_indices.get(&opts.dest_format)?;

    // TODO: edge cost
    let conversion_path = dijkstra(built_graph, *source_index, Some(*dest_index), || 1);

    Ok(())
}
