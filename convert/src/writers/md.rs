use crate::error::ConversionResult;
use std::io::Write;

pub enum MarkdownHeadingOrder {
    Heading1 = 1,
    Heading2 = 2,
    Heading3 = 3,
}

#[derive(Copy, Clone, Debug)]
pub struct MarkdownStyingContext {
    pub bold: bool,
    pub italic: bool,
    pub strike: bool,
    pub code: bool,
}

impl Default for MarkdownStyingContext {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            strike: false,
            code: false,
        }
    }
}

pub struct MarkdownWriter<'a> {
    sink: &'a mut dyn Write,
}
impl<'a> MarkdownWriter<'a> {
    pub fn new(sink: &'a mut dyn Write) -> Self {
        Self { sink }
    }

    pub fn write_with_styling(
        &mut self,
        styling: MarkdownStyingContext,
        str: &str,
    ) -> ConversionResult<()> {
        // TODO: do this in a smarter way
        let mut styled_str: String = str.to_string();

        for (style_attr, style_marker) in vec![
            (styling.bold, "**"),
            (styling.italic, "*"),
            (styling.strike, "~"),
            (styling.code, "`"),
        ] {
            if style_attr {
                let mut str_buf = String::new();
                str_buf.push_str(style_marker);
                str_buf.push_str(&styled_str);
                str_buf.push_str(style_marker);
                styled_str = str_buf;
            }
        }

        self.sink.write(styled_str.as_bytes())?;
        Ok(())
    }

    pub fn write_line(&mut self) -> ConversionResult<()> {
        self.sink.write(b"\n")?;
        Ok(())
    }

    pub fn write_tab(&mut self) -> ConversionResult<()> {
        self.sink.write(b"\t")?;
        Ok(())
    }

    pub fn write_list_item(&mut self, indentation_level: usize) -> ConversionResult<()> {
        for _ in 1..indentation_level {
            self.sink.write(b"\t")?;
        }
        self.sink.write(b"- ")?;
        Ok(())
    }

    pub fn write_heading(order: MarkdownHeadingOrder) {
        todo!("Support writing markdown headings!")
    }
}
