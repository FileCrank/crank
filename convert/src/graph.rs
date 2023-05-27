use crate::conversions::identity::identity_conversion;
use crate::conversions::md::md_to_txt;
use crate::error::ConversionResult;
use crate::format::Format;
use comrak::{parse_document, Arena, ComrakOptions};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{copy, BufRead, Read, Write};

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

pub(super) fn build_graph() -> (HashMap<Format, NodeIndex>, Graph<Format, ConversionFn>) {
    // TODO: figure out a better way for this type to work
    let mut format_indices = HashMap::new();
    let mut g: Graph<Format, ConversionFn> = Graph::new();

    let txt = g.add_node(Format::Txt);
    format_indices.insert(Format::Txt, txt);

    let md = g.add_node(Format::Md);
    format_indices.insert(Format::Md, md);

    g.add_edge(md, txt, md_to_txt);
    g.add_edge(txt, md, identity_conversion);

    (format_indices, g)
}
