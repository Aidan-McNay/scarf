// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.8

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::token::any;

pub fn string_literal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StringLiteral<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::StringLiteral(text) => {
                Some(StringLiteral::QuotedString(Box::new(QuotedString(
                    text,
                    Metadata {
                        span: s.1.clone(),
                        extra_nodes: Vec::new(),
                    },
                ))))
            }
            Token::TripleQuoteStringLiteral(text) => {
                Some(StringLiteral::TripleQuotedString(Box::new(
                    TripleQuotedString(
                        text,
                        Metadata {
                            span: s.1.clone(),
                            extra_nodes: Vec::new(),
                        },
                    ),
                )))
            }
            _ => None,
        }),
        extra_node_parser,
    )
        .map(|(string, extra_nodes)| match string {
            StringLiteral::QuotedString(quoted_string_box) => {
                StringLiteral::QuotedString(Box::new(QuotedString(
                    quoted_string_box.0,
                    replace_nodes(quoted_string_box.1, extra_nodes),
                )))
            }
            StringLiteral::TripleQuotedString(triple_quoted_string_box) => {
                StringLiteral::TripleQuotedString(Box::new(TripleQuotedString(
                    triple_quoted_string_box.0,
                    replace_nodes(triple_quoted_string_box.1, extra_nodes),
                )))
            }
        })
        .context("a string")
        .parse_next(input)
}
