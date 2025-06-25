// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.8.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_function_call_parser<'a, I>(
    _constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, ConstantFunctionCall<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn function_subroutine_call_parser<'a, I>()
-> impl Parser<'a, I, FunctionSubroutineCall<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn list_of_arguments_parser<'a, I>()
-> impl Parser<'a, I, ListOfArguments<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
