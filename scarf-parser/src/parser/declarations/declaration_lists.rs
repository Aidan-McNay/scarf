// =======================================================================
// declaration_lists.rs
// =======================================================================
// Parsing for 1800-2023 A.2.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn list_of_interface_identifiers_parser<'a, I>()
-> impl Parser<'a, I, ListOfInterfaceIdentifiers<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    interface_identifier_parser()
        .then(
            unpacked_dimension_parser()
                .repeated()
                .collect::<Vec<UnpackedDimension<'a>>>(),
        )
        .then(
            token(Token::Colon)
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
}

pub fn list_of_param_assignments_parser<'a, I>()
-> impl Parser<'a, I, ListOfParamAssignments<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    param_assignment_parser()
        .then(
            token(Token::Colon)
                .then(param_assignment_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, ParamAssignment<'a>)>>(),
        )
        .map(|(a, b)| ListOfParamAssignments(a, b))
}

pub fn list_of_port_identifiers_parser<'a, I>()
-> impl Parser<'a, I, ListOfPortIdentifiers<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    port_identifier_parser()
        .then(
            unpacked_dimension_parser()
                .repeated()
                .collect::<Vec<UnpackedDimension<'a>>>(),
        )
        .then(
            token(Token::Colon)
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
}

pub fn list_of_type_assignments_parser<'a, I>()
-> impl Parser<'a, I, ListOfTypeAssignments<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    type_assignment_parser()
        .then(
            token(Token::Colon)
                .then(type_assignment_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, TypeAssignment<'a>)>>(),
        )
        .map(|(a, b)| ListOfTypeAssignments(a, b))
}

pub fn list_of_variable_identifiers_parser<'a, I>()
-> impl Parser<'a, I, ListOfVariableIdentifiers<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    variable_identifier_parser()
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(
            token(Token::Colon)
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
}
