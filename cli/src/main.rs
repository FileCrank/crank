use std::arch::x86_64::_mm256_zeroupper;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use clap::Parser;
use convert::{convert, format::{JPG, PNG}};
use convert::format::{Format, FORMATS_BY_CODE};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(required(true), index(1))]
    input: PathBuf,

    #[arg(required(true), index(2))]
    output: PathBuf
}

fn parse_format_from_extension(path: &PathBuf) -> Option<&Format> {
    let extension = path.extension();
    match extension {
        Some(e) => {
            match FORMATS_BY_CODE.get(e.to_str().unwrap()) {
                Some(f) => Some(*f),
                _ => None
            }
        },
        _ => None
    }
}

fn main() {
    let args = Args::parse();

    // TODO: Let the user specify the file type if it doesn't have an extension
    let input_format = parse_format_from_extension(&args.input).expect("Input format unsupported");
    let output_format = parse_format_from_extension(&args.output).expect("Output format unsupported");

    let mut in_file = File::open(&args.input).expect(
        "Couldn't open input file"
    );
    let mut in_reader = BufReader::new(in_file);

    let mut out_file = File::create(&args.output).expect(
        "Couldn't open output file"
    );
    convert(
        input_format,
        output_format,
        &mut in_reader,
        &mut out_file,
    ).expect("Couldn't convert");
}
