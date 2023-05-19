use crate::data::{DataSink, DataSource};
use crate::error::ConversionResult;
use crate::format::Format;
use petgraph::visit::Data;
use petgraph::Graph;
use phf::phf_map;
use std::io::{Read, Write};

/// TODO: the edges should be a struct which contain conversionFn, but also have information about the *quality* of the conversion

pub type ConversionFn<S: DataSource, D: DataSink> = fn(&mut S, &mut D) -> ConversionResult<()>;

fn identity_conversion<S: DataSource, D: DataSink>(
    source: &mut S,
    sink: &mut D,
) -> ConversionResult<()> {
    source.read(sink)?;
    Ok(())
}

pub(super) fn graph<S: DataSource, D: DataSink>() -> Graph<Format, ConversionFn<S, D>> {
    let mut g = Graph::new();

    let json = g.add_node(Format::TXT);
    let rtf = g.add_node(Format::RTF);

    g.add_edge(json, rtf, identity_conversion);

    g
}
