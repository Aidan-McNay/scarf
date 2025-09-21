// =======================================================================
// statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.4

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, fail, opt};

pub fn statement_or_null_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StatementOrNull<'s>, VerboseError<'s>> {
    alt((
        statement_parser.map(|a| StatementOrNull::Statement(Box::new(a))),
        (attribute_instance_vec_parser, token(Token::SColon))
            .map(|(a, b)| StatementOrNull::Null(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Statement<'s>, VerboseError<'s>> {
    (
        opt((block_identifier_parser, token(Token::Colon))),
        attribute_instance_vec_parser,
        statement_item_parser,
    )
        .map(|(a, b, c)| Statement(a, b, c))
        .parse_next(input)
}

pub fn statement_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StatementItem<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn function_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionStatement<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
