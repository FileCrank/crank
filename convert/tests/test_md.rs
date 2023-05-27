use convert::format::Format;
use convert::{convert, Opts};
use std::fs::File;
use std::io::BufReader;
use std::str;

#[test]
fn test_md_to_txt() {
    let mut basic = BufReader::new(File::open("tests/test_files/basic.md").unwrap());
    let mut dest: Vec<u8> = Vec::new();

    convert(
        Opts {
            source_format: Format::Md,
            dest_format: Format::Txt,
        },
        &mut basic,
        &mut dest,
    )
    .unwrap();

    let dest_str = str::from_utf8(&dest.as_slice()).unwrap();
    assert_eq!(dest_str, "Markdown\nitalic\nbold\ncode")
}
