// =======================================================================
// declaration_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.2.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn param_assignment_parser<'a, I>() -> impl Parser<'a, I, ParamAssignment<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    parameter_identifier_parser()
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(
            token(Token::Eq)
                .then(constant_param_expression_parser())
                .or_not(),
        )
        .map(|((a, b), c)| ParamAssignment(a, b, c))
}

pub fn specparam_assignment_parser<'a, I>()
-> impl Parser<'a, I, SpecparamAssignment<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn type_assignment_parser<'a, I>() -> impl Parser<'a, I, TypeAssignment<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    type_identifier_parser()
        .then(
            token(Token::Eq)
                .then(data_type_or_implicit_parser())
                .or_not(),
        )
        .map(|(a, b)| TypeAssignment(a, b))
}
