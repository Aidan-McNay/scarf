// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.8

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn string_literal_parser<'a, I>() -> impl Parser<'a, I, StringLiteral<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::StringLiteral(text) = e => StringLiteral::QuotedString(Box::new(QuotedString(text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }))),
        Token::EscapedIdentifier(text) = e => StringLiteral::TripleQuotedString(Box::new(TripleQuotedString(text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }))),
    }
}
