// =======================================================================
// let_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.12

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn let_expression_parser<'a, I>(
    _expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, LetExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Error)
}
