// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_expression_parser<'a, I>()
-> impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn constant_param_expression_parser<'a, I>()
-> impl Parser<'a, I, ConstantParamExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn constant_range_parser<'a, I>()
-> impl Parser<'a, I, ConstantRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    constant_expression_parser()
        .then(token(Token::Colon))
        .then(constant_expression_parser())
        .map(|((a, b), c)| ConstantRange(a, b, c))
}

pub fn expression_parser<'a, I>() -> impl Parser<'a, I, Expression, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
