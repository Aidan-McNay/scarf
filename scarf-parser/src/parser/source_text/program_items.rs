// =======================================================================
// program_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn program_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramItem, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn non_port_program_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NonPortProgramItem, ParserError<'a>> + Clone {
    todo_parser()
}
