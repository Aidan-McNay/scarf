// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn bind_directive_parser<'a, I>() -> impl Parser<'a, I, BindDirective, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn module_item_parser<'a, I>() -> impl Parser<'a, I, ModuleItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn non_port_module_item_parser<'a, I>() -> impl Parser<'a, I, NonPortModuleItem, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
