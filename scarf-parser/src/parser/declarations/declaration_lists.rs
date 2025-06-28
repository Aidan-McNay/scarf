// =======================================================================
// declaration_lists.rs
// =======================================================================
// Parsing for 1800-2023 A.2.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn list_of_interface_identifiers_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfInterfaceIdentifiers<'a>, ParserError<'a>> + Clone {
    interface_identifier_parser()
        .then(
            unpacked_dimension_parser()
                .repeated()
                .collect::<Vec<UnpackedDimension<'a>>>(),
        )
        .then(
            token(Token::Comma)
                .then(interface_identifier_parser())
                .then(
                    unpacked_dimension_parser()
                        .repeated()
                        .collect::<Vec<UnpackedDimension<'a>>>(),
                )
                .map(|((a, b), c)| (a, b, c))
                .repeated()
                .collect::<Vec<(
                    Metadata<'a>, // ,
                    InterfaceIdentifier<'a>,
                    Vec<UnpackedDimension<'a>>,
                )>>(),
        )
        .map(|((a, b), c)| ListOfInterfaceIdentifiers(a, b, c))
        .boxed()
}

pub fn list_of_param_assignments_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfParamAssignments<'a>, ParserError<'a>> + Clone {
    param_assignment_parser()
        .then(
            token(Token::Comma)
                .then(param_assignment_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, ParamAssignment<'a>)>>(),
        )
        .map(|(a, b)| ListOfParamAssignments(a, b))
        .boxed()
}

pub fn list_of_port_identifiers_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfPortIdentifiers<'a>, ParserError<'a>> + Clone {
    port_identifier_parser()
        .then(
            unpacked_dimension_parser()
                .repeated()
                .collect::<Vec<UnpackedDimension<'a>>>(),
        )
        .then(
            token(Token::Comma)
                .then(port_identifier_parser())
                .then(
                    unpacked_dimension_parser()
                        .repeated()
                        .collect::<Vec<UnpackedDimension<'a>>>(),
                )
                .map(|((a, b), c)| (a, b, c))
                .repeated()
                .collect::<Vec<(
                    Metadata<'a>, // ,
                    PortIdentifier<'a>,
                    Vec<UnpackedDimension<'a>>,
                )>>(),
        )
        .map(|((a, b), c)| ListOfPortIdentifiers(a, b, c))
        .boxed()
}

pub fn list_of_specparam_assignments_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfSpecparamAssignments<'a>, ParserError<'a>> + Clone {
    specparam_assignment_parser()
        .then(
            token(Token::Comma)
                .then(specparam_assignment_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, SpecparamAssignment<'a>)>>(),
        )
        .map(|(a, b)| ListOfSpecparamAssignments(a, b))
        .boxed()
}

pub fn list_of_type_assignments_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfTypeAssignments<'a>, ParserError<'a>> + Clone {
    type_assignment_parser()
        .then(
            token(Token::Comma)
                .then(type_assignment_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, TypeAssignment<'a>)>>(),
        )
        .map(|(a, b)| ListOfTypeAssignments(a, b))
        .boxed()
}

pub fn list_of_variable_identifiers_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfVariableIdentifiers<'a>, ParserError<'a>> + Clone {
    variable_identifier_parser()
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(
            token(Token::Comma)
                .then(variable_identifier_parser())
                .then(
                    variable_dimension_parser()
                        .repeated()
                        .collect::<Vec<VariableDimension<'a>>>(),
                )
                .map(|((a, b), c)| (a, b, c))
                .repeated()
                .collect::<Vec<(
                    Metadata<'a>, // ,
                    VariableIdentifier<'a>,
                    Vec<VariableDimension<'a>>,
                )>>(),
        )
        .map(|((a, b), c)| ListOfVariableIdentifiers(a, b, c))
        .boxed()
}
