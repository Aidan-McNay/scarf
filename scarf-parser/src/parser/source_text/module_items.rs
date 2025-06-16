// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, SystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        fatal_system_severity_task_parser().map(|a| SystemSeverityTask::Fatal(Box::new(a))),
        error_system_severity_task_parser().map(|a| SystemSeverityTask::Error(Box::new(a))),
        warning_system_severity_task_parser().map(|a| SystemSeverityTask::Warning(Box::new(a))),
        info_system_severity_task_parser().map(|a| SystemSeverityTask::Info(Box::new(a))),
    ))
    .boxed()
}

pub fn fatal_system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, FatalSystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::DollarFatal)
        .then(
            token(Token::Paren)
                .then(finish_number_parser())
                .then(
                    token(Token::Comma)
                        .then(list_of_arguments_parser())
                        .or_not(),
                )
                .then(token(Token::EParen))
                .map(|(((a, b), c), d)| (a, b, c, d))
                .or_not(),
        )
        .then(token(Token::SColon))
        .map(|((a, b), c)| FatalSystemSeverityTask(a, b, c))
}

pub fn error_system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, ErrorSystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::DollarError)
        .then(
            token(Token::Paren)
                .then(list_of_arguments_parser().or_not())
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::SColon))
        .map(|((a, b), c)| ErrorSystemSeverityTask(a, b, c))
}

pub fn warning_system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, WarningSystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::DollarWarning)
        .then(
            token(Token::Paren)
                .then(list_of_arguments_parser().or_not())
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::SColon))
        .map(|((a, b), c)| WarningSystemSeverityTask(a, b, c))
}

pub fn info_system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, InfoSystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::DollarInfo)
        .then(
            token(Token::Paren)
                .then(list_of_arguments_parser().or_not())
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::SColon))
        .map(|((a, b), c)| InfoSystemSeverityTask(a, b, c))
}

pub fn finish_number_parser<'a, I>() -> impl Parser<'a, I, FinishNumber<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::UnsignedNumber(num) = e if num == "0" => FinishNumber::Zero(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::UnsignedNumber(num) = e if num == "1" => FinishNumber::One(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::UnsignedNumber(num) = e if num == "2" => FinishNumber::Two(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
    }
}

pub fn elaboration_system_severity_task_parser<'a, I>()
-> impl Parser<'a, I, ElaborationSystemSeverityTask<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    system_severity_task_parser().map(|a| ElaborationSystemSeverityTask(a))
}

pub fn bind_directive_parser<'a, I>() -> impl Parser<'a, I, BindDirective, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn module_item_parser<'a, I>() -> impl Parser<'a, I, ModuleItem, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn non_port_module_item_parser<'a, I>()
-> impl Parser<'a, I, NonPortModuleItem, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
