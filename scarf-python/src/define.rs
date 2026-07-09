// =======================================================================
// define.rs
// =======================================================================
//! A wrapper around [`scarf_parser::preprocessor::Define`]

use crate::{SpannedToken, lex};
use pyo3::prelude::*;
use scarf_parser::PreprocessorCache;

/// A wrapper around [`scarf_parser::preprocessor::Define`]
#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub struct Define {
    /// The name being defined
    #[pyo3(get, set)]
    pub name: String,
    /// The replacement tokens, if any, to use
    #[pyo3(get, set)]
    pub body: Option<Vec<SpannedToken>>,
}

impl<'a> Define {
    /// Turn a [`Define`] into a [`scarf_parser::preprocessor::Define`]
    pub fn to_rust(
        &'a self,
        cache: &'a PreprocessorCache<'a>,
    ) -> scarf_parser::preprocessor::Define<'a> {
        scarf_parser::preprocessor::Define {
            name: scarf_parser::SpannedString(
                &self.name,
                scarf_syntax::Span::default(),
            ),
            body: match &self.body {
                Some(tokens) => scarf_parser::preprocessor::DefineBody::Text(
                    tokens
                        .into_iter()
                        .map(|python_token| python_token.to_rust(cache))
                        .collect(),
                ),
                None => scarf_parser::preprocessor::DefineBody::Empty,
            },
        }
    }
}

/// Create a [`Define`] for a name with no replacement text
#[pyfunction]
pub fn define_empty(name: String) -> Define {
    Define { name, body: None }
}

/// Create a [`Define`] for a name with some replacement text
#[pyfunction]
pub fn define_text(name: String, text: String) -> Define {
    Define {
        name,
        body: Some(lex(text, "".to_string())),
    }
}
