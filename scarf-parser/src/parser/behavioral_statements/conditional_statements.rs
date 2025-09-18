// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.6

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, repeat};

pub fn cond_predicate_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CondPredicate<'s>, VerboseError<'s>> {
    (
        expression_or_cond_pattern_parser,
        repeat(
            0..,
            (token(Token::AmpAmpAmp), expression_or_cond_pattern_parser),
        ),
    )
        .map(|(a, b)| CondPredicate(a, b))
        .parse_next(input)
}

pub fn expression_or_cond_pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ExpressionOrCondPattern<'s>, VerboseError<'s>> {
    alt((
        expression_parser
            .map(|a| ExpressionOrCondPattern::Expression(Box::new(a))),
        cond_pattern_parser
            .map(|a| ExpressionOrCondPattern::CondPattern(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn cond_pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CondPattern<'s>, VerboseError<'s>> {
    (expression_parser, token(Token::Matches), pattern_parser)
        .map(|(a, b, c)| CondPattern(a, b, c))
        .parse_next(input)
}
