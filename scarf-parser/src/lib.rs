// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub use ariadne::{Report, Source};
use lexer::*;
pub use lexer::{Span, Token, lex, report_lex_errors};
use parser::*;
use scarf_syntax::DriveStrength;
use winnow::stream::TokenSlice;
use winnow::{
    Parser,
    error::{ContextError, ParseError},
};

pub fn lex_to_parse_stream<'s>(
    input: Vec<(Result<Token<'s>, String>, Span)>,
) -> Vec<SpannedToken<'s>> {
    let mapped_input = input.into_iter().map(|(tok, span)| match tok {
        Ok(tok) => SpannedToken(tok, span),
        Err(_) => SpannedToken(Token::Error, span),
    });
    mapped_input.collect::<Vec<SpannedToken<'s>>>()
}

pub fn parse<'s>(
    input: &'s [SpannedToken<'s>],
) -> Result<
    DriveStrength<'s>,
    ParseError<TokenSlice<'s, SpannedToken<'s>>, ContextError>,
> {
    drive_strength_parser.parse(TokenSlice::new(input))
}
