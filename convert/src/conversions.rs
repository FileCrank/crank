use crate::error::ConversionResult;
use crate::format::Format;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{copy, BufRead, Write};

/// TODO: the edges should be a struct which contain conversionFn, but also have information about the *quality* of the conversion

pub type ConversionFn = fn(&mut dyn BufRead, &mut dyn Write) -> ConversionResult<()>;

pub struct ConversionWeight {
    pub quality: u8,
}

pub struct ConversionEdge {
    conversion: ConversionFn,
    weight: ConversionWeight,
}

pub struct Conversion {
    quality: ConversionWeight,
    executor: ConversionFn,
}

fn identity_conversion(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    copy(source, sink)?;
    Ok(())
}

pub(super) fn build_graph() -> (HashMap<Format, NodeIndex>, Graph<Format, ConversionFn>) {
    // TODO: figure out a better way for this type to work
    let mut format_indices = HashMap::new();
    let mut g: Graph<Format, ConversionFn> = Graph::new();

    let txt = g.add_node(Format::TXT);
    format_indices.insert(Format::TXT, txt);

    let rtf = g.add_node(Format::RTF);
    format_indices.insert(Format::RTF, rtf);

    g.add_edge(txt, rtf, identity_conversion);

    (format_indices, g)
}
