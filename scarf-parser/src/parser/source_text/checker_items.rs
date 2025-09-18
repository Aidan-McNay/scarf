// =======================================================================
// checker_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.8

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn checker_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerPortList, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn checker_or_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerOrGenerateItem, VerboseError<'s>> {
    fail.parse_next(input)
}
