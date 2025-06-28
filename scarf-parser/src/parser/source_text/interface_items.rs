// =======================================================================
// interface_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.6

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn interface_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceItem, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn non_port_interface_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NonPortInterfaceItem, ParserError<'a>> + Clone {
    todo_parser()
}
