// =======================================================================
// port_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.1.2

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn inout_declaration_parser<'a, I>() -> impl Parser<'a, I, InoutDeclaration<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Inout)
        .then(net_port_type_parser())
        .then(list_of_port_identifiers_parser())
        .map(|((a, b), c)| InoutDeclaration(a, b, c))
}

pub fn input_declaration_parser<'a, I>() -> impl Parser<'a, I, InputDeclaration<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let net_input_declaration_parser = token(Token::Input)
        .then(net_port_type_parser())
        .then(list_of_port_identifiers_parser())
        .map(|((a, b), c)| InputDeclaration::NetInputDeclaration(Box::new((a, b, c))));
    let variable_input_declaration_parser = token(Token::Input)
        .then(variable_port_type_parser())
        .then(list_of_variable_identifiers_parser())
        .map(|((a, b), c)| InputDeclaration::VariableInputDeclaration(Box::new((a, b, c))));
    choice((
        net_input_declaration_parser,
        variable_input_declaration_parser,
    ))
}

pub fn output_declaration_parser<'a, I>()
-> impl Parser<'a, I, OutputDeclaration<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let net_output_declaration_parser = token(Token::Output)
        .then(net_port_type_parser())
        .then(list_of_port_identifiers_parser())
        .map(|((a, b), c)| OutputDeclaration::NetOutputDeclaration(Box::new((a, b, c))));
    let variable_output_declaration_parser = token(Token::Output)
        .then(variable_port_type_parser())
        .then(list_of_variable_identifiers_parser())
        .map(|((a, b), c)| OutputDeclaration::VariableOutputDeclaration(Box::new((a, b, c))));
    choice((
        net_output_declaration_parser,
        variable_output_declaration_parser,
    ))
}

pub fn interface_port_declaration_parser<'a, I>()
-> impl Parser<'a, I, InterfacePortDeclaration<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let interface_parser = interface_identifier_parser()
        .then(list_of_interface_identifiers_parser())
        .map(|(a, b)| InterfacePortDeclaration::Interface(Box::new((a, b))));
    let modport_parser = interface_identifier_parser()
        .then(token(Token::Period))
        .then(modport_identifier_parser())
        .then(list_of_interface_identifiers_parser())
        .map(|(((a, b), c), d)| InterfacePortDeclaration::Modport(Box::new((a, b, c, d))));
    choice((interface_parser, modport_parser))
}

pub fn ref_declaration_parser<'a, I>() -> impl Parser<'a, I, RefDeclaration<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Ref)
        .then(variable_port_type_parser())
        .then(list_of_variable_identifiers_parser())
        .map(|((a, b), c)| RefDeclaration(a, b, c))
}
