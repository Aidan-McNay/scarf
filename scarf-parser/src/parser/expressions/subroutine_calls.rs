// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.8.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_function_call_parser<'a>(
    _constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantFunctionCall<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn function_subroutine_call_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, FunctionSubroutineCall<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn list_of_arguments_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ListOfArguments<'a>, ParserError<'a>> + Clone {
    todo_parser()
}
