use crate::conversions::identity::identity_conversion;
use crate::conversions::md::md_to_txt;
use crate::error::ConversionResult;
use crate::format::{ConversionFn, Format};
use comrak::{parse_document, Arena, ComrakOptions};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::{copy, BufRead, Read, Write};

/// TODO: the edges should be a struct which contain conversionFn, but also have information about the *quality* of the conversion

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
