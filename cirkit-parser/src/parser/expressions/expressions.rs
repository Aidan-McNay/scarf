// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn constant_expression_parser<'a, I>() -> impl Parser<'a, I, ConstantExpression, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
