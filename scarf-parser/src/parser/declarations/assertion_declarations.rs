// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn sequence_method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMethodCall<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
