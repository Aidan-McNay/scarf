// =======================================================================
// interface_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.6

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn interface_item_parser<'a>() -> impl Parser<'a, &'a str, InterfaceItem, ParserError<'a>> {
    todo_parser()
}

pub fn non_port_interface_item_parser<'a>()
-> impl Parser<'a, &'a str, NonPortInterfaceItem, ParserError<'a>> {
    todo_parser()
}
