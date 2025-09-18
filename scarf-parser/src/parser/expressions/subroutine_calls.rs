// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.8.2

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn constant_function_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantFunctionCall<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn function_subroutine_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionSubroutineCall<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfArguments<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
