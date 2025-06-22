// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use chumsky::input::ValueInput;
use chumsky::prelude::*;
use scarf_syntax::Span;
use scarf_syntax::*;

// Fold parsed text into a vector
pub fn foldl_vector<T>(mut a: Vec<T>, b: T) -> Vec<T> {
    a.push(b);
    a
}

// Span conversion
pub fn convert_span(simple_span: ParserSpan) -> Span {
    simple_span.into()
}

// A parser for matching extra nodes
pub fn extra_node_parser<'a, I>()
-> impl Parser<'a, I, Vec<(ExtraNode<'a>, Span)>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _comment_parser = select! {
        Token::OnelineComment(text) = e => (ExtraNode::OnelineComment(text), convert_span(e.span())),
        Token::BlockComment(text) = e => (ExtraNode::BlockComment(text), convert_span(e.span()))
    }
    .labelled("a comment");
    let _whitespace_parser = select! {
        Token::Newline = e => (ExtraNode::Newline, convert_span(e.span()))
    }
    .labelled("whitespace");
    choice((_comment_parser, _whitespace_parser))
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

// A parser for matching a token and extra nodes, producing metadata
pub fn token<'a, I>(
    token_to_match: Token<'a>,
) -> impl Parser<'a, I, Metadata<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    just(token_to_match)
        .map_with(|_, e| Metadata {
            span: convert_span(e.span()),
            extra_nodes: Vec::default(),
        })
        .then(extra_node_parser())
        .map(|(a, b)| replace_nodes(a, b))
}

pub fn todo_parser<'a, I>() -> impl Parser<'a, I, (), ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    // Match against nothing
    just(Token::Error).to(())
}
