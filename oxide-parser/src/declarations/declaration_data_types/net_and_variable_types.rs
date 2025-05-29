// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn class_type_parser<'a>() -> impl Parser<'a, &'a str, ClassType, ParserError<'a>> {
    todo_parser()
}

pub fn interface_class_type_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceClassType, ParserError<'a>> {
    todo_parser()
}
