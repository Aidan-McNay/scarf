// =======================================================================
// case_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn range_list_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, RangeList<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _value_range_vec_parser = token(Token::Comma)
        .then(value_range_parser(expression_parser.clone()))
        .repeated()
        .collect::<Vec<(Metadata<'a>, ValueRange<'a>)>>();
    value_range_parser(expression_parser)
        .then(_value_range_vec_parser)
        .map(|(a, b)| RangeList(a, b))
        .boxed()
}

pub fn value_range_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ValueRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _expression_parser = expression_parser
        .clone()
        .map(|a| ValueRange::Expression(Box::new(a)));
    let _slice_parser = token(Token::Bracket)
        .then(expression_parser.clone())
        .then(token(Token::Colon))
        .then(expression_parser.clone())
        .then(token(Token::EBracket))
        .map(|((((a, b), c), d), e)| ValueRange::Slice(Box::new((a, b, c, d, e))));
    let _dollar_low_parser = token(Token::Bracket)
        .then(token(Token::Dollar))
        .then(token(Token::Colon))
        .then(expression_parser.clone())
        .then(token(Token::EBracket))
        .map(|((((a, b), c), d), e)| ValueRange::DollarLow(Box::new((a, b, c, d, e))));
    let _dollar_high_parser = token(Token::Bracket)
        .then(expression_parser.clone())
        .then(token(Token::Colon))
        .then(token(Token::Dollar))
        .then(token(Token::EBracket))
        .map(|((((a, b), c), d), e)| ValueRange::DollarHigh(Box::new((a, b, c, d, e))));
    let _absolute_tolerance_parser = token(Token::Bracket)
        .then(expression_parser.clone())
        .then(token(Token::PlusSlashMinus))
        .then(expression_parser.clone())
        .then(token(Token::EBracket))
        .map(|((((a, b), c), d), e)| ValueRange::AbsoluteTolerance(Box::new((a, b, c, d, e))));
    let _relative_tolerance_parser = token(Token::Bracket)
        .then(expression_parser.clone())
        .then(token(Token::PlusPercentMinus))
        .then(expression_parser.clone())
        .then(token(Token::EBracket))
        .map(|((((a, b), c), d), e)| ValueRange::RelativeTolerance(Box::new((a, b, c, d, e))));
    choice((
        _expression_parser,
        _slice_parser,
        _dollar_low_parser,
        _dollar_high_parser,
        _absolute_tolerance_parser,
        _relative_tolerance_parser,
    ))
    .boxed()
}
