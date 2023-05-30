mod conversions;
pub mod error;
pub mod format;

use crate::error::{ConversionError, ConversionResult};
use crate::format::{build_graph, Conversion, ConversionFn, Format, FORMAT_DATA};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Data, IntoEdges};
use petgraph::Graph;
use std::io::{copy, BufRead, BufReader, Cursor, Write};
use std::ops::Deref;

pub struct Opts {
    pub source_format: Format,
    pub dest_format: Format,
}

pub fn execute_path(
    graph: &Graph<&Format, &Conversion>,
    path: Vec<NodeIndex>,
    source: &mut dyn BufRead,
    dest: &mut dyn Write,
) -> ConversionResult<()> {
    let mut path_iter = path.iter();
    let mut curr_node = path_iter.next().ok_or(ConversionError::EmptyPathError)?;

    let mut src_inner: BufReader<Cursor<Vec<u8>>>;
    let mut src_buf: &mut dyn BufRead = source;
    let mut dest_buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    // TODO: ideally we wouldn't have to do this in memory, and would be smart about the one-hop case where we don't need an extra copy
    while let Some(next) = path_iter.next() {
        let conversion_idx = graph
            .find_edge(*curr_node, *next)
            .ok_or(ConversionError::ConversionNotFoundError)?;

        let conversion = graph
            .edge_weight(conversion_idx)
            .ok_or(ConversionError::ConversionNotFoundError)?;

        // Actually execute the conversion
        (conversion.executor)(src_buf, &mut dest_buf)?;

        // The destination is the new source, and we need to allocate a new destination - the
        // capacity of the old one is a good starting point
        let curr_capacity = dest_buf.get_ref().capacity();
        dest_buf.set_position(0);
        src_inner = BufReader::new(dest_buf);
        src_buf = &mut src_inner;
        dest_buf = Cursor::new(Vec::with_capacity(curr_capacity))
    }

    // TODO: it would be great if this could be expressed more elegantly - if we were smart enough to skip the state management on the last iteration, we could just copy from dest
    copy(&mut src_buf, dest)?;
    Ok(())
}

pub fn convert(opts: Opts, source: &mut dyn BufRead, dest: &mut dyn Write) -> ConversionResult<()> {
    let (format_indices, graph) = FORMAT_DATA.deref();

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
        None => Err(ConversionError::PathNotFoundError),
    }
}
