// =======================================================================
// package_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.11

use crate::*;
use winnow::ModalResult;
use winnow::combinator::todo;

use scarf_syntax::*;

pub fn package_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageItem, VerboseError<'s>> {
    todo(input)
}
