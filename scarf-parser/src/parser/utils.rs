// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use scarf_syntax::*;
use winnow::Parser;
use winnow::error::ModalResult;

// Span conversion
// pub fn convert_span(simple_span: LexerSpan) -> Span {
//     simple_span.into()
// }

// A parser for matching extra nodes
// pub fn extra_node_parser<'a>()
// -> impl Parser<'a, ParserInput<'a>, Vec<(ExtraNode<'a>, Span)>, ParserError<'a>> + Clone {
//     let _comment_parser = select! {
//         Token::OnelineComment(text) = e => (ExtraNode::OnelineComment(text), convert_span(e.span())),
//         Token::BlockComment(text) = e => (ExtraNode::BlockComment(text), convert_span(e.span()))
//     }
//     .labelled("a comment");
//     let _whitespace_parser = select! {
//         Token::Newline = e => (ExtraNode::Newline, convert_span(e.span()))
//     }
//     .labelled("whitespace");
//     choice((_comment_parser, _whitespace_parser))
//         .repeated()
//         .collect::<Vec<(ExtraNode<'a>, Span)>>()
// }

// A mapping function for replacing extra nodes in metadata
// pub fn replace_nodes<'a>(
//     old_metadata: Metadata<'a>,
//     new_nodes: Vec<(ExtraNode<'a>, Span)>,
// ) -> Metadata<'a> {
//     Metadata {
//         span: old_metadata.span,
//         extra_nodes: new_nodes,
//     }
// }

// A parser for matching a token and extra nodes, producing metadata
pub fn token<'s>(
    mut token_to_match: Token<'s>,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>> {
    move |input: &mut Tokens<'s>| {
        token_to_match
            .parse_next(input)
            .map(|spanned_token| Metadata {
                span: spanned_token.1.clone(),
                extra_nodes: vec![],
            })
    }
}

// pub fn todo_parser<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserError<'a>> + Clone {
//     // Match against nothing
//     just(Token::Error).to(())
// }
