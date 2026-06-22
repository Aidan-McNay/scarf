// =======================================================================
// lib.rs
// =======================================================================
//! Python bindings for the Scarf SystemVerilog tools
//!
//! Currently, only read-only access is provided; a Rust AST cannot be
//! constructed from Python. Instead, a source file can be processed in
//! Rust and then exposed in Python for examination. Many data structures
//! are simplified when exposed to Python.
//!
//! When crossing the FFI boundary, Rust's borrow checker can no longer
//! provide lifetime guarantees; as such, many data structures must be
//! cloned. If optimal runtime/space usage becomes a concern, native Rust
//! applications should be considered instead.

mod define;
mod error;
mod node;
mod token;
use std::path::PathBuf;

pub use define::define as definitions;
pub use error::*;
pub use node::*;
use pyo3::prelude::*;
use scarf_parser::{LexedSource, PreprocessorCache};
pub use token::*;

#[pymodule]
pub mod scarf_python {
    #[pymodule_export]
    pub use super::definitions;
    #[pymodule_export]
    pub use super::lex;
    #[pymodule_export]
    pub use super::{Bytes, Node, NodeIterator, Span, SpannedToken, Token};
    #[pymodule_export]
    pub use super::{ParserResult, parse, parse_from_preprocess};
    #[pymodule_export]
    pub use super::{
        PreprocessorError, PreprocessorResult, preprocess, preprocess_from_lex,
    };
}

// -----------------------------------------------------------------------
// Lexing
// -----------------------------------------------------------------------

#[pyfunction]
pub fn lex(src: String, file_name: String) -> Vec<SpannedToken> {
    scarf_parser::lex(&src, &file_name)
        .tokens()
        .map(|rust_spanned_token| rust_spanned_token.into())
        .collect()
}

// -----------------------------------------------------------------------
// Preprocessing
// -----------------------------------------------------------------------

#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum PreprocessorResult {
    Ok { tokens: Vec<SpannedToken> },
    Err { error: PreprocessorError },
}

#[pyfunction]
pub fn preprocess_from_lex(
    tokens: Vec<SpannedToken>,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::definitions::Define>,
) -> PreprocessorResult {
    let cache = PreprocessorCache::new();
    let rust_tokens = tokens
        .iter()
        .map(|python_token| python_token.to_rust(&cache));
    let mut state = scarf_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    match scarf_parser::preprocess(rust_tokens, &mut state, &cache) {
        Ok(tokens) => PreprocessorResult::Ok {
            tokens: tokens
                .into_iter()
                .map(|rust_token| rust_token.into())
                .collect(),
        },
        Err(err) => PreprocessorResult::Err { error: err.into() },
    }
}

#[pyfunction]
pub fn preprocess(
    src: String,
    file_name: String,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::definitions::Define>,
) -> PreprocessorResult {
    let cache = PreprocessorCache::new();
    let tokens = scarf_parser::lex(&src, &file_name).tokens();
    let mut state = scarf_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    match scarf_parser::preprocess(tokens, &mut state, &cache) {
        Ok(tokens) => PreprocessorResult::Ok {
            tokens: tokens
                .into_iter()
                .map(|rust_token| rust_token.into())
                .collect(),
        },
        Err(err) => PreprocessorResult::Err { error: err.into() },
    }
}

// -----------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------

#[pyclass(eq, from_py_object, module = "scarf_python")]
#[derive(Clone, PartialEq, Eq)]
pub enum ParserResult {
    Ok {
        root: Node,
    },
    ParserErr {
        error: VerboseError,
    },
    PreprocessorErr {
        preprocessor_error: PreprocessorError,
    },
}

#[pyfunction]
pub fn parse_from_preprocess(tokens: Vec<SpannedToken>) -> ParserResult {
    let cache = PreprocessorCache::new();
    let rust_tokens = tokens
        .iter()
        .map(|python_token| python_token.to_rust(&cache))
        .collect::<Vec<_>>();
    match scarf_parser::parse(&rust_tokens) {
        Ok(result) => {
            let node: scarf_syntax::Node<'_, '_> = (&result).into();
            ParserResult::Ok { root: node.into() }
        }
        Err(err) => ParserResult::ParserErr { error: err.into() },
    }
}

#[pyfunction]
pub fn parse(
    src: String,
    file_name: String,
    include_paths: Vec<PathBuf>,
    defines: Vec<crate::definitions::Define>,
) -> ParserResult {
    let cache = PreprocessorCache::new();
    let tokens = scarf_parser::lex(&src, &file_name).tokens();
    let mut state = scarf_parser::PreprocessorState::new(
        include_paths
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect(),
        defines
            .iter()
            .map(|python_define| python_define.to_rust(&cache))
            .collect(),
    );
    let tokens = match scarf_parser::preprocess(tokens, &mut state, &cache) {
        Ok(tokens) => tokens,
        Err(err) => {
            return ParserResult::PreprocessorErr {
                preprocessor_error: err.into(),
            };
        }
    };
    match scarf_parser::parse(&tokens) {
        Ok(result) => {
            let node: scarf_syntax::Node<'_, '_> = (&result).into();
            ParserResult::Ok { root: node.into() }
        }
        Err(err) => ParserResult::ParserErr { error: err.into() },
    }
}
