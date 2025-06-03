// =======================================================================
// metadata.rs
// =======================================================================
// Extra metadata attached to leaf nodes to encode a CST

use crate::*;
use core::ops::Range;

pub type Span = Range<usize>;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Metadata<'a> {
    pub span: Span,
    pub extra_nodes: Vec<(ExtraNode<'a>, Span)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraNode<'a> {
    OnelineComment(&'a str),
    BlockComment(&'a str),
    Directive(CompilerDirective),
    Newline,
}
