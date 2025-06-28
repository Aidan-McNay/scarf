// =======================================================================
// class_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.9

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn class_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassItem, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn interface_class_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceClassItem, ParserError<'a>> + Clone {
    todo_parser()
}
