// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn final_specifier_parser<'a, I>()
-> impl Parser<'a, I, FinalSpecifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Colon)
        .then(token(Token::Final))
        .map(|(a, b)| FinalSpecifier(a, b))
        .boxed()
}
