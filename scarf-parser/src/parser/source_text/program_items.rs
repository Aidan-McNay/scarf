// =======================================================================
// program_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.7

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::todo;

pub fn program_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ProgramItem, VerboseError<'s>> {
    todo(input)
}

pub fn non_port_program_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortProgramItem, VerboseError<'s>> {
    todo(input)
}
