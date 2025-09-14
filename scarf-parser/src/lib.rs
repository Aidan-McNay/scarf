// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub use ariadne::{Report, Source};
use lexer::*;
pub use lexer::{Span, Token, lex, report_lex_errors};
pub use parser::Tokens;
