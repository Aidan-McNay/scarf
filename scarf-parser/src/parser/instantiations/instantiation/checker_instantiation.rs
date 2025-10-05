// =======================================================================
// checker_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.1.4

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::fail;

pub fn checker_instantiation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CheckerInstantiation<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
