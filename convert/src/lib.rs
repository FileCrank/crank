mod conversions;
pub mod data;
pub mod error;
pub mod format;

use crate::conversions::graph;
use crate::data::{DataSink, DataSource};
use crate::error::{ConversionError, ConversionResult};
use crate::format::Format;
use petgraph::algo::dijkstra;

pub struct Opts {
    pub source_format: Format,
    pub dest_format: Format,
}

pub fn convert<S: DataSource, D: DataSink>(
    opts: Opts,
    source: &mut S,
    dest: &mut D,
) -> ConversionResult<()> {
    let (format_indices, built_graph) = graph::<S, D>();
    let source_index = format_indices
        .get(&opts.source_format)
        .ok_or(ConversionError::FormatNotFoundError)?;
    let dest_index = format_indices
        .get(&opts.dest_format)
        .ok_or(ConversionError::FormatNotFoundError)?;

    // TODO: edge cost
    let conversion_costs = dijkstra(&built_graph, *source_index, Some(*dest_index), |_| 1);

    Ok(())
}
