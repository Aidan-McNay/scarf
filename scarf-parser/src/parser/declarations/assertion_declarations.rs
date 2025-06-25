// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn sequence_method_call_parser<'a, I>(
    _expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, SequenceMethodCall<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Error)
}
