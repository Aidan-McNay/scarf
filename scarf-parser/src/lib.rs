// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub use ariadne::{Report, Source};
use lexer::*;
pub use lexer::{Span, Token, dump_lex, lex, report_lex_errors};
use parser::*;
pub use parser::{parse, report_parse_errors};
use winnow::Parser;
use winnow::stream::TokenSlice;

pub fn lex_to_parse_stream<'s>(
    input: Vec<(Result<Token<'s>, String>, Span)>,
) -> Vec<SpannedToken<'s>> {
    let mapped_input = input.into_iter().map(|(tok, span)| match tok {
        Ok(tok) => SpannedToken(tok, span),
        Err(_) => SpannedToken(Token::Error, span),
    });
    mapped_input.collect::<Vec<SpannedToken<'s>>>()
}
