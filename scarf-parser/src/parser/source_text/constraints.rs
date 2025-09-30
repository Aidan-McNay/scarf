// =======================================================================
// constraints.rs
// =======================================================================
// Parsing for 1800-2023 A.1.10

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn constraint_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstraintBlock<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
