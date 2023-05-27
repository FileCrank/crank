use crate::error::ConversionResult;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use std::io::{BufRead, Write};
use std::ops::Deref;
use std::ptr::NonNull;

macro_rules! parse_md {
    ($source: expr, $arena: expr) => {{
        let mut str_buf = String::new();
        $source.read_to_string(&mut str_buf)?;
        parse_document(&$arena, str_buf.as_str(), &ComrakOptions::default())
    }};
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, sink: &mut dyn Write, f: &F) -> ConversionResult<()>
where
    F: Fn(&'a AstNode<'a>, &mut dyn Write) -> ConversionResult<()>,
{
    f(node, sink)?;
    for c in node.children() {
        iter_nodes(c, sink, f)?;
    }
    Ok(())
}

pub fn parse_md_fn(source: &mut dyn BufRead) -> ConversionResult<()> {
    let arena: Arena<AstNode> = Arena::new();
    let mut str_buf = String::new();
    source.read_to_string(&mut str_buf)?;
    parse_document(&arena, str_buf.as_str(), &ComrakOptions::default());
    Ok(())
}

pub fn md_to_txt(source: &mut dyn BufRead, sink: &mut dyn Write) -> ConversionResult<()> {
    let arena = Arena::new();
    let root = parse_md!(source, arena);
    // TODO: figure out a way to use sink without the indirection
    iter_nodes(root, sink, &|node, sink_ref| {
        match node.data.borrow_mut().value {
            NodeValue::Text(ref mut text) => {
                let mut text_bytes = text.as_bytes();
                sink_ref.write(text_bytes)?;
            }
            NodeValue::LineBreak | NodeValue::Paragraph => {
                sink_ref.write("\n".as_bytes())?;
            }
            NodeValue::Code(ref mut cell) => {
                sink_ref.write(cell.literal.as_bytes())?;
            }
            _ => {}
        };
        Ok(())
    })?;
    Ok(())
}