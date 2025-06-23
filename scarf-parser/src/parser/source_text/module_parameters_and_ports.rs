// =======================================================================
// module_parameters_and_ports.rs
// =======================================================================
// Parsing for 1800-2023 A.1.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn parameter_port_list_parser<'a, I>()
-> impl Parser<'a, I, ParameterPortList<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let defaults_parser = token(Token::Pound)
        .then(token(Token::Paren))
        .then(list_of_param_assignments_parser())
        .then(
            token(Token::Comma)
                .then(parameter_port_declaration_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, ParameterPortDeclaration<'a>)>>(),
        )
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| ParameterPortList::Defaults(a, b, c, d, e));
    let no_defaults_parser = token(Token::Pound)
        .then(token(Token::Paren))
        .then(parameter_port_declaration_parser())
        .then(
            token(Token::Comma)
                .then(parameter_port_declaration_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, ParameterPortDeclaration<'a>)>>(),
        )
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| ParameterPortList::NoDefaults(a, b, c, d, e));
    let empty_parser = token(Token::Pound)
        .then(token(Token::Paren))
        .then(token(Token::EParen))
        .map(|((a, b), c)| ParameterPortList::Empty(a, b, c));
    choice((defaults_parser, no_defaults_parser, empty_parser)).boxed()
}

pub fn parameter_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, ParameterPortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        parameter_declaration_parser()
            .map(|a| ParameterPortDeclaration::ParameterDeclaration(Box::new(a))),
        local_parameter_declaration_parser()
            .map(|a| ParameterPortDeclaration::LocalParameterDeclaration(Box::new(a))),
        data_type_parser()
            .then(list_of_param_assignments_parser())
            .map(|(a, b)| ParameterPortDeclaration::DataAssignments(Box::new((a, b)))),
        type_parameter_declaration_parser()
            .map(|a| ParameterPortDeclaration::TypeParameterDeclaration(Box::new(a))),
    ))
    .boxed()
}

pub fn list_of_ports_parser<'a, I>() -> impl Parser<'a, I, ListOfPorts<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Paren)
        .then(port_parser())
        .then(
            token(Token::Comma)
                .then(port_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, Port<'a>)>>(),
        )
        .then(token(Token::EParen))
        .map(|(((a, b), c), d)| ListOfPorts(a, b, c, d))
        .boxed()
}

pub fn list_of_port_declarations_parser<'a, I>()
-> impl Parser<'a, I, ListOfPortDeclarations<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Paren)
        .then(
            attribute_instance_vec_parser()
                .then(ansi_port_declaration_parser())
                .then(
                    token(Token::Comma)
                        .then(attribute_instance_vec_parser())
                        .then(ansi_port_declaration_parser())
                        .map(|((a, b), c)| (a, b, c))
                        .repeated()
                        .collect::<Vec<(
                            Metadata<'a>, // ,
                            Vec<AttributeInstance<'a>>,
                            AnsiPortDeclaration<'a>,
                        )>>(),
                )
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::EParen))
        .map(|((a, b), c)| ListOfPortDeclarations(a, b, c))
        .boxed()
}

pub fn port_declaration_parser<'a, I>()
-> impl Parser<'a, I, PortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _inout_declaration_parser = attribute_instance_vec_parser()
        .then(inout_declaration_parser())
        .map(|(a, b)| PortDeclaration::InoutDeclaration(Box::new((a, b))));
    let _input_declaration_parser = attribute_instance_vec_parser()
        .then(input_declaration_parser())
        .map(|(a, b)| PortDeclaration::InputDeclaration(Box::new((a, b))));
    let _output_declaration_parser = attribute_instance_vec_parser()
        .then(output_declaration_parser())
        .map(|(a, b)| PortDeclaration::OutputDeclaration(Box::new((a, b))));
    let _ref_declaration_parser = attribute_instance_vec_parser()
        .then(ref_declaration_parser())
        .map(|(a, b)| PortDeclaration::RefDeclaration(Box::new((a, b))));
    let _interface_port_declaration_parser = attribute_instance_vec_parser()
        .then(interface_port_declaration_parser())
        .map(|(a, b)| PortDeclaration::InterfacePortDeclaration(Box::new((a, b))));
    choice((
        _inout_declaration_parser,
        _input_declaration_parser,
        _output_declaration_parser,
        _ref_declaration_parser,
        _interface_port_declaration_parser,
    ))
    .boxed()
}

pub fn port_parser<'a, I>() -> impl Parser<'a, I, Port<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _port_expression_parser = port_expression_parser()
        .or_not()
        .map(|a| Port::PortExpression(Box::new(a)));
    let _port_identifier_parser = token(Token::Period)
        .then(port_identifier_parser())
        .then(token(Token::Paren))
        .then(port_expression_parser().or_not())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| Port::PortIdentifier(Box::new((a, b, c, d, e))));
    choice((_port_expression_parser, _port_identifier_parser)).boxed()
}

pub fn port_expression_parser<'a, I>()
-> impl Parser<'a, I, PortExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let single_port_reference_parser =
        port_reference_parser().map(|a| PortExpression::SinglePortReference(Box::new(a)));
    let multi_port_reference_parser = token(Token::Brace)
        .then(port_reference_parser())
        .then(
            token(Token::Comma)
                .then(port_reference_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, PortReference<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| PortExpression::MultiPortReference(Box::new((a, b, c, d))));
    choice((single_port_reference_parser, multi_port_reference_parser)).boxed()
}

pub fn port_reference_parser<'a, I>()
-> impl Parser<'a, I, PortReference<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    port_identifier_parser()
        .then(constant_select_parser(constant_expression_parser()))
        .map(|(a, b)| PortReference(a, b))
        .boxed()
}

pub fn port_direction_parser<'a, I>()
-> impl Parser<'a, I, PortDirection<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Input).map(|a| PortDirection::Input(a)),
        token(Token::Output).map(|a| PortDirection::Output(a)),
        token(Token::Inout).map(|a| PortDirection::Inout(a)),
        token(Token::Ref).map(|a| PortDirection::Ref(a)),
    ))
    .boxed()
}

pub fn net_port_header_parser<'a, I>()
-> impl Parser<'a, I, NetPortHeader<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    port_direction_parser()
        .or_not()
        .then(net_port_type_parser())
        .map(|(a, b)| NetPortHeader(a, b))
        .boxed()
}

pub fn variable_port_header_parser<'a, I>()
-> impl Parser<'a, I, VariablePortHeader<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    port_direction_parser()
        .or_not()
        .then(variable_port_type_parser())
        .map(|(a, b)| VariablePortHeader(a, b))
        .boxed()
}

pub fn interface_port_header_parser<'a, I>()
-> impl Parser<'a, I, InterfacePortHeader<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _interface_identifier_parser = interface_identifier_parser()
        .then(
            token(Token::Period)
                .then(modport_identifier_parser())
                .or_not(),
        )
        .map(|(a, b)| InterfacePortHeader::InterfaceIdentifier((a, b)));
    let _interface_parser = token(Token::Interface)
        .then(
            token(Token::Period)
                .then(modport_identifier_parser())
                .or_not(),
        )
        .map(|(a, b)| InterfacePortHeader::Interface((a, b)));
    choice((_interface_identifier_parser, _interface_parser)).boxed()
}

pub fn ansi_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, AnsiPortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        ansi_net_port_declaration_parser().map(|a| AnsiPortDeclaration::NetPort(Box::new(a))),
        ansi_variable_port_declaration_parser()
            .map(|a| AnsiPortDeclaration::VariablePort(Box::new(a))),
        ansi_constant_port_declaration_parser()
            .map(|a| AnsiPortDeclaration::ConstantPort(Box::new(a))),
    ))
    .boxed()
}

pub fn ansi_net_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, AnsiNetPortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let net_or_interface_port_header_parser = choice((
        net_port_header_parser().map(|a| NetOrInterfacePortHeader::NetPortHeader(Box::new(a))),
        interface_port_header_parser()
            .map(|a| NetOrInterfacePortHeader::InterfacePortHeader(Box::new(a))),
    ));
    net_or_interface_port_header_parser
        .or_not()
        .then(port_identifier_parser())
        .then(
            unpacked_dimension_parser()
                .repeated()
                .collect::<Vec<UnpackedDimension<'a>>>(),
        )
        .then(token(Token::Eq).then(constant_expression_parser()).or_not())
        .map(|(((a, b), c), d)| AnsiNetPortDeclaration(a, b, c, d))
        .boxed()
}

pub fn ansi_variable_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, AnsiVariablePortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    variable_port_header_parser()
        .or_not()
        .then(port_identifier_parser())
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(token(Token::Eq).then(constant_expression_parser()).or_not())
        .map(|(((a, b), c), d)| AnsiVariablePortDeclaration(a, b, c, d))
        .boxed()
}

pub fn ansi_constant_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, AnsiConstantPortDeclaration<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    port_direction_parser()
        .or_not()
        .then(token(Token::Period))
        .then(port_identifier_parser())
        .then(token(Token::Paren))
        .then(expression_parser().or_not())
        .then(token(Token::EParen))
        .map(|(((((a, b), c), d), e), f)| AnsiConstantPortDeclaration(a, b, c, d, e, f))
        .boxed()
}
