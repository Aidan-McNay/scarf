// =======================================================================
// primitive_terminals.rs
// =======================================================================
// Parsing for 1800-2023 A.3.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn enable_terminal_parser<'a, I>()
-> impl Parser<'a, I, EnableTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    expression_parser().map(|a| EnableTerminal(a))
}

pub fn inout_terminal_parser<'a, I>()
-> impl Parser<'a, I, InoutTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    net_lvalue_parser().map(|a| InoutTerminal(a))
}

pub fn input_terminal_parser<'a, I>()
-> impl Parser<'a, I, InputTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    expression_parser().map(|a| InputTerminal(a))
}

pub fn ncontrol_terminal_parser<'a, I>()
-> impl Parser<'a, I, NcontrolTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    expression_parser().map(|a| NcontrolTerminal(a))
}

pub fn output_terminal_parser<'a, I>()
-> impl Parser<'a, I, OutputTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    net_lvalue_parser().map(|a| OutputTerminal(a))
}

pub fn pcontrol_terminal_parser<'a, I>()
-> impl Parser<'a, I, PcontrolTerminal<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    expression_parser().map(|a| PcontrolTerminal(a))
}
