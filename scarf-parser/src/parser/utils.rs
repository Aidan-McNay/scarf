// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use scarf_syntax::*;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::error::ModalResult;
use winnow::token::any;

// A parser for matching extra nodes
pub fn extra_node_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Vec<ExtraNode<'s>>, VerboseError<'s>> {
    let comment_parser = any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
        Token::OnelineComment(text) => {
            Some(ExtraNode::OnelineComment((text, s.1.clone())))
        }
        Token::BlockComment(text) => {
            Some(ExtraNode::BlockComment((text, s.1.clone())))
        }
        _ => None,
    });
    let newline_parser = any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
        Token::Newline => Some(ExtraNode::Newline),
        _ => None,
    });
    repeat_strict(alt((comment_parser, newline_parser))).parse_next(input)
}

// A mapping function for replacing extra nodes in metadata
pub fn replace_nodes<'a>(
    old_metadata: Metadata<'a>,
    new_nodes: Vec<ExtraNode<'a>>,
) -> Metadata<'a> {
    Metadata {
        span: old_metadata.span,
        extra_nodes: new_nodes,
    }
}

// A parser for matching a token and extra nodes, producing metadata
pub fn token<'s>(
    token_to_match: Token<'s>,
) -> impl FnMut(&mut Tokens<'s>) -> ModalResult<Metadata<'s>, VerboseError<'s>>
{
    move |input: &mut Tokens<'s>| {
        (token_to_match, extra_node_parser)
            .context(token_to_match)
            .parse_next(input)
            .map(|(spanned_token, extra_nodes)| Metadata {
                span: spanned_token.1.clone(),
                extra_nodes: extra_nodes,
            })
    }
}
