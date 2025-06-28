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
pub use lexer::{Span, Token, lex, report_lex_errors};
pub use parser::report_parse_errors;
use parser::*;

fn map_span<'a>(src: (Token<'a>, SimpleSpan)) -> (Token<'a>, ParserSpan) {
    src
}

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
    let stream_lexed_src = Stream::from_iter(mapped_lexed_src).boxed().map(
        (0..src.len()).into(),
        map_span as fn((Token<'a>, SimpleSpan)) -> (Token<'a>, ParserSpan),
    );
    parse(stream_lexed_src)
}
