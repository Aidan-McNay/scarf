// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.8

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn string_literal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, StringLiteral<'a>, ParserError<'a>> + Clone {
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
