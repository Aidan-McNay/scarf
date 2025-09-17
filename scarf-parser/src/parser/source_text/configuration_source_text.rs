// =======================================================================
// configuration_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.5

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::todo;

pub fn config_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConfigDeclaration, VerboseError<'s>> {
    todo(input)
}
