// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn bind_directive_parser<'a>() -> impl Parser<'a, &'a str, BindDirective, ParserError<'a>> {
    todo_parser()
}

pub fn module_item_parser<'a>() -> impl Parser<'a, &'a str, ModuleItem, ParserError<'a>> {
    todo_parser()
}

pub fn non_port_module_item_parser<'a>()
-> impl Parser<'a, &'a str, NonPortModuleItem, ParserError<'a>> {
    todo_parser()
}
