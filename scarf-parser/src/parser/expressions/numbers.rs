// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn fixed_point_number_parser<'a, I>()
-> impl Parser<'a, I, FixedPointNumber<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    unsigned_number_parser()
        .then_ignore(just(Token::Period))
        .then(unsigned_number_parser())
        .map(|(a, b)| FixedPointNumber(a, b))
}

pub fn unsigned_number_parser<'a, I>()
-> impl Parser<'a, I, UnsignedNumber<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::UnsignedNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("an unsigned number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| (text, replace_nodes(metadata, b)))
}
