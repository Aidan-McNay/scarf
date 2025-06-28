// =======================================================================
// declaration_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.2.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn param_assignment_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ParamAssignment<'a>, ParserError<'a>> + Clone {
    parameter_identifier_parser()
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(
            token(Token::Eq)
                .then(constant_param_expression_parser(
                    constant_expression_parser(expression_parser()),
                ))
                .or_not(),
        )
        .map(|((a, b), c)| ParamAssignment(a, b, c))
        .boxed()
}

pub fn specparam_assignment_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, SpecparamAssignment<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn type_assignment_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, TypeAssignment<'a>, ParserError<'a>> + Clone {
    type_identifier_parser()
        .then(
            token(Token::Eq)
                .then(data_type_or_implicit_parser())
                .or_not(),
        )
        .map(|(a, b)| TypeAssignment(a, b))
        .boxed()
}
