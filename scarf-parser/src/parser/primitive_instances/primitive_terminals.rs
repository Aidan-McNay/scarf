// =======================================================================
// primitive_terminals.rs
// =======================================================================
// Parsing for 1800-2023 A.3.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn enable_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, EnableTerminal<'a>, ParserError<'a>> + Clone {
    expression_parser().map(|a| EnableTerminal(a))
}

pub fn inout_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InoutTerminal<'a>, ParserError<'a>> + Clone {
    net_lvalue_parser().map(|a| InoutTerminal(a))
}

pub fn input_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InputTerminal<'a>, ParserError<'a>> + Clone {
    expression_parser().map(|a| InputTerminal(a))
}

pub fn ncontrol_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NcontrolTerminal<'a>, ParserError<'a>> + Clone {
    expression_parser().map(|a| NcontrolTerminal(a))
}

pub fn output_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, OutputTerminal<'a>, ParserError<'a>> + Clone {
    net_lvalue_parser().map(|a| OutputTerminal(a))
}

pub fn pcontrol_terminal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, PcontrolTerminal<'a>, ParserError<'a>> + Clone {
    expression_parser().map(|a| PcontrolTerminal(a))
}
