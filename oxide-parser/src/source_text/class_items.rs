// =======================================================================
// class_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.9

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn class_item_parser<'a>() -> impl Parser<'a, &'a str, ClassItem, ParserError<'a>> {
    todo_parser()
}

pub fn interface_class_item_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceClassItem, ParserError<'a>> {
    todo_parser()
}
