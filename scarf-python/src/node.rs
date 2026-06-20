// =======================================================================
// node.rs
// =======================================================================
//! A wrapper around [`scarf_syntax::Node`]

use pyo3::prelude::*;
use scarf_syntax::*;
use std::ops::Range;

/// A wrapper around [`std::ops::Range<usize>`]
#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Bytes {
    /// The start of the byte-span (inclusive)
    #[pyo3(get, set)]
    pub start: usize,
    /// The end of the byte-span (exclusive)
    #[pyo3(get, set)]
    pub end: usize,
}

impl<T> From<Range<T>> for Bytes
where
    T: Into<usize>,
{
    fn from(value: Range<T>) -> Self {
        Bytes {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

/// A wrapper around [`scarf_syntax::Span`], providing a location in the
/// source code where a [`Node`] was found
#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Span {
    /// The source file
    #[pyo3(get, set)]
    pub file: String,
    /// The byte-span within the source file
    #[pyo3(get, set)]
    pub bytes: Bytes,
}

impl<'a> From<scarf_syntax::Span<'a>> for Span {
    fn from(value: scarf_syntax::Span<'a>) -> Self {
        Self {
            file: value.file.to_string(),
            bytes: value.bytes.into(),
        }
    }
}

/// A wrapper around [`scarf_syntax::Node`], providing a single AST node
#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Node {
    /// The name of the node (see [`scarf_syntax::Node::name`])
    #[pyo3(get, set)]
    pub name: String,
    /// The span of the node (see [`scarf_syntax::Node::span`])
    #[pyo3(get, set)]
    pub span: Span,
    /// All children of the [`Node`] in the AST (see [`scarf_syntax::Node::children`])
    #[pyo3(get, set)]
    pub children: Vec<Node>,
}

impl<'a, 'b> From<scarf_syntax::Node<'a, 'b>> for Node {
    fn from(value: scarf_syntax::Node<'a, 'b>) -> Self {
        Self {
            name: value.name().to_string(),
            span: value.span().into(),
            children: value
                .children()
                .into_iter()
                .map(|rust_node: scarf_syntax::Node<'_, '_>| rust_node.into())
                .collect(),
        }
    }
}
