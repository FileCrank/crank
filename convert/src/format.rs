use crate::conversions::identity::identity_conversion;
use crate::conversions::md::md_to_txt;
use crate::error::ConversionResult;
use lazy_static::lazy_static;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::fmt::{format, Debug, Display, Formatter, Octal};
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

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code)
    }
}

macro_rules! conversion_format {
    ($code: literal) => {
        Format { code: $code }
    };
}

pub const TXT: Format = conversion_format!("txt");
pub const MD: Format = conversion_format!("md");
pub const DOCX: Format = conversion_format!("docx");

macro_rules! add_node {
    ($format: expr, $graph: expr, $indices: expr) => {{
        let node = $graph.add_node($format);
        $indices.insert($format, node);
        node
    }};
}

pub fn build_graph() -> (
    HashMap<&'static Format, NodeIndex>,
    Graph<&'static Format, Conversion>,
) {
    let mut graph: Graph<&'static Format, Conversion> = Graph::new();
    let mut format_indices: HashMap<&'static Format, NodeIndex> = HashMap::new();

    let txt = add_node!(&TXT, graph, format_indices);
    let md = add_node!(&MD, graph, format_indices);

    graph.add_edge(
        txt,
        md,
        Conversion {
            quality: ConversionQuality {},
            executor: identity_conversion,
        },
    );

    graph.add_edge(
        md,
        txt,
        Conversion {
            quality: ConversionQuality {},
            executor: md_to_txt,
        },
    );

    (format_indices, graph)
}

lazy_static! {
    pub static ref FORMAT_DATA: (
        HashMap<&'static Format, NodeIndex>,
        Graph<&'static Format, Conversion>,
    ) = build_graph();
}
