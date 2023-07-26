mod conversions;
pub mod error;
pub mod format;
mod macros;
pub(crate) mod writers;

use crate::error::{ConversionError, ConversionResult};
use crate::format::{Conversion, Format, Source, FORMAT_DATA};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::io::{copy, BufReader, Cursor, Read, Seek, Write};
use std::ops::Deref;

impl<T> Source for T where T: Read + Seek {}

pub fn execute_path(
    graph: &Graph<&Format, Conversion>,
    path: Vec<NodeIndex>,
    source: &mut dyn Source,
    dest: &mut dyn Write,
) -> ConversionResult<()> {
    let mut path_iter = path.iter();
    let mut curr_node = path_iter.next().ok_or(ConversionError::EmptyPathError)?;

    let mut src_buf: &mut dyn Source = source;
    let mut src_inner: Cursor<Vec<u8>>;
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
        // TODO: we should be able to get rid of src_inner here
        src_inner = dest_buf.clone();
        src_buf = &mut src_inner;
        dest_buf = Cursor::new(Vec::with_capacity(curr_capacity));

        curr_node = next;
    }

    // TODO: it would be great if this could be expressed more elegantly - if we were smart enough to skip the state management on the last iteration, we could just copy from dest
    copy(&mut src_buf, dest)?;
    Ok(())
}

pub fn convert(
    from: &Format,
    to: &Format,
    source: &mut dyn Source,
    dest: &mut dyn Write,
) -> ConversionResult<()> {
    let (format_indices, graph) = FORMAT_DATA.deref();

    let source_index = format_indices
        .get(from)
        .ok_or(ConversionError::FormatNotFoundError)?;

    let dest_index = format_indices
        .get(to)
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
