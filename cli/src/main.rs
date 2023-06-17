use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use clap::error::ErrorKind::Format;
use clap::Parser;
use convert::{convert, format::{JPG, PNG}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf
}

fn main() {
    let args = Args::parse();
    let mut in_file = File::open(args.input).expect(
        "Couldn't open input file"
    );
    let mut in_reader = BufReader::new(in_file);

    let mut out_file = File::create(args.output).expect(
        "Couldn't open output file"
    );
    convert(
        &JPG,
        &PNG,
        &mut in_reader,
        &mut out_file,
    ).expect("Couldn't convert");
}
