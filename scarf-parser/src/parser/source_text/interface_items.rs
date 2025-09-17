// =======================================================================
// interface_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.6

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::todo;

pub fn interface_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceItem, VerboseError<'s>> {
    todo(input)
}

pub fn non_port_interface_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortInterfaceItem, VerboseError<'s>> {
    todo(input)
}
