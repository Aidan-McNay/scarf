// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use cirkit_syntax::Span;
use cirkit_syntax::*;

// Fold parsed text into a vector
pub fn foldl_vector<T>(mut a: Vec<T>, b: T) -> Vec<T> {
    a.push(b);
    a
}

// A parser for matching a token with metadata
pub fn token<'a, I, R>(
    tok: Token<'a>,
    mapping: impl Fn(SimpleSpan) -> R + 'a,
) -> impl Parser<'a, I, R, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    just(tok).map_with(move |_, e| mapping(e.span()))
}

// Span conversion
pub fn convert_span(simple_span: ParserSpan) -> Span {
    simple_span.into()
}

// A parser for matching extra nodes
pub fn extra_node_parser<'a, I>() -> impl Parser<'a, I, Vec<(ExtraNode<'a>, Span)>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::OnelineComment(text) = e => (ExtraNode::OnelineComment(text), convert_span(e.span()))
    }
    .repeated()
    .collect::<Vec<(ExtraNode<'a>, Span)>>()
}

// A mapping function for replacing extra nodes in metadata
pub fn replace_nodes<'a>(
    old_metadata: Metadata<'a>,
    new_nodes: Vec<(ExtraNode<'a>, Span)>,
) -> Metadata<'a> {
    Metadata {
        span: old_metadata.span,
        extra_nodes: new_nodes,
    }
}

pub fn todo_parser<'a, I>() -> impl Parser<'a, I, (), ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    // Match against nothing
    just(Token::Error).to(())
}
