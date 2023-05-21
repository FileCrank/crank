use crate::data::{DataSink, DataSource};
use crate::error::ConversionResult;
use crate::format::Format;
use petgraph::graph::NodeIndex;
use petgraph::visit::Data;
use petgraph::Graph;
use phf::phf_map;
use std::collections::HashMap;
use std::fmt::format;
use std::io::{copy, Read, Write};

/// TODO: the edges should be a struct which contain conversionFn, but also have information about the *quality* of the conversion

pub type ConversionFn<S: DataSource, D: DataSink> = fn(&mut S, &mut D) -> ConversionResult<()>;

pub struct ConversionWeight {
    pub quality: u8,
}

pub struct ConversionEdge<S, D> {
    conversion: ConversionFn<S, D>,
    weight: ConversionWeight,
}

fn identity_conversion<S: DataSource, D: DataSink>(
    source: &mut S,
    sink: &mut D,
) -> ConversionResult<()> {
    copy(source, sink)?;
    Ok(())
}

pub(super) fn graph<S: DataSource, D: DataSink>() -> (
    HashMap<Format, NodeIndex>,
    Graph<Format, ConversionFn<S, D>>,
) {
    // TODO: figure out a better way for this type to work
    let mut format_indices = HashMap::new();
    let mut g = Graph::new();

    let txt = g.add_node(Format::TXT);
    format_indices.insert(Format::TXT, txt);

    let rtf = g.add_node(Format::RTF);
    format_indices.insert(Format::RTF, rtf);

    g.add_edge(txt, rtf, identity_conversion);

    (format_indices, g)
}
