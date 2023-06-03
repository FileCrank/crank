use crate::error::ConversionResult;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use std::io::{BufRead, Write};

macro_rules! parse_md {
    ($source: expr, $arena: expr) => {{
        let mut str_buf = String::new();
        $source.read_to_string(&mut str_buf)?;
        parse_document(&$arena, str_buf.as_str(), &ComrakOptions::default())
    }};
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F) -> ConversionResult<()>
where
    F: FnMut(&'a AstNode<'a>) -> ConversionResult<()>,
{
    f(node)?;
    for c in node.children() {
        iter_nodes(c, f)?;
    }
    Ok(())
}

pub fn parse_md_fn(source: &mut dyn BufRead) -> ConversionResult<()> {
    let arena: Arena<AstNode> = Arena::new();
    parse_md!(source, arena);
    Ok(())
}

pub fn md_to_txt(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    let arena = Arena::new();
    let root = parse_md!(source, arena);
    // TODO: figure out a way to use sink without the indirection
    iter_nodes(root, &mut |node| {
        match node.data.borrow_mut().value {
            NodeValue::Text(ref mut text) => {
                let mut text_bytes = text.as_bytes();
                sink.write(text_bytes)?;
            }
            NodeValue::LineBreak | NodeValue::Paragraph => {
                sink.write("\n".as_bytes())?;
            }
            NodeValue::Code(ref mut cell) => {
                sink.write(cell.literal.as_bytes())?;
            }
            _ => {}
        };
        Ok(())
    })?;
    Ok(())
}
