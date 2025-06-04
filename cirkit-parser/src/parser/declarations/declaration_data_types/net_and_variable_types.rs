// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn class_type_parser<'a, I>() -> impl Parser<'a, I, ClassType, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn interface_class_type_parser<'a, I>()
-> impl Parser<'a, I, InterfaceClassType, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
