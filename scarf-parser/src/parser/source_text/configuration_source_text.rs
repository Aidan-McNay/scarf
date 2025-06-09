// =======================================================================
// configuration_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.5

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn config_declaration_parser<'a, I>() -> impl Parser<'a, I, ConfigDeclaration, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
