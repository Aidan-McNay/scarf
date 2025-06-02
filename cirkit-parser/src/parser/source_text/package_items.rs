// =======================================================================
// package_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.11

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn package_item_parser<'a>() -> impl Parser<'a, &'a str, PackageItem, ParserError<'a>> {
    todo_parser()
}
