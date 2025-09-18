// =======================================================================
// case_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, repeat};

pub fn range_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RangeList<'s>, VerboseError<'s>> {
    let _value_range_vec_parser =
        repeat(0.., (token(Token::Comma), value_range_parser));
    (value_range_parser, _value_range_vec_parser)
        .map(|(a, b)| RangeList(a, b))
        .parse_next(input)
}

pub fn value_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ValueRange<'s>, VerboseError<'s>> {
    let _expression_parser =
        expression_parser.map(|a| ValueRange::Expression(Box::new(a)));
    let _slice_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::Colon),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| ValueRange::Slice(Box::new((a, b, c, d, e))));
    let _dollar_low_parser = (
        token(Token::Bracket),
        token(Token::Dollar),
        token(Token::Colon),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::DollarLow(Box::new((a, b, c, d, e)))
        });
    let _dollar_high_parser = (
        token(Token::Bracket),
        expression_parser.clone(),
        token(Token::Colon),
        token(Token::Dollar),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::DollarHigh(Box::new((a, b, c, d, e)))
        });
    let _absolute_tolerance_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::PlusSlashMinus),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::AbsoluteTolerance(Box::new((a, b, c, d, e)))
        });
    let _relative_tolerance_parser = (
        token(Token::Bracket),
        expression_parser,
        token(Token::PlusPercentMinus),
        expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            ValueRange::RelativeTolerance(Box::new((a, b, c, d, e)))
        });
    alt((
        _expression_parser,
        _slice_parser,
        _dollar_low_parser,
        _dollar_high_parser,
        _absolute_tolerance_parser,
        _relative_tolerance_parser,
    ))
    .parse_next(input)
}
