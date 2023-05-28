use crate::error::ConversionResult;
use docx_rs::{read_docx, Docx};
use std::io::{BufRead, Read, Write};

pub fn docx_to_md(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    let mut buf: Vec<u8> = Vec::new();
    source.read_to_end(&mut buf)?;
    let docx = read_docx(buf.as_slice())?;
    Ok(())
}

#[cfg(test)]
mod test_docx {
    use crate::conversions::docx::docx_to_md;
    use crate::{convert, Opts};
    use std::fs::File;
    use std::io::BufReader;
    use std::str;

    #[test]
    pub fn test_docx_to_md() {
        let mut basic = BufReader::new(File::open("tests/test_files/basic.docx").unwrap());
        let mut dest: Vec<u8> = Vec::new();
        docx_to_md(&mut basic, &mut dest).unwrap();
        let dest_str = str::from_utf8(&dest.as_slice()).unwrap();
        assert_eq!(
            dest_str,
            "# Word Doc\n- A\n- B\n- C\n- D\n\n1. **Ordered**\n2. *list*"
        )
    }
}
