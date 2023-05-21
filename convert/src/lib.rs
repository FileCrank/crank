mod conversions;
pub mod error;
pub mod format;

use crate::conversions::{build_graph, ConversionFn};
use crate::error::{ConversionError, ConversionResult};
use crate::format::Format;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Data, IntoEdges};
use petgraph::Graph;
use std::io::{BufRead, Write};

pub struct Opts {
    pub source_format: Format,
    pub dest_format: Format,
}

pub fn execute_path(
    graph: &Graph<Format, ConversionFn>,
    path: Vec<NodeIndex>,
    source: &mut dyn BufRead,
    dest: &mut dyn Write,
) -> ConversionResult<()> {
    Ok(())
}

pub fn convert(opts: Opts, source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    let (format_indices, graph) = build_graph();

    let source_index = format_indices
        .get(&opts.source_format)
        .ok_or(ConversionError::FormatNotFoundError)?;

    let dest_index = format_indices
        .get(&opts.dest_format)
        .ok_or(ConversionError::FormatNotFoundError)?;

    // TODO: edge cost, estimate cost
    match astar(
        &graph,
        *source_index,
        |node| node == *dest_index,
        |_| 1,
        |_| 0,
    ) {
        Some((_, path)) => execute_path(&graph, path, source, dest),
        None => Err(ConversionError::NoPathFoundError),
    }
}
