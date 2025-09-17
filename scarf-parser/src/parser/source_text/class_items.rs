// =======================================================================
// class_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.9

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::todo;

pub fn class_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassItem, VerboseError<'s>> {
    todo(input)
}

pub fn interface_class_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassItem, VerboseError<'s>> {
    todo(input)
}
