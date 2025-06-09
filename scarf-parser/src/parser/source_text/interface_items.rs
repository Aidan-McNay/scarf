// =======================================================================
// interface_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.6

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn interface_item_parser<'a, I>() -> impl Parser<'a, I, InterfaceItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn non_port_interface_item_parser<'a, I>()
-> impl Parser<'a, I, NonPortInterfaceItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
