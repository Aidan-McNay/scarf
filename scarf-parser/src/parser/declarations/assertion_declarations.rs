// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::fail;

pub fn concurrent_assertion_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionItem<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn concurrent_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionStatement<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn assertion_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionItemDeclaration<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn sequence_method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMethodCall<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn sequence_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceInstance<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}
