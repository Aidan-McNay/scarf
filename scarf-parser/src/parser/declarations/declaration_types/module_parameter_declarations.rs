// =======================================================================
// module_parameter_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.1.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn local_parameter_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LocalParameterDeclaration<'a>, ParserError<'a>> + Clone {
    let data_parameter_parser = token(Token::Localparam)
        .then(data_type_or_implicit_parser())
        .then(list_of_param_assignments_parser())
        .map(|((a, b), c)| LocalParameterDeclaration::DataParameter(a, b, c));
    let type_parameter_parser = token(Token::Localparam)
        .then(type_parameter_declaration_parser())
        .map(|(a, b)| LocalParameterDeclaration::TypeParameter(a, b));
    choice((data_parameter_parser, type_parameter_parser)).boxed()
}

pub fn parameter_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ParameterDeclaration<'a>, ParserError<'a>> + Clone {
    let data_parameter_parser = token(Token::Parameter)
        .then(data_type_or_implicit_parser())
        .then(list_of_param_assignments_parser())
        .map(|((a, b), c)| ParameterDeclaration::DataParameter(a, b, c));
    let type_parameter_parser = token(Token::Parameter)
        .then(type_parameter_declaration_parser())
        .map(|(a, b)| ParameterDeclaration::TypeParameter(a, b));
    choice((data_parameter_parser, type_parameter_parser)).boxed()
}

pub fn type_parameter_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, TypeParameterDeclaration<'a>, ParserError<'a>> + Clone {
    token(Token::Type)
        .then(forward_type_parser().or_not())
        .then(list_of_type_assignments_parser())
        .map(|((a, b), c)| TypeParameterDeclaration(a, b, c))
        .boxed()
}

pub fn specparam_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, SpecparamDeclaration<'a>, ParserError<'a>> + Clone {
    token(Token::Specparam)
        .then(packed_dimension_parser().or_not())
        .then(list_of_specparam_assignments_parser())
        .then(token(Token::SColon))
        .map(|(((a, b), c), d)| SpecparamDeclaration(a, b, c, d))
        .boxed()
}
