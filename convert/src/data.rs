use std::io::{BufRead, Write};

pub trait DataSource: BufRead {}

pub trait DataSink: Write {}
