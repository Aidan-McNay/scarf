// =======================================================================
// metadata.rs
// =======================================================================
// Extra metadata attached to leaf nodes to encode a CST

use crate::*;
use core::ops::Range;

/// The start and end bytes of a particular portion of a source file
pub type ByteSpan = Range<usize>;

/// A representation of a unique location in a source file
///
/// If the file was included from another file (using the `#include`
/// directive), [`Span::included_from`] will reference the [`Span`]
/// of the include directive
///
/// If the [`Span`] is part of a `#define` directive, each expanded
/// text macro will have the original [`Span`] of the `#define` token,
/// with [`Span::expanded_from`] referencing the macro expansion
/// directive
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub file: &'a str,
    pub bytes: ByteSpan,
    pub expanded_from: Option<&'a Span<'a>>,
    pub included_from: Option<&'a Span<'a>>,
}

/// A relationship between two [`Span`]s, ordering them relative to
/// each other in declaration order
#[derive(Debug, PartialEq)]
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
    /// An empty [`Span`]
    pub const fn empty() -> Span<'a> {
        Span {
            file: "",
            bytes: ByteSpan { start: 0, end: 0 },
            expanded_from: None,
            included_from: None,
        }
    }
    /// Compare two [`Span`]s, returning the relationship of the first to the second
    ///
    /// ```rust
    /// # use scarf_syntax::*;
    /// let span1 = Span {
    ///     file: "test",
    ///     bytes: ByteSpan { start: 0, end: 2 },
    ///     expanded_from: None,
    ///     included_from: None
    /// };
    /// let span2 = Span {
    ///     file: "test",
    ///     bytes: ByteSpan { start: 6, end: 8 },
    ///     expanded_from: None,
    ///     included_from: None
    /// };
    /// assert_eq!(span1.compare(&span2), SpanRelation::Earlier)
    /// ```
    ///
    /// Expanded [`Span`]s will be compared starting at their expansion
    /// point, and working backwards through `#define`s. Included [`Span`]s
    /// will be compared starting at their first `#include` and working
    /// through the include hierarchy to their final token.
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

/// Metadata for a given syntax token.
///
/// This includes the [`Span`] of the token. With the `lossless` feature,
/// [`Metadata`] also includes `non_trivia`, which stores non-trivia tokens
/// such as whitespace and comments
#[cfg(feature = "lossless")]
#[derive(Default, Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
    pub non_trivia: Vec<NonTriviaToken<'a>>,
}

/// Metadata for a given syntax token.
///
/// This includes the [`Span`] of the token. With the `lossless` feature,
/// [`Metadata`] also includes `non_trivia`, which stores non-trivia tokens
/// such as whitespace and comments
#[cfg(not(feature = "lossless"))]
#[derive(Default, Clone, Debug)]
pub struct Metadata<'a> {
    pub span: Span<'a>,
}

impl<'a> Metadata<'a> {
    /// Construct a new [`Metadata`]. If the `lossless` feature isn't enabled,
    /// `non_trivia` is discarded.
    #[cfg(feature = "lossless")]
    pub fn new(span: Span<'a>, non_trivia: Vec<NonTriviaToken<'a>>) -> Self {
        Self { span, non_trivia }
    }

    /// Construct a new [`Metadata`]. If the `lossless` feature isn't enabled,
    /// `non_trivia` is discarded.
    #[cfg(not(feature = "lossless"))]
    pub fn new(span: Span<'a>, _: Vec<NonTriviaToken<'a>>) -> Self {
        Self { span }
    }
}

impl<'a> PartialEq for Metadata<'a> {
    fn eq(&self, _: &Self) -> bool {
        // Allows checking of overall CST structure without checking
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

/// A non-trivia token from the source file
#[derive(Clone, Debug, PartialEq)]
pub enum NonTriviaToken<'a> {
    OnelineComment((&'a str, Span<'a>)),
    BlockComment((&'a str, Span<'a>)),
    Newline,
}
