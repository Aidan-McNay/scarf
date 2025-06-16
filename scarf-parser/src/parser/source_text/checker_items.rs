// =======================================================================
// checker_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.8

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn checker_port_list_parser<'a, I>()
-> impl Parser<'a, I, CheckerPortList, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn checker_or_generate_item_parser<'a, I>()
-> impl Parser<'a, I, CheckerOrGenerateItem, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
