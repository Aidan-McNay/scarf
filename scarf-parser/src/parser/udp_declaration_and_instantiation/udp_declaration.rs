// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.5.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn udp_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<UdpDeclaration, VerboseError<'s>> {
    fail.parse_next(input)
}
