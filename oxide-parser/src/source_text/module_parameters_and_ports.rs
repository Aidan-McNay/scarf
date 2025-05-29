// =======================================================================
// module_parameters_and_ports.rs
// =======================================================================
// Parsing for 1800-2023 A.1.3

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn parameter_port_list_parser<'a>()
-> impl Parser<'a, &'a str, ParameterPortList, ParserError<'a>> {
    todo_parser()
}

pub fn list_of_ports_parser<'a>() -> impl Parser<'a, &'a str, ListOfPorts, ParserError<'a>> {
    todo_parser()
}

pub fn list_of_port_declarations_parser<'a>()
-> impl Parser<'a, &'a str, ListOfPortDeclarations, ParserError<'a>> {
    todo_parser()
}
