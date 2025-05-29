// =======================================================================
// program_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.7

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn program_item_parser<'a>() -> impl Parser<'a, &'a str, ProgramItem, ParserError<'a>> {
    todo_parser()
}

pub fn non_port_program_item_parser<'a>()
-> impl Parser<'a, &'a str, NonPortProgramItem, ParserError<'a>> {
    todo_parser()
}
