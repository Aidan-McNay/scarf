// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.5.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn udp_declaration_parser<'a, I>() -> impl Parser<'a, I, UdpDeclaration, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    todo_parser()
}
