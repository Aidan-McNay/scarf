// =======================================================================
// error.rs
// =======================================================================
// Warnings/errors that are thrown by the preprocessor

use crate::*;

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
    NotPreviouslyDefinedMacro((&'a str, Span<'a>)),
    DuplicateMacroParameter((&'a str, Span<'a>)),
    NoMacroArguments((&'a str, Span<'a>)),
    TooManyMacroArguments((&'a str, usize, usize, Span<'a>)),
    MissingMacroArgument((&'a str, Span<'a>)),
    InvalidIdentifierFormation((&'a str, Span<'a>)),
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
) -> Report<'s, (String, std::ops::Range<usize>)> {
    let mut report =
        Report::build(kind, (span.file.to_string(), span.bytes.clone()))
            .with_code(code)
            .with_config(
                ariadne::Config::new()
                    .with_index_type(ariadne::IndexType::Byte),
            )
            .with_message(reason)
            .with_label(
                Label::new((span.file.to_string(), span.bytes.clone()))
                    .with_message(code_label)
                    .with_color(Color::Red),
            );
    let mut curr_span: &Span<'s> = &span;
    let mut note = "".to_string();
    let mut note_pad = "".to_string();
    loop {
        if let Some(included_span) = curr_span.included_from {
            curr_span = included_span;
            if note.is_empty() {
                note = format!("Included from {}", curr_span.file);
            } else {
                note = format!(
                    "{}\n{}╰-Included from {}",
                    note, note_pad, curr_span.file
                );
                note_pad += "  ";
            }
        } else {
            break;
        }
    }
    if !note.is_empty() {
        report = report.with_note(note);
    }
    report.finish()
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
            ),
            PreprocessorError::NoEndif(token, ifdef_span) => make_report(
                ifdef_span,
                "PP2",
                format!("No matching `endif for {token}"),
                "No matching `endif".to_owned(),
                ReportKind::Error,
            ),
            PreprocessorError::Elsif(elsif_span) => make_report(
                elsif_span,
                "PP3",
                "Unexpected `elsif".to_string(),
                "Unexpected `elsif".to_string(),
                ReportKind::Error,
            ),
            PreprocessorError::Else(else_span) => make_report(
                else_span,
                "PP4",
                "Unexpected `else".to_string(),
                "Unexpected `else".to_string(),
                ReportKind::Error,
            ),
            PreprocessorError::EndKeywords(end_keywords_span) => make_report(
                end_keywords_span,
                "PP5",
                "`end_keywords with no previous `begin_keywords".to_string(),
                "No matching `begin_keywords".to_string(),
                ReportKind::Error,
            ),
            PreprocessorError::NoEndKeywords(begin_span) => make_report(
                begin_span,
                "PP6",
                "`begin_keywords with no matching `end_keywords".to_string(),
                "No matching `end_keywords".to_string(),
                ReportKind::Error,
            ),
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
                )
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
                )
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
            ),
            PreprocessorError::IncompleteDirective(span) => make_report(
                span,
                "PP9",
                "Incomplete directive".to_string(),
                "Expected a complete directive".to_string(),
                ReportKind::Error,
            ),
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
            ),
            PreprocessorError::UndefinedMacro((macro_name, macro_span)) => {
                make_report(
                    macro_span,
                    "PP11",
                    format!("{macro_name} has not been previously defined"),
                    "Not previously defined".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::NotPreviouslyDefinedMacro((
                macro_name,
                macro_span,
            )) => make_report(
                macro_span,
                "PP12",
                format!("{macro_name} has not been previously defined"),
                "Not previously defined".to_string(),
                ReportKind::Error,
            ),
            PreprocessorError::DuplicateMacroParameter((arg_name, arg_span)) => {
                make_report(
                    arg_span,
                    "PP13",
                    format!("'{arg_name}' was already declared as a macro parameter"),
                    "Parameter already used".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::NoMacroArguments((macro_name, macro_span)) => {
                make_report(
                    macro_span,
                    "PP14",
                    format!("Expected arguments when using {macro_name}"),
                    "Expected arguments not present".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::TooManyMacroArguments((macro_name, expected, found, macro_span)) => {
                make_report(
                    macro_span,
                    "PP15",
                    format!("{} expected {} arguments, but {} were supplied", macro_name, expected, found),
                    "Too many arguments".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::MissingMacroArgument((arg_name, macro_span)) => {
                make_report(
                    macro_span,
                    "PP16",
                    format!("'{arg_name}' wasn't specified and has no default"),
                    "Missing argument".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::InvalidIdentifierFormation((arg_name, arg_span)) => {
                make_report(
                    arg_span,
                    "PP17",
                    format!("The argument for '{arg_name}' cannot be concatenated into an identifier"),
                    "No valid conversion to identifier".to_string(),
                    ReportKind::Error,
                )
            }
            PreprocessorError::IncompleteMacroWithToken(err_spanned_token) => {
                make_report(
                  err_spanned_token.1,
                  "PP18",
                  format!("Usage of {} is incomplete", err_spanned_token.0),
                  "Expected a complete macro argument or escaped newline after".to_string(),
                  ReportKind::Error,
              )
            }
            PreprocessorError::Error(_verbose_error) => {
              todo!("Implement VerboseError::into<Report>")
            },
            PreprocessorError::NewlineInDefine(newline_span) => make_report(
              newline_span,
              "PPX",
              "(Internal Error) Newline in define not handled correctly".to_string(),
              "(Internal Error) Newline in define not handled correctly".to_string(),
              ReportKind::Error,
          ),
            PreprocessorError::EndOfFunctionArgument(err_spanned_token) => {
              make_report(
                err_spanned_token.1,
                "PPX",
                "(Internal Error) End of function argument not handled correctly".to_string(),
                "(Internal Error) End of function argument not handled correctly".to_string(),
                ReportKind::Error,
            )
            }
        }
    }
}
