// =======================================================================
// module_parameter_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.1.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn local_parameter_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LocalParameterDeclaration<'s>, VerboseError<'s>> {
    let data_parameter_parser = (
        token(Token::Localparam),
        data_type_or_implicit_parser,
        list_of_param_assignments_parser,
    )
        .map(|(a, b, c)| LocalParameterDeclaration::DataParameter(a, b, c));
    let type_parameter_parser =
        (token(Token::Localparam), type_parameter_declaration_parser)
            .map(|(a, b)| LocalParameterDeclaration::TypeParameter(a, b));
    alt((data_parameter_parser, type_parameter_parser)).parse_next(input)
}

pub fn parameter_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParameterDeclaration<'s>, VerboseError<'s>> {
    let data_parameter_parser = (
        token(Token::Parameter),
        data_type_or_implicit_parser,
        list_of_param_assignments_parser,
    )
        .map(|(a, b, c)| ParameterDeclaration::DataParameter(a, b, c));
    let type_parameter_parser =
        (token(Token::Parameter), type_parameter_declaration_parser)
            .map(|(a, b)| ParameterDeclaration::TypeParameter(a, b));
    alt((data_parameter_parser, type_parameter_parser)).parse_next(input)
}

pub fn type_parameter_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeParameterDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Type),
        opt_note(forward_type_parser),
        list_of_type_assignments_parser,
    )
        .map(|(a, b, c)| TypeParameterDeclaration(a, b, c))
        .parse_next(input)
}

pub fn specparam_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SpecparamDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Specparam),
        opt_note(packed_dimension_parser),
        list_of_specparam_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d)| SpecparamDeclaration(a, b, c, d))
        .parse_next(input)
}
