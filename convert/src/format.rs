use crate::conversions::docx::docx_to_md;
use crate::conversions::identity::identity_conversion;
use crate::conversions::md::md_to_txt;
use crate::conversions::txt::txt_to_docx;
use crate::error::ConversionResult;
use crate::{for_all_pairs, image_to_image};
use comrak::nodes::NodeValue::Image;
use image::ImageFormat;
use lazy_static::lazy_static;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Write};
use std::sync::Arc;

pub type ChunkFn<'a, T> = dyn Fn(&'a T) -> ConversionResult<()>;

// TODO: I think this reader interface is promising, but need to figure out how to incorporate the dynamic ChunkType without breaking
pub trait ConversionFormat {
    type ChunkType;

    fn read(source: &mut dyn BufRead, recv: &ChunkFn<Self::ChunkType>) -> ConversionResult<()>;
}

pub type ConversionFn =
    dyn Fn(&mut dyn BufRead, &mut dyn Write) -> ConversionResult<()> + Send + Sync;

#[derive(Debug)]
pub struct ConversionQuality {}

pub struct Conversion {
    // TODO: this will probably eventually be a struct that has a whole bunch of properties, like streamability, structure, lossiness, etc. For the moment, though, we haven't even implemented a cost function, so it doesn't matter
    pub quality: ConversionQuality,
    pub executor: Box<ConversionFn>,
}

impl Debug for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conversion")
            .field("quality", &self.quality)
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format<'a> {
    /// The unique identifier for the format, for ex. "txt"
    pub code: &'a str,
    pub name: &'a str,
}

impl PartialEq<Self> for Format<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Eq for Format<'_> {}

impl Hash for Format<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state)
    }
}

impl Display for Format<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.code)
    }
}

macro_rules! conversion_format {
    ($code: literal, $name: literal) => {
        Format {
            code: $code,
            name: $name,
        }
    };
}

// Document formats
pub const TXT: Format = conversion_format!("txt", "Text");
pub const MD: Format = conversion_format!("md", "Markdown");
pub const DOCX: Format = conversion_format!("docx", "Word Document");

// Image formats
pub const JPG: Format = conversion_format!("jpg", "JPG Image");
pub const PNG: Format = conversion_format!("png", "PNG Image");
pub const GIF: Format = conversion_format!("gif", "Animated GIF");
pub const WEBP: Format = conversion_format!("webp", "WebP Image");
pub const PNM: Format = conversion_format!("pnm", "PNM Image");
pub const TIFF: Format = conversion_format!("tiff", "TIFF Image");
pub const TGA: Format = conversion_format!("tga", "TGA Image");
pub const DDS: Format = conversion_format!("dds", "DDS Image");
pub const BMP: Format = conversion_format!("bmp", "BMP Image");
pub const ICO: Format = conversion_format!("ico", "ICO Image");
pub const HDR: Format = conversion_format!("hdr", "HDR image");

macro_rules! add_node {
    ($format: expr, $graph: ident, $indices: ident) => {{
        let node = $graph.add_node($format);
        $indices.insert($format, node);
        node
    }};
}

macro_rules! add_image_conversion {
    ($graph: ident, $from_format: expr, $to_format: expr) => {
        $graph.add_edge(
            $from_format.0,
            $to_format.0,
            Conversion {
                quality: ConversionQuality {},
                executor: image_to_image!($from_format.1, $to_format.1),
            },
        )
    };
}

pub fn build_graph() -> (
    HashMap<&'static Format<'static>, NodeIndex>,
    Graph<&'static Format<'static>, Conversion>,
) {
    let mut graph: Graph<&'static Format, Conversion> = Graph::new();
    let mut format_indices: HashMap<&'static Format, NodeIndex> = HashMap::new();

    // Document formats
    let txt = add_node!(&TXT, graph, format_indices);
    let md = add_node!(&MD, graph, format_indices);
    let docx = add_node!(&DOCX, graph, format_indices);

    graph.add_edge(
        txt,
        md,
        Conversion {
            quality: ConversionQuality {},
            executor: Box::new(identity_conversion),
        },
    );

    graph.add_edge(
        md,
        txt,
        Conversion {
            quality: ConversionQuality {},
            executor: Box::new(md_to_txt),
        },
    );

    graph.add_edge(
        docx,
        md,
        Conversion {
            quality: ConversionQuality {},
            executor: Box::new(docx_to_md),
        },
    );

    graph.add_edge(
        txt,
        docx,
        Conversion {
            quality: ConversionQuality {},
            executor: Box::new(txt_to_docx),
        },
    );

    // Image formats
    let jpg = (add_node!(&JPG, graph, format_indices), ImageFormat::Jpeg);
    let png = (add_node!(&PNG, graph, format_indices), ImageFormat::Png);
    let gif = (add_node!(&GIF, graph, format_indices), ImageFormat::Gif);
    let webp = (add_node!(&WEBP, graph, format_indices), ImageFormat::WebP);
    let pnm = (add_node!(&PNM, graph, format_indices), ImageFormat::Pnm);
    let tiff = (add_node!(&TIFF, graph, format_indices), ImageFormat::Tiff);
    let tga = (add_node!(&TGA, graph, format_indices), ImageFormat::Tga);
    let dds = (add_node!(&DDS, graph, format_indices), ImageFormat::Dds);
    let bmp = (add_node!(&BMP, graph, format_indices), ImageFormat::Bmp);
    let ico = (add_node!(&ICO, graph, format_indices), ImageFormat::Ico);
    let hdr = (add_node!(&HDR, graph, format_indices), ImageFormat::Hdr);

    // This fun little macro figures out all of the combinations of image formats at compile time,
    // and writes the code to add them all to the graph. Eventually it'd be nice to define the
    // mappings between our formats and ImageFormat variants inline as well
    for_all_pairs!(
        add_image_conversion,
        graph: jpg png gif webp pnm tiff tga dds bmp ico hdr
    );

    (format_indices, graph)
}

lazy_static! {
    pub static ref FORMAT_DATA: (
        HashMap<&'static Format<'static>, NodeIndex>,
        Graph<&'static Format<'static>, Conversion>,
    ) = build_graph();
    pub static ref FORMATS_BY_CODE: HashMap<&'static str, &'static Format<'static>> = {
        let mut map: HashMap<&'static str, &'static Format<'static>> = HashMap::new();
        for (key, _) in &FORMAT_DATA.deref().0 {
            map.insert(key.code.into(), key);
        }
        map
    };
}
