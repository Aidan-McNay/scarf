// =======================================================================
// package_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.11

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn package_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, PackageItem, ParserError<'a>> + Clone {
    todo_parser()
}
