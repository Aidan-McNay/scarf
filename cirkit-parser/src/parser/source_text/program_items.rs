// =======================================================================
// program_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.7

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn program_item_parser<'a, I>() -> impl Parser<'a, I, ProgramItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn non_port_program_item_parser<'a, I>()
-> impl Parser<'a, I, NonPortProgramItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
