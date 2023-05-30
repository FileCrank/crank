use crate::conversions::identity::identity_conversion;
use crate::conversions::md::md_to_txt;
use crate::error::ConversionResult;
use lazy_static::lazy_static;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::fmt::{format, Debug, Formatter, Octal};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Read, Write};
use std::ops::Deref;
use std::str::FromStr;

pub type ChunkFn<'a, T> = dyn Fn(&'a T) -> ConversionResult<()>;

// TODO: I think this reader interface is promising, but need to figure out how to incorporate the dynamic ChunkType without breaking
pub trait ConversionFormat {
    type ChunkType;

    fn read(source: &mut dyn BufRead, recv: &ChunkFn<Self::ChunkType>) -> ConversionResult<()>;
}

pub type ConversionFn = fn(&mut dyn BufRead, &mut dyn Write) -> ConversionResult<()>;

#[derive(Debug)]
pub struct ConversionQuality {}

pub struct Conversion {
    // TODO: this will probably eventually be a struct that has a whole bunch of properties, like streamability, structure, lossiness, etc. For the moment, though, we haven't even implemented a cost function, so it doesn't matter
    pub quality: ConversionQuality,
    pub executor: ConversionFn,
}

impl Debug for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversion")
            .field("quality", &self.quality)
            .finish()
    }
}

#[derive(Debug)]
pub struct Format {
    /// The unique identifier for the format, for ex. "txt"
    pub code: &'static str,
    pub conversions: HashMap<&'static Format, Conversion>,
}

impl PartialEq<Self> for Format {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for Format {}

impl Hash for Format {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state)
    }
}

fn initialize_formats<'a>() -> Vec<Format> {
    let mut formats = Vec::new();

    // TODO: write a fun lil macro for defining this more succinctly
    let mut txt_conversions: HashMap<&Format, Conversion> = HashMap::new();
    let txt_format = Format {
        code: "txt",
        conversions: txt_conversions,
    };
    formats.push(txt_format);

    let mut md_conversions: HashMap<&Format, Conversion> = HashMap::new();
    let md_format = Format {
        code: "md",
        conversions: md_conversions,
    };

    &txt_conversions.insert(
        &md_format,
        Conversion {
            quality: ConversionQuality {},
            executor: identity_conversion,
        },
    );
    &md_conversions.insert(
        &txt_format,
        Conversion {
            quality: ConversionQuality {},
            executor: md_to_txt,
        },
    );

    formats.push(md_format);
    formats
}

pub fn build_graph(
    formats: Vec<Format>,
) -> (
    HashMap<&'static Format, NodeIndex>,
    Graph<&'static Format, &'static Conversion>,
) {
    let mut format_indices: HashMap<&'static Format, NodeIndex> = HashMap::new();
    let mut graph: Graph<&'static Format, &'static Conversion> = Graph::new();

    // We have to do two passes - one to put all the format references as nodes in the graph, and a second to connect them
    for format in formats {
        let format_node_index = graph.add_node(&format);
        format_indices.insert(&format, format_node_index);
    }

    for src_format in formats {
        for (dest_format, conversion) in src_format.conversions {
            // These calls can only fail if we link a format struct that's not provided in the
            // final vec, and it would be caught immediately by any of the integration tests, so
            // these are safe in practice
            let source_index = format_indices.get(&src_format).expect(&format!(
                "Format {} missing from FORMATS vec",
                src_format.code
            ));
            let dest_index = format_indices.get(dest_format).expect(&format!(
                "Format {} missing from FORMATS vec",
                dest_format.code
            ));

            graph.add_edge(*source_index, *dest_index, &conversion);
        }
    }

    (format_indices, graph)
}

lazy_static! {
    pub static ref FORMATS: Vec<Format> = initialize_formats();
    pub static ref FORMAT_DATA: (
        HashMap<&'static Format, NodeIndex>,
        Graph<&'static Format, &'static Conversion>,
    ) = build_graph(*FORMATS.deref());
}
