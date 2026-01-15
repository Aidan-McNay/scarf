// =======================================================================
// metadata.rs
// =======================================================================
// Extra metadata attached to leaf nodes to encode a CST

use crate::*;
use core::ops::Range;

pub type ByteSpan = Range<usize>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub file: &'a str,
    pub bytes: ByteSpan,
    pub expanded_from: Option<&'a Span<'a>>,
    pub included_from: Option<&'a Span<'a>>,
}

#[derive(PartialEq)]
pub enum SpanRelation {
    Earlier,
    Later,
    Same,
}

impl<'a> Span<'a> {
    fn include_byte_indeces(&self) -> Vec<usize> {
        match &self.included_from {
            None => vec![self.bytes.start],
            Some(inner_span) => {
                let mut nested_byte_indeces = inner_span.include_byte_indeces();
                nested_byte_indeces.push(self.bytes.start);
                nested_byte_indeces
            }
        }
    }
    pub const fn empty() -> Span<'a> {
        Span {
            file: "",
            bytes: ByteSpan { start: 0, end: 0 },
            expanded_from: None,
            included_from: None,
        }
    }
    pub fn compare(&self, other: &Self) -> SpanRelation {
        let mut idx = 0;
        let self_byte_indeces = self.include_byte_indeces();
        let other_byte_indeces = other.include_byte_indeces();
        loop {
            match (self_byte_indeces.get(idx), other_byte_indeces.get(idx)) {
                (Some(self_idx), Some(other_idx)) => {
                    if self_idx < other_idx {
                        break SpanRelation::Earlier;
                    } else if self_idx > other_idx {
                        break SpanRelation::Later;
                    } else {
                        idx += 1;
                        continue;
                    }
                }
                (None, None) => {
                    break SpanRelation::Same;
                }
                _ => {
                    panic!(
                        "Internal error comparing spans {:?} and {:?}",
                        self, other
                    )
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
    pub extra_nodes: Vec<ExtraNode<'a>>,
}

impl<'a> PartialEq for Metadata<'a> {
    fn eq(&self, _: &Self) -> bool {
        // Allows checking of overall AST structure without checking
        // exact whitespace
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraNode<'a> {
    OnelineComment((&'a str, Span<'a>)),
    BlockComment((&'a str, Span<'a>)),
    Directive(CompilerDirective<'a>),
    Newline,
}
