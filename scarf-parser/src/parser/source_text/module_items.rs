// =======================================================================
// module_items.rs
// =======================================================================
// Parsing for 1800-2023 A.1.4

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, fail};
use winnow::token::any;

pub fn system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SeveritySystemTask<'s>, VerboseError<'s>> {
    alt((
        fatal_system_severity_task_parser
            .map(|a| SeveritySystemTask::Fatal(Box::new(a))),
        error_system_severity_task_parser
            .map(|a| SeveritySystemTask::Error(Box::new(a))),
        warning_system_severity_task_parser
            .map(|a| SeveritySystemTask::Warning(Box::new(a))),
        info_system_severity_task_parser
            .map(|a| SeveritySystemTask::Info(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn fatal_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FatalSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarFatal),
        opt_note((
            token(Token::Paren),
            finish_number_parser,
            opt_note((token(Token::Comma), list_of_arguments_parser)),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| FatalSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn error_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ErrorSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarError),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| ErrorSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn warning_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WarningSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarWarning),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| WarningSeveritySystemTask(a, b, c))
        .parse_next(input)
}

pub fn info_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InfoSeveritySystemTask<'s>, VerboseError<'s>> {
    (
        token(Token::DollarInfo),
        opt_note((
            token(Token::Paren),
            opt_note(list_of_arguments_parser),
            token(Token::EParen),
        )),
        token(Token::SColon),
    )
        .map(|(a, b, c)| InfoSeveritySystemTask(a, b, c))
        .parse_next(input)
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
            FinishNumber::Zero(metadata) => {
                FinishNumber::Zero(replace_nodes(metadata, extra_nodes))
            }
            FinishNumber::One(metadata) => {
                FinishNumber::One(replace_nodes(metadata, extra_nodes))
            }
            FinishNumber::Two(metadata) => {
                FinishNumber::Two(replace_nodes(metadata, extra_nodes))
            }
        })
        .parse_next(input)
}

pub fn elaboration_system_severity_task_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ElaborationSeveritySystemTask<'s>, VerboseError<'s>> {
    system_severity_task_parser
        .map(|a| ElaborationSeveritySystemTask(a))
        .parse_next(input)
}

pub fn bind_directive_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BindDirective, VerboseError<'s>> {
    token(Token::Error).value(()).parse_next(input)
}

pub fn module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleItem, VerboseError<'s>> {
    token(Token::Error).value(()).parse_next(input)
}

pub fn module_or_generate_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleOrGenerateItem, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn module_or_generate_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModuleOrGenerateItemDeclaration<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn non_port_module_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonPortModuleItem, VerboseError<'s>> {
    token(Token::Error).value(()).parse_next(input)
}
