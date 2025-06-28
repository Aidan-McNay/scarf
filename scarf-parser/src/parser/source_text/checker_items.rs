// =======================================================================
// checker_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.8

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn checker_port_list_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, CheckerPortList, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn checker_or_generate_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, CheckerOrGenerateItem, ParserError<'a>> + Clone {
    todo_parser()
}
