// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn sequence_method_call_parser<'a>(
    _expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, SequenceMethodCall<'a>, ParserError<'a>> + Clone {
    token(Token::Error)
}
