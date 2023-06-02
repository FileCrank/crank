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
    pub sink: &'a mut dyn Write,
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
        let mut styled_str = str;

        for (style_attr, style_marker) in vec![
            (styling.bold, "**"),
            (styling.italic, "*"),
            (styling.strike, "~"),
            (styling.code, "`"),
        ] {
            if style_attr {
                let format_str = format!("{}{}{}", style_marker, styled_str, style_marker);
                styled_str = format_str.as_str();
            }
        }

        self.sink.write(styled_str.as_bytes())?;
        Ok(())
    }

    pub fn write_line(&self) -> ConversionResult<()> {
        self.sink.write(b"\n")?;
        Ok(())
    }

    pub fn write_tab(&self) -> ConversionResult<()> {
        self.sink.write(b"\t")?;
        Ok(())
    }

    pub fn write_heading(order: MarkdownHeadingOrder) {
        todo!("Support writing markdown headings!")
    }
}
