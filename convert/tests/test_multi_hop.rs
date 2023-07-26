use convert::convert;
use convert::format::{Format, CSV, DOCX};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::str;

#[test]
fn test_csv_to_docx() {
    let mut basic = File::open("tests/test_files/basic.csv").unwrap();
    let mut dest: Vec<u8> = Vec::new();

    convert(&CSV, &DOCX, &mut basic, &mut dest).unwrap();

    let dest_str = str::from_utf8(&dest.as_slice()).unwrap();
    assert_eq!(dest_str, "A1 B1\nA2 B2")
}
