// =======================================================================
// constraints.rs
// =======================================================================
// Parsing for 1800-2023 A.1.10

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::{fail, opt};

pub fn constraint_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintBlock<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn expression_or_dist_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExpressionOrDist<'s>, VerboseError<'s>> {
    (
        expression_parser,
        opt((
            token(Token::Dist),
            token(Token::Brace),
            dist_list_parser,
            token(Token::EBrace),
        )),
    )
        .map(|(a, b)| ExpressionOrDist(a, b))
        .parse_next(input)
}

pub fn dist_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DistList<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
