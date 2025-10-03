// =======================================================================
// block_item_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.8

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::fail;

pub fn block_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BlockItemDeclaration<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
