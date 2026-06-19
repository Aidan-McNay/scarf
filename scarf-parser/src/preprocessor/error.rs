// =======================================================================
// error.rs
// =======================================================================
//! Warnings/errors that are thrown by the preprocessor

use crate::{include::MAX_INCLUDE_DEPTH, *};
use ariadne::ReportBuilder;
use std::io;

const NOTE_COLOR: Color = Color::Fixed(81);

/// An error encountered during preprocessing
///
/// As preprocessing can affect the interpretation of later
/// source code, these errors are often irrecoverable
///
/// Errors marked with **INTERNAL** are meant for use inside the
/// preprocessor for passing information, and should not be returned
#[derive(Debug)]
pub enum PreprocessorError<'a> {
    /// An `` `endif `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `endif
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::Endif(_))));
    /// ```
    Endif(Span<'a>),
    /// No terminating `` `endif `` for a conditional preprocessor block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `ifdef TEST
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::NoEndif(Token::DirIfdef, _))));
    /// ```
    NoEndif(Token<'a>, Span<'a>),
    /// An `` `elsif `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `elsif
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::Elsif(_))));
    /// ```
    Elsif(Span<'a>),
    /// An `` `else `` encountered outside a conditional preprocessor block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `else
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::Else(_))));
    /// ```
    Else(Span<'a>),
    /// An `` `end_keywords `` encountered outside a `` `begin_keywords `` block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `end_keywords
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::EndKeywords(_))));
    /// ```
    EndKeywords(Span<'a>),
    /// No terminating `` `end_keywords `` for a `` `begin_keywords `` block
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `begin_keywords \"1800-2009\"
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::NoEndKeywords(_))));
    /// ```
    NoEndKeywords(Span<'a>),
    /// A missing parameter in a `` `define `` function declaration where one is expected
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST()
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::InvalidDefineParameter(_))));
    /// ```
    InvalidDefineParameter(SpannedToken<'a>),
    /// A missing or invalid argument specification in a `` `define `` function
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b c)
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::InvalidDefineArgument(_))));
    /// ```
    InvalidDefineArgument(SpannedToken<'a>),
    /// An invalid version specifier for a `` `begin_keywords `` directive
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `begin_keywords \"MyVersion\"
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::InvalidVersionSpecifier(_))));
    /// ```
    InvalidVersionSpecifier((Option<&'a str>, Span<'a>)),
    /// A directive that doesn't have all of the required components
    ///
    /// In general, [`PreprocessorError::VerboseError`] is preferred, but may
    /// not be suitable due to a lack of subsequent tokens
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "`line";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::IncompleteDirective(_))));
    /// ```
    IncompleteDirective(Span<'a>),
    /// An incomplete preprocessor definition, specifically with function macro arguments
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::IncompleteDefine(_))));
    /// ```
    IncompleteDefine(SpannedToken<'a>),
    /// Use of a text macro that wasn't previously defined
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `TEST
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::UndefinedMacro(_))));
    /// ```
    UndefinedMacro((&'a str, Span<'a>)),
    /// Specifying a macro parameter that was already specified
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b, a) a + b
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::DuplicateMacroParameter(_))));
    /// ```
    DuplicateMacroParameter((&'a str, &'a str, Span<'a>, Span<'a>)),
    /// Attempting to have a macro parameter with no default value after one that does
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a = 1, b) a + b
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::NoDefaultAfterDefault(_))));
    /// ```
    NoDefaultAfterDefault((SpannedString<'a>, SpannedString<'a>)),
    /// Specifying no arguments for a macro function that takes arguments
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::NoMacroArguments(_))));
    /// ```
    NoMacroArguments((Span<'a>, (&'a str, Span<'a>))),
    /// Specifying too many arguments for a macro function
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(1, 2, 3)
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::TooManyMacroArguments(_))));
    /// ```
    TooManyMacroArguments((Span<'a>, (&'a str, usize, usize, Span<'a>))),
    /// Missing an argument in a macro function use
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(1)
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::MissingMacroArgument(_))));
    /// ```
    MissingMacroArgument((Span<'a>, (&'a str, Span<'a>))),
    /// An invalid preprocessor identifier specification
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a``_with_``b
    /// `TEST(\"one\", \"two\")
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::InvalidIdentifierFormation(_))));
    /// ```
    InvalidIdentifierFormation((&'a str, Span<'a>)),
    /// A precision that is less precise than the unit in a `` `timescale `` directive
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `timescale 100 fs / 1 s
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::InvalidRelativeTimescales(_))));
    /// ```
    InvalidRelativeTimescales(Span<'a>),
    /// An incomplete macro due to mismatching grouping tokens (`[]`, `()`, or `{}`)
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST(a, b) a + b
    /// `TEST(a = 1, b = 2])
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::IncompleteMacroWithToken(_))));
    /// ```
    IncompleteMacroWithToken(SpannedToken<'a>),
    /// An error reading a file specified by an  `` `include `` macro
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `include \"other.v\"
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::Include(_, _, _))));
    /// ```
    Include(Span<'a>, String, io::Error),
    /// The maximum include depth was hit, likely as a result of a self-referential
    /// `` `include `` sequence
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `include \"test.v\"
    /// ";
    /// state.retain_file(
    ///     "test.v".to_string(),
    ///     source.to_string(),
    ///     &cache,
    /// );
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::IncludeDepth(_, _))));
    /// ```
    IncludeDepth(Span<'a>, Vec<Span<'a>>),
    /// A [`VerboseError`] detailing the expected and found tokens, for a case not covered above
    ///
    /// This is most commonly used when we can provide the user with a bit more context
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `line
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// // Expects a line number
    /// assert!(matches!(preprocess_result, Err(PreprocessorError::VerboseError(_))));
    /// ```
    VerboseError(VerboseError<'a>),
    // Internal "errors" used for communication
    // - Should not be exposed outside of main preprocess function
    /// **INTERNAL**: A newline encountered in a `` `define `` directive
    NewlineInDefine(Span<'a>),
    /// **INTERNAL**: The end of a function argument was encountered
    EndOfFunctionArgument(SpannedToken<'a>),
}

fn make_report<'s>(
    span: &Span<'s>,
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

impl<'s> From<&PreprocessorError<'s>>
    for Report<'s, (String, std::ops::Range<usize>)>
{
    fn from(s: &PreprocessorError<'s>) -> Self {
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
                    &err_spanned_token.1,
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
                    &err_spanned_token.1,
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
                match spec_string {
                    Some(version_string) => format!("{version_string} is not a valid version specifier"),
                    None => "Not a valid version specifier".to_string()
                },
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
            PreprocessorError::IncompleteDefine(
                err_spanned_token,
            ) => make_report(
                &err_spanned_token.1,
                "PP10",
                format!(
                    "Found {}, expected more in the preprocessor definition",
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
                attach_span_label(&last_default_arg.1, NOTE_COLOR, format!("{} had a default specified", last_default_arg.0), make_report(
                    &no_default_arg.1,
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
                  &err_spanned_token.1,
                  "PP21",
                  format!("Usage of {} is incomplete", err_spanned_token.0),
                  "Expected a complete macro argument or escaped newline after".to_string(),
                  ReportKind::Error,
              ).finish()
            }
            PreprocessorError::Include(span, path, io_error) => {
                make_report(
                    span,
                    "PP22",
                    format!("Error when reading {}", path),
                    io_error.to_string(),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::IncludeDepth(span, _prev_include_spans) => {
                make_report(
                    span,
                    "PP23",
                    format!("Include depth of {} reached", MAX_INCLUDE_DEPTH),
                    "Check for an `include loop".to_string(),
                    ReportKind::Error,
                ).finish()
            }
            PreprocessorError::VerboseError(verbose_error) => {
              verbose_error.report("PP24")
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
                &err_spanned_token.1,
                "PPX",
                "(Internal Error) End of function argument not handled correctly".to_string(),
                "(Internal Error) End of function argument not handled correctly".to_string(),
                ReportKind::Error,
            ).finish()
            }
        }
    }
}

/// A warning encountered during preprocessing
///
/// Warnings reflect an irregularity in the source code, but are
/// still well-defined and allow preprocessing to continue
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorWarning<'a> {
    /// Attempted to `` `undef `` a macro that had no previous definition
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `undef TEST
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Ok(_)));
    /// assert!(matches!(state.warnings.first(), Some(PreprocessorWarning::NotPreviouslyDefinedMacro(_))))
    /// ```
    NotPreviouslyDefinedMacro((&'a str, Span<'a>)),
    /// A redefinition of a text macro that was previously defined
    ///
    /// ```rust
    /// # use scarf_parser::*;
    /// # let mut state = PreprocessorState::new(vec![], vec![]);
    /// # let cache = PreprocessorCache::new();
    /// let source = "
    /// `define TEST definition_one
    /// `define TEST definition_two
    /// ";
    /// let input = lex(source, "test.v").tokens();
    /// let preprocess_result = preprocess(
    ///     &mut TokenIterator::new(input.into_iter()),
    ///     &mut state,
    ///     &cache,
    /// );
    /// assert!(matches!(preprocess_result, Ok(_)));
    /// assert!(matches!(state.warnings.first(), Some(PreprocessorWarning::RedefinedMacro(_))))
    /// ```
    RedefinedMacro((&'a str, Span<'a>, Span<'a>)),
}

impl<'s> From<&PreprocessorWarning<'s>>
    for Report<'s, (String, std::ops::Range<usize>)>
{
    fn from(s: &PreprocessorWarning<'s>) -> Self {
        match s {
            PreprocessorWarning::RedefinedMacro((macro_name, macro_span, prev_span)) => {
                attach_span_label(prev_span, NOTE_COLOR, "Previously defined here", make_report(
                    macro_span,
                    "PP12",
                    format!("Redefining {macro_name}"),
                    "Redefined here".to_string(),
                    ReportKind::Warning,
                )).finish()
            }
            PreprocessorWarning::NotPreviouslyDefinedMacro((
                macro_name,
                macro_span,
            )) => make_report(
                macro_span,
                "PP13",
                format!("Undefining {macro_name}, which has not been previously defined"),
                "Not previously defined".to_string(),
                ReportKind::Warning,
            ).finish(),
        }
    }
}
