// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub use ariadne::{Report, Source};
use lexer::*;
pub use lexer::{lex, report_lex_errors};
use parser::*;
pub use parser::{parse, report_parse_errors};
