// =======================================================================
// package_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.11

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn package_item_parser<'a, I>() -> impl Parser<'a, I, PackageItem, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
