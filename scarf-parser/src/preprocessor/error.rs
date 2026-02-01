// =======================================================================
// error.rs
// =======================================================================
// Warnings/errors that are thrown by the preprocessor

use crate::*;
use ariadne::ReportBuilder;

const NOTE_COLOR: Color = Color::Fixed(81);

#[derive(Debug)]
pub enum PreprocessorError<'a> {
    // Errors that can be exposed outside preprocess
    Endif(Span<'a>),
    NoEndif(Token<'a>, Span<'a>),
    Elsif(Span<'a>),
    Else(Span<'a>),
    EndKeywords(Span<'a>),
    NoEndKeywords(Span<'a>),
    InvalidDefineParameter(SpannedToken<'a>),
    InvalidDefineArgument(SpannedToken<'a>),
    InvalidVersionSpecifier((&'a str, Span<'a>)),
    IncompleteDirective(Span<'a>),
    IncompleteDirectiveWithToken(SpannedToken<'a>),
    UndefinedMacro((&'a str, Span<'a>)),
    RedefinedMacro((&'a str, Span<'a>, Span<'a>)),
    NotPreviouslyDefinedMacro((&'a str, Span<'a>)),
    DuplicateMacroParameter((&'a str, &'a str, Span<'a>, Span<'a>)),
    NoDefaultAfterDefault((SpannedString<'a>, SpannedString<'a>)),
    NoMacroArguments((Span<'a>, (&'a str, Span<'a>))),
    TooManyMacroArguments((Span<'a>, (&'a str, usize, usize, Span<'a>))),
    MissingMacroArgument((Span<'a>, (&'a str, Span<'a>))),
    InvalidIdentifierFormation((&'a str, Span<'a>)),
    InvalidRelativeTimescales(Span<'a>),
    IncompleteMacroWithToken(SpannedToken<'a>),
    Error(VerboseError<'a>),
    // Internal "errors" used for communication
    // - Should not be exposed outside of main preprocess function
    NewlineInDefine(Span<'a>),
    EndOfFunctionArgument(SpannedToken<'a>),
}

fn make_report<'s>(
    span: Span<'s>,
    code: &str,
    reason: String,
    code_label: String,
    kind: ariadne::ReportKind<'s>,
) -> ReportBuilder<'s, (String, std::ops::Range<usize>)> {
    let report =
        Report::build(kind, (span.file.to_string(), span.bytes.clone()))
            .with_code(code)
            .with_config(
                ariadne::Config::new()
                    .with_index_type(ariadne::IndexType::Byte),
            )
            .with_message(reason);
    attach_span_label(span, kind_color(&kind), code_label, report)
}

impl<'s> From<PreprocessorError<'s>>
    for Report<'s, (String, std::ops::Range<usize>)>
{
    fn from(s: PreprocessorError<'s>) -> Self {
        match s {
            PreprocessorError::Endif(endif_span) => make_report(
                endif_span,
                "PP1",
                "Unexpected `endif".to_string(),
                "Unexpected `endif".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::NoEndif(token, ifdef_span) => make_report(
                ifdef_span,
                "PP2",
                format!("No matching `endif for {token}"),
                "No matching `endif".to_owned(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::Elsif(elsif_span) => make_report(
                elsif_span,
                "PP3",
                "Unexpected `elsif".to_string(),
                "Unexpected `elsif".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::Else(else_span) => make_report(
                else_span,
                "PP4",
                "Unexpected `else".to_string(),
                "Unexpected `else".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::EndKeywords(end_keywords_span) => make_report(
                end_keywords_span,
                "PP5",
                "`end_keywords with no previous `begin_keywords".to_string(),
                "No matching `begin_keywords".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::NoEndKeywords(begin_span) => make_report(
                begin_span,
                "PP6",
                "`begin_keywords with no matching `end_keywords".to_string(),
                "No matching `end_keywords".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::InvalidDefineParameter(err_spanned_token) => {
                make_report(
                    err_spanned_token.1,
                    "PP7",
                    format!(
                        "Found {}, expected a preprocessor macro parameter/identifier",
                        err_spanned_token.0
                    ),
                    format!("Unexpected {}", err_spanned_token.0),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::InvalidDefineArgument(err_spanned_token) => {
                make_report(
                    err_spanned_token.1,
                    "PP7",
                    format!(
                        "Found {}, expected a comma, ), or a preprocessor macro argument",
                        err_spanned_token.0
                    ),
                    format!("Unexpected {}", err_spanned_token.0),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::InvalidVersionSpecifier((
                spec_string,
                spec_span,
            )) => make_report(
                spec_span,
                "PP8",
                format!("{spec_string} is not a valid version specifier"),
                "Invalid version specifier".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::IncompleteDirective(span) => make_report(
                span,
                "PP9",
                "Incomplete directive".to_string(),
                "Expected a complete directive".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::IncompleteDirectiveWithToken(
                err_spanned_token,
            ) => make_report(
                err_spanned_token.1,
                "PP10",
                format!(
                    "Found {}, expected more in the directive",
                    err_spanned_token.0
                ),
                "Expected more after".to_string(),
                ReportKind::Error,
            ).finish(),
            PreprocessorError::UndefinedMacro((macro_name, macro_span)) => {
                make_report(
                    macro_span,
                    "PP11",
                    format!("{macro_name} has not been previously defined"),
                    "Not previously defined".to_string(),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::RedefinedMacro((macro_name, macro_span, prev_span)) => {
                attach_span_label(prev_span, NOTE_COLOR, "Previously defined here", make_report(
                    macro_span,
                    "PP12",
                    format!("Redefining {macro_name}"),
                    "New definition".to_string(),
                    ReportKind::Warning,
                )).finish()
            }
            PreprocessorError::NotPreviouslyDefinedMacro((
                macro_name,
                macro_span,
            )) => make_report(
                macro_span,
                "PP13",
                format!("Undefining {macro_name}, which has not been previously defined"),
                "Not previously defined".to_string(),
                ReportKind::Warning,
            ).finish(),
            PreprocessorError::DuplicateMacroParameter((define_name, arg_name, arg_span, prev_span)) => {
                attach_span_label(prev_span, NOTE_COLOR, "Previously declared here", make_report(
                    arg_span,
                    "PP14",
                    format!("'{arg_name}' was already declared as a macro parameter for {define_name}"),
                    "Duplicate parameter declaration".to_string(),
                    ReportKind::Error,
                )).finish()
            }
            PreprocessorError::NoDefaultAfterDefault((last_default_arg, no_default_arg)) => {
                attach_span_label(last_default_arg.1, NOTE_COLOR, format!("{} had a default specified", last_default_arg.0), make_report(
                    no_default_arg.1,
                    "PP15",
                    format!("No default specified for argument after one with a default"),
                    "No default specified".to_string(),
                    ReportKind::Error,
                )).finish()
            }
            PreprocessorError::NoMacroArguments((define_span, (macro_name, macro_span))) => {
                attach_span_label(define_span, NOTE_COLOR, "Macro defined here", make_report(
                    macro_span,
                    "PP16",
                    format!("Expected arguments when using {macro_name}"),
                    "Expected arguments not present".to_string(),
                    ReportKind::Error,
                )).finish()
            }
            PreprocessorError::TooManyMacroArguments((define_span, (macro_name, expected, found, macro_span))) => {
                attach_span_label(define_span, NOTE_COLOR, format!("Macro definition expects {expected} arguments"), make_report(
                    macro_span,
                    "PP17",
                    format!("{} expected {} arguments, but {} were provided", macro_name, expected, found),
                    format!("{found} arguments provided"),
                    ReportKind::Error,
                )).finish()
            }
            PreprocessorError::MissingMacroArgument((define_span, (arg_name, macro_span))) => {
                attach_span_label(define_span, NOTE_COLOR, "Macro defined here", make_report(
                    macro_span,
                    "PP18",
                    format!("'{arg_name}' wasn't specified and has no default"),
                    "Missing argument".to_string(),
                    ReportKind::Error,
                )).finish()
            }
            PreprocessorError::InvalidIdentifierFormation((arg_name, arg_span)) => {
                make_report(
                    arg_span,
                    "PP19",
                    format!("The argument for '{arg_name}' cannot be concatenated into an identifier"),
                    "No valid conversion to identifier".to_string(),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::InvalidRelativeTimescales(timescale_span) => {
                make_report(
                    timescale_span,
                    "PP20",
                    "Time precision is larger than the time unit".to_string(),
                    "Cannot have delay unit be smaller than precision".to_string(),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::IncompleteMacroWithToken(err_spanned_token) => {
                make_report(
                  err_spanned_token.1,
                  "PP21",
                  format!("Usage of {} is incomplete", err_spanned_token.0),
                  "Expected a complete macro argument or escaped newline after".to_string(),
                  ReportKind::Error,
              ).finish()
            }
            PreprocessorError::Error(verbose_error) => {
              verbose_error.into()
            },
            PreprocessorError::NewlineInDefine(newline_span) => make_report(
              newline_span,
              "PPX",
              "(Internal Error) Newline in define not handled correctly".to_string(),
              "(Internal Error) Newline in define not handled correctly".to_string(),
              ReportKind::Error,
          ).finish(),
            PreprocessorError::EndOfFunctionArgument(err_spanned_token) => {
              make_report(
                err_spanned_token.1,
                "PPX",
                "(Internal Error) End of function argument not handled correctly".to_string(),
                "(Internal Error) End of function argument not handled correctly".to_string(),
                ReportKind::Error,
            ).finish()
            }
        }
    }
}
