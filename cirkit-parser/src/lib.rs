// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub use ariadne::{Report, Source};
use chumsky::input::Stream;
pub use chumsky::input::ValueInput;
use chumsky::prelude::*;
use lexer::*;
pub use lexer::{lex, report_lex_errors};
pub use parser::parse as parse_from_lex;
pub use parser::report_parse_errors;
use parser::*;

pub fn parse<'a>(src: &'a str) -> ParseResult<SourceText<'a>, Rich<'a, Token<'a>>> {
    let lexed_src = lex(src);
    let mapped_lexed_src = lexed_src.into_iter().map(|(tok, span)| match tok {
        Ok(tok) => (
            tok,
            <std::ops::Range<usize> as Into<SimpleSpan>>::into(span),
        ),
        Err(_) => (
            Token::Error,
            <std::ops::Range<usize> as Into<SimpleSpan>>::into(span),
        ),
    });
    let stream_lexed_src = Stream::from_iter(mapped_lexed_src).map((0..src.len()).into(), |x| x);
    trivial_parser().parse(stream_lexed_src)
}
