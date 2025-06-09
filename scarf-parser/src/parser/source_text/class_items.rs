// =======================================================================
// class_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.9

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn class_item_parser<'a, I>() -> impl Parser<'a, I, ClassItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn interface_class_item_parser<'a, I>()
-> impl Parser<'a, I, InterfaceClassItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
