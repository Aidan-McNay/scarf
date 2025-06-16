// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.8.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn list_of_arguments_parser<'a, I>() -> impl Parser<'a, I, ListOfArguments<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
