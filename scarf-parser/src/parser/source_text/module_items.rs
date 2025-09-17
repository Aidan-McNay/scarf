// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::todo;
use winnow::combinator::{alt, opt};
use winnow::token::any;

pub fn system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SystemSeverityTask<'s>, VerboseError<'s>> {
    alt((
        fatal_system_severity_task_parser
            .map(|a| SystemSeverityTask::Fatal(Box::new(a))),
        error_system_severity_task_parser
            .map(|a| SystemSeverityTask::Error(Box::new(a))),
        warning_system_severity_task_parser
            .map(|a| SystemSeverityTask::Warning(Box::new(a))),
        info_system_severity_task_parser
            .map(|a| SystemSeverityTask::Info(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn fatal_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FatalSystemSeverityTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarFatal),
        opt((
            token(Token::Paren),
            finish_number_parser,
            opt((token(Token::Comma), list_of_arguments_parser)),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| FatalSystemSeverityTask(a, b, c))
        .parse_next(input)
}

pub fn error_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ErrorSystemSeverityTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarError),
        opt((
            token(Token::Paren),
            opt(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| ErrorSystemSeverityTask(a, b, c))
        .parse_input(input)
}

pub fn warning_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WarningSystemSeverityTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarWarning),
        opt((
            token(Token::Paren),
            opt(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| WarningSystemSeverityTask(a, b, c))
        .parse_input(input)
}

pub fn info_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InfoSystemSeverityTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarInfo),
        opt((
            token(Token::Paren),
            opt(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| ErrorSystemSeverityTask(a, b, c))
        .parse_input(input)
}

pub fn finish_number_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FinishNumber<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::UnsignedNumber("0") => Some(FinishNumber::Zero(Metadata {
                span: s.1.clone(),
                extra_nodes: vec![],
            })),
            Token::UnsignedNumber("1") => Some(FinishNumber::One(Metadata {
                span: s.1.clone(),
                extra_nodes: vec![],
            })),
            Token::UnsignedNumber("2") => Some(FinishNumber::Two(Metadata {
                span: s.1.clone(),
                extra_nodes: vec![],
            })),
            _ => None,
        }),
        extra_node_parser,
    )
        .map(|(finish_number, extra_nodes)| match finish_number {
            FinishNumber::Zero(metadata) => FinishNumber::Zero(Metadata {
                span: metadata.span,
                extra_nodes: extra_nodes,
            }),
            FinishNumber::One(metadata) => FinishNumber::One(Metadata {
                span: metadata.span,
                extra_nodes: extra_nodes,
            }),
            FinishNumber::Two(metadata) => FinishNumber::Two(Metadata {
                span: metadata.span,
                extra_nodes: extra_nodes,
            }),
        })
        .parse_next(input)
}

pub fn elaboration_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ElaborationSystemSeverityTask<'s>, VerboseError<'s>> {
    system_severity_task_parser
        .map(|a| ElaborationSystemSeverityTask(a))
        .parse_next(input)
}

pub fn bind_directive_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindDirective, VerboseError<'s>> {
    todo(input)
}

pub fn module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleItem, VerboseError<'s>> {
    todo(input)
}

pub fn non_port_module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortModuleItem, VerboseError<'s>> {
    todo(input)
}
