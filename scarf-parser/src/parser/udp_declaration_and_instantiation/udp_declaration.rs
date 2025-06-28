// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.5.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn udp_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, UdpDeclaration, ParserError<'a>> + Clone {
    todo_parser()
}
