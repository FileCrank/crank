use crate::error::ConversionResult;
use crate::writers::md::{MarkdownStyingContext, MarkdownWriter};
use docx_rs::{read_docx, DocumentChild, Paragraph, ParagraphChild, Run, RunChild, RunProperty};
use std::io::{BufRead, Write};

fn apply_run_property(ctx: &mut MarkdownStyingContext, property: &RunProperty) {
    if let Some(_) = property.bold {
        ctx.bold = true;
    }

    if let Some(_) = property.italic {
        ctx.italic = true;
    }

    if let Some(_) = property.strike {
        ctx.strike = true;
    }
}

fn run_child_to_md(
    ctx: MarkdownStyingContext,
    run_child: &RunChild,
    writer: &mut MarkdownWriter,
) -> ConversionResult<()> {
    match run_child {
        RunChild::Text(text) => {
            // TODO: handle preserve_space here?
            let text_str = text.text.as_str();
            writer.write_with_styling(ctx, text_str)
        }
        RunChild::Tab(_) => {
            // There are a bunch of different Tab types implemented in docx, but at least for now
            // we'll be dumb and just write a literal \t
            writer.write_tab()
        }
        RunChild::Break(_) => {
            // Same story with break as with tab - just write a \n
            writer.write_line()
        }
        _ => {
            todo!("Handle more RunChild types")
        }
    }
}

fn run_to_md(
    mut ctx: MarkdownStyingContext,
    run: &Run,
    writer: &mut MarkdownWriter,
) -> ConversionResult<()> {
    apply_run_property(&mut ctx, &run.run_property);

    for child in &run.children {
        run_child_to_md(ctx, child, writer)?;
    }

    Ok(())
}

fn paragraph_child_to_md(
    ctx: MarkdownStyingContext,
    child: &ParagraphChild,
    writer: &mut MarkdownWriter,
) -> ConversionResult<()> {
    match child {
        ParagraphChild::Run(run) => run_to_md(ctx, run, writer)?,
        // Types that come up in documents, but aren't yet handled
        ParagraphChild::BookmarkStart(_) | ParagraphChild::BookmarkEnd(_) => {}
        ParagraphChild::Insert(_) => todo!("Handle insert"),
        ParagraphChild::Delete(_) => todo!("Handle delete"),
        ParagraphChild::Hyperlink(_) => todo!("Handle hyperlink"),
        ParagraphChild::CommentStart(_) => todo!("Handle comment start"),
        ParagraphChild::CommentEnd(_) => todo!("Handle comment end"),
        ParagraphChild::StructuredDataTag(_) => todo!("Handle structured data tag"),
    };
    Ok(())
}

fn paragraph_to_md(
    mut ctx: MarkdownStyingContext,
    paragraph: &Paragraph,
    writer: &mut MarkdownWriter,
) -> ConversionResult<()> {
    apply_run_property(&mut ctx, &paragraph.property.run_property);

    if let Some(numbering_property) = &paragraph.property.numbering_property {
        if let Some(numbering_level) = &numbering_property.level {
            writer.write_list_item(numbering_level.val)?;
        }
    }

    for child in &paragraph.children {
        paragraph_child_to_md(ctx, child, writer)?;
    }

    Ok(())
}

fn document_child_to_md(
    ctx: MarkdownStyingContext,
    child: &DocumentChild,
    writer: &mut MarkdownWriter,
) -> ConversionResult<()> {
    match child {
        DocumentChild::Paragraph(ref par) => paragraph_to_md(ctx, par, writer)?,
        _ => todo!("Impl other document children"),
    };
    Ok(())
}

pub fn docx_to_md(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    let mut buf: Vec<u8> = Vec::new();
    source.read_to_end(&mut buf)?;

    let docx = read_docx(buf.as_slice())?;

    let ctx = MarkdownStyingContext::default();
    let mut writer = MarkdownWriter::new(sink);

    for child in docx.document.children {
        document_child_to_md(ctx, &child, &mut writer)?;
    }

    Ok(())
}

fn run_to_txt(run: &Run, sink: &mut dyn Write) -> ConversionResult<()> {
    for child in &run.children {
        match child {
            RunChild::Text(text) => {
                // TODO: handle preserve_space here?
                sink.write(text.text.as_bytes())?;
            }
            RunChild::Tab(_) => {
                sink.write(b"\t")?;
            }
            RunChild::Break(_) => {
                sink.write(b"\n")?;
            }
            _ => {
                todo!("Handle more RunChild types")
            }
        }
    }
    Ok(())
}

fn paragraph_to_txt(paragraph: &Paragraph, sink: &mut dyn Write) -> ConversionResult<()> {
    for child in &paragraph.children {
        match child {
            ParagraphChild::Run(run) => run_to_txt(run, sink)?,
            _ => {}
        };
    }
    Ok(())
}

fn document_child_to_txt(child: &DocumentChild, sink: &mut dyn Write) -> ConversionResult<()> {
    match child {
        // For converting to plaintext we only care about paragraphs
        DocumentChild::Paragraph(ref par) => paragraph_to_txt(par, sink)?,
        _ => {}
    };

    Ok(())
}

pub fn docx_to_txt(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    let mut buf: Vec<u8> = Vec::new();
    source.read_to_end(&mut buf)?;
    let docx = read_docx(buf.as_slice())?;

    for child in docx.document.children {
        document_child_to_txt(&child, sink)?;
    }

    Ok(())
}

#[cfg(test)]
mod test_docx {
    use crate::conversions::docx::{docx_to_md, docx_to_txt};
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

    #[test]
    pub fn test_docx_to_txt() {
        let mut basic = BufReader::new(File::open("tests/test_files/basic.docx").unwrap());
        let mut dest: Vec<u8> = Vec::new();
        docx_to_txt(&mut basic, &mut dest).unwrap();
        let dest_str = str::from_utf8(&dest.as_slice()).unwrap();
        assert_eq!(dest_str, " Word Doc\nA\nB\nC\nOrdered\nlist")
    }
}
