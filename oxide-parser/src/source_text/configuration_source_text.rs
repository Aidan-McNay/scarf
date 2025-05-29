// =======================================================================
// configuration_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.5

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn config_declaration_parser<'a>()
-> impl Parser<'a, &'a str, ConfigDeclaration, ParserError<'a>> {
    todo_parser()
}
