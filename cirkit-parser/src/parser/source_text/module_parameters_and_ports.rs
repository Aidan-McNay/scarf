// =======================================================================
// module_parameters_and_ports.rs
// =======================================================================
// Parsing for 1800-2023 A.1.3

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn parameter_port_list_parser<'a, I>() -> impl Parser<'a, I, ParameterPortList, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn list_of_ports_parser<'a, I>() -> impl Parser<'a, I, ListOfPorts, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn list_of_port_declarations_parser<'a, I>()
-> impl Parser<'a, I, ListOfPortDeclarations, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
