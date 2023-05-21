use convert::format::Format;
use convert::{convert, Opts};
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

#[test]
fn test_txt_to_rtf() {
    let mut basic = BufReader::new(File::open("tests/test_files/basic.txt").unwrap());
    let mut dest: Vec<u8> = Vec::new();

    convert(
        Opts {
            source_format: Format::TXT,
            dest_format: Format::RTF,
        },
        &mut basic,
        &mut dest,
    )
    .unwrap();

    let dest_str = str::from_utf8(&dest.as_slice()).unwrap();
    assert_eq!(dest_str, "Hello, world! 🤠")
}
