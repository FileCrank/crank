pub mod error;
pub mod format;

use std::fs::File;
use std::io::{BufRead, Write};

pub trait DataSource: BufRead {}

pub trait DataSink: Write {}

pub struct Opts {

}


pub fn convert<S: DataSource, D: DataSink>(opts: Opts,
                                           source: &mut S,
                                           dest: &mut D) {

}