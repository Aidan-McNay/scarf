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
pub use lexer::{Token, lex, report_lex_errors};
use parser::*;
pub use parser::{parse, report_parse_errors};

pub fn parse_from_lex<'a>(
    src: &'a str,
    lexed_stream: Vec<(Result<Token<'a>, String>, Span)>,
) -> ParseResult<SourceText<'a>, Rich<'a, Token<'a>>> {
    let mapped_lexed_src = lexed_stream.into_iter().map(|(tok, span)| match tok {
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
    parse(stream_lexed_src)
}
