// =======================================================================
// function_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.6

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::fail;

pub fn function_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionDeclaration<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
