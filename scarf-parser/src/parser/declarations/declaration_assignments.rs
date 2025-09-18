// =======================================================================
// declaration_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.2.4

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{opt, repeat, todo};

pub fn param_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParamAssignment<'s>, VerboseError<'s>> {
    (
        parameter_identifier_parser,
        repeat(0.., variable_dimension_parser),
        opt((token(Token::Eq), constant_param_expression_parser)),
    )
        .map(|(a, b, c)| ParamAssignment(a, b, c))
        .parse_next(input)
}

pub fn specparam_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecparamAssignment<'s>, VerboseError<'s>> {
    todo(input)
}

pub fn type_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeAssignment<'s>, VerboseError<'s>> {
    (
        type_identifier_parser,
        opt((token(Token::Eq), data_type_or_implicit_parser)),
    )
        .map(|(a, b)| TypeAssignment(a, b))
        .parse_next(input)
}
