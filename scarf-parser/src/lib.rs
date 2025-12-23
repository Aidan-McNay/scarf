// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub mod preprocessor;
pub use ariadne::{Report, Source};
use lexer::*;
pub use lexer::{Token, dump_lex, lex, report_lex_errors};
use parser::*;
pub use parser::{SpannedToken, VerboseError, parse, report_parse_errors};
pub use preprocessor::*;
use winnow::Parser;
use winnow::stream::TokenSlice;
#[cfg(test)]
pub mod test;
pub use scarf_syntax::Span;
#[cfg(test)]
pub use test::*;

pub fn lex_to_parse_stream<'s>(
    input: Vec<(Result<Token<'s>, String>, Span<'s>)>,
) -> Vec<SpannedToken<'s>> {
    let mapped_input = input.into_iter().map(|(tok, span)| match tok {
        Ok(tok) => SpannedToken(tok, span),
        Err(_) => SpannedToken(Token::Error, span),
    });
    mapped_input.collect::<Vec<SpannedToken<'s>>>()
}
