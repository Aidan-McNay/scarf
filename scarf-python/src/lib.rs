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

mod node;
mod token;
pub use node::*;
use pyo3::prelude::*;
use scarf_parser::{LexedSource, lex as rust_lex};
pub use token::*;

#[pymodule]
pub mod scarf_python {
    #[pymodule_export]
    pub use super::lex;
    #[pymodule_export]
    pub use super::{Bytes, Node, Span, SpannedToken, Token};
}

// -----------------------------------------------------------------------
// Lexing
// -----------------------------------------------------------------------

#[pyfunction]
pub fn lex(src: String, file_name: String) -> Vec<SpannedToken> {
    rust_lex(&src, &file_name)
        .tokens()
        .map(|rust_spanned_token| rust_spanned_token.into())
        .collect()
}

// -----------------------------------------------------------------------
// Preprocessing
// -----------------------------------------------------------------------

// -----------------------------------------------------------------------
// Parsing
// -----------------------------------------------------------------------
