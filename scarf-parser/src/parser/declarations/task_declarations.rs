// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use scarf_syntax::*;
use winnow::Parser;
use winnow::error::ModalResult;

pub fn final_specifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FinalSpecifier<'s>, VerboseError<'s>> {
    (token(Token::Colon), token(Token::Final))
        .map(|(a, b)| FinalSpecifier(a, b))
        .parse_next(input)
}
