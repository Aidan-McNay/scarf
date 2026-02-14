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
    fn include_indeces(&self) -> Vec<usize> {
        match &self.included_from {
            None => {
                vec![self.bytes.start]
            }
            Some(inner_span) => {
                let mut nested_byte_indeces = inner_span.include_indeces();
                nested_byte_indeces.push(self.bytes.start);
                nested_byte_indeces
            }
        }
    }
    fn indeces_to_compare(&self) -> Vec<Vec<usize>> {
        let mut indeces = match self.expanded_from {
            Some(expanded_span) => expanded_span.indeces_to_compare(),
            None => vec![],
        };
        indeces.push(self.include_indeces());
        indeces
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
        let self_include_byte_indeces = self.indeces_to_compare();
        let other_include_byte_indeces = other.indeces_to_compare();
        loop {
            match (
                self_include_byte_indeces.get(idx),
                other_include_byte_indeces.get(idx),
            ) {
                (Some(self_idxs), Some(other_idxs)) => {
                    let mut nested_idx = 0;
                    'match_define: loop {
                        match (
                            self_idxs.get(nested_idx),
                            other_idxs.get(nested_idx),
                        ) {
                            (Some(self_idx), Some(other_idx)) => {
                                if self_idx < other_idx {
                                    return SpanRelation::Earlier;
                                } else if self_idx > other_idx {
                                    return SpanRelation::Later;
                                } else {
                                    nested_idx += 1;
                                }
                            }
                            (None, None) => {
                                break 'match_define;
                            }
                            _ => {
                                panic!(
                                    "Internal error comparing spans {:?} and {:?}",
                                    self, other
                                )
                            }
                        }
                    }
                    idx += 1;
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

#[cfg(feature = "keep_extras")]
#[derive(Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
    pub extra_nodes: Vec<ExtraNode<'a>>,
}

#[cfg(not(feature = "keep_extras"))]
#[derive(Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
}

impl<'a> Metadata<'a> {
    #[cfg(feature = "keep_extras")]
    pub fn new(span: Span<'a>, extra_nodes: Vec<ExtraNode<'a>>) -> Self {
        Self { span, extra_nodes }
    }

    #[cfg(not(feature = "keep_extras"))]
    pub fn new(span: Span<'a>, _: Vec<ExtraNode<'a>>) -> Self {
        Self { span }
    }
}

impl<'a> PartialEq for Metadata<'a> {
    fn eq(&self, _: &Self) -> bool {
        // Allows checking of overall AST structure without checking
        // exact whitespace
        true
    }
}

// Metadata never returns any Nodes while iterating
impl<'a, 'b> IntoIterator for &'b Metadata<'a> {
    type Item = Node<'a, 'b>;
    type IntoIter = std::iter::Empty<Node<'a, 'b>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl<'a, 'b> IntoIterator for &'b mut Metadata<'a> {
    type Item = Node<'a, 'b>;
    type IntoIter = std::iter::Empty<Node<'a, 'b>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraNode<'a> {
    OnelineComment((&'a str, Span<'a>)),
    BlockComment((&'a str, Span<'a>)),
    Directive(CompilerDirective<'a>),
    Newline,
}
