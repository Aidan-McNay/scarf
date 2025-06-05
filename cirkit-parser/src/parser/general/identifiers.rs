// =======================================================================
// mod.rs
// =======================================================================
// Parsing for 1800-2023 A.9.3

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn checker_identifier_parser<'a, I>()
-> impl Parser<'a, I, CheckerIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CheckerIdentifier(a))
}

pub fn class_identifier_parser<'a, I>() -> impl Parser<'a, I, ClassIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ClassIdentifier(a))
}

pub fn modport_identifier_parser<'a, I>()
-> impl Parser<'a, I, ModportIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ModportIdentifier(a))
}

pub fn module_identifier_parser<'a, I>() -> impl Parser<'a, I, ModuleIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ModuleIdentifier(a))
}

pub fn identifier_parser<'a, I>() -> impl Parser<'a, I, Identifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    select! {
        Token::SimpleIdentifier(text) = e => Identifier::SimpleIdentifier((text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })),
        Token::EscapedIdentifier(text) = e => Identifier::EscapedIdentifier((text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })),
    }
    .labelled("an identifier")
    .then(extra_node_parser())
    .map(|(identifier, b)| match identifier {
        Identifier::SimpleIdentifier((text, metadata)) => {
            Identifier::SimpleIdentifier((text, replace_nodes(metadata, b)))
        }
        Identifier::EscapedIdentifier((text, metadata)) => {
            Identifier::EscapedIdentifier((text, replace_nodes(metadata, b)))
        }
    })
}

pub fn interface_identifier_parser<'a, I>()
-> impl Parser<'a, I, InterfaceIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InterfaceIdentifier(a))
}

pub fn package_identifier_parser<'a, I>()
-> impl Parser<'a, I, PackageIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PackageIdentifier(a))
}

pub fn parameter_identifier_parser<'a, I>()
-> impl Parser<'a, I, ParameterIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ParameterIdentifier(a))
}

pub fn port_identifier_parser<'a, I>() -> impl Parser<'a, I, PortIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PortIdentifier(a))
}

pub fn program_identifier_parser<'a, I>()
-> impl Parser<'a, I, ProgramIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ProgramIdentifier(a))
}

pub fn type_identifier_parser<'a, I>() -> impl Parser<'a, I, TypeIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TypeIdentifier(a))
}

pub fn variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, VariableIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| VariableIdentifier(a))
}
