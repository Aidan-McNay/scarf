// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the preprocessor

pub mod conditional_compilation;
pub mod configs;
pub mod define;
pub mod keywords;
pub mod text_macro;
use crate::*;
pub use conditional_compilation::*;
pub use configs::*;
pub use define::*;
pub use keywords::*;
use std::iter::Peekable;
pub use text_macro::*;

pub(crate) trait Pushable<T> {
    fn push_element(&mut self, item: T);
}

impl<T> Pushable<T> for Option<&mut Vec<T>> {
    fn push_element(&mut self, item: T) {
        if let Some(inner_vec) = self {
            inner_vec.push(item);
        }
    }
}

pub enum PreprocessorError<'a> {
    // Errors that can be exposed outside preprocess
    Endif(Span<'a>),
    NoEndif(Token<'a>, Span<'a>),
    Elsif(Span<'a>),
    Else(Span<'a>),
    EndKeywords(Span<'a>),
    NoEndKeywords(Span<'a>),
    InvalidDefineArgument(SpannedToken<'a>),
    InvalidVersionSpecifier((&'a str, Span<'a>)),
    IncompleteDirective(Span<'a>),
    IncompleteDirectiveWithToken(SpannedToken<'a>),
    UndefinedMacro((&'a str, Span<'a>)),
    NoMacroArguments((&'a str, Span<'a>)),
    IncompleteMacroWithToken(SpannedToken<'a>),
    Error(VerboseError<'a>),
    // Internal "errors" used for communication
    // - Should not be exposed outside of main preprocess function
    NewlineInDefine(Span<'a>),
    EndOfFunctionArgument(SpannedToken<'a>),
}

impl<'s> From<PreprocessorError<'s>> for VerboseError<'s> {
    fn from(s: PreprocessorError<'s>) -> Self {
        match s {
            PreprocessorError::Endif(endif_span) => VerboseError {
                valid: true,
                span: endif_span,
                found: Some(Token::DirEndif),
                expected: vec![Expectation::Label("a previous `ifdef")],
            },
            PreprocessorError::NoEndif(token, ifdef_span) => VerboseError {
                valid: true,
                span: ifdef_span,
                found: Some(token),
                expected: vec![Expectation::Label("a matching `endif")],
            },
            PreprocessorError::Elsif(elsif_span) => VerboseError {
                valid: true,
                span: elsif_span,
                found: Some(Token::DirElsif),
                expected: vec![Expectation::Label("a previous `ifdef")],
            },
            PreprocessorError::Else(else_span) => VerboseError {
                valid: true,
                span: else_span,
                found: Some(Token::DirElse),
                expected: vec![Expectation::Label("a previous `ifdef")],
            },
            PreprocessorError::EndKeywords(end_keywords_span) => VerboseError {
                valid: true,
                span: end_keywords_span,
                found: Some(Token::DirEndKeywords),
                expected: vec![Expectation::Label(
                    "a previous `begin_keywords",
                )],
            },
            PreprocessorError::NoEndKeywords(begin_span) => VerboseError {
                valid: true,
                span: begin_span,
                found: Some(Token::DirBeginKeywords),
                expected: vec![Expectation::Label("a matching `end_keywords")],
            },
            PreprocessorError::InvalidDefineArgument(err_spanned_token) => {
                VerboseError {
                    valid: true,
                    span: err_spanned_token.1,
                    found: Some(err_spanned_token.0),
                    expected: vec![
                        Expectation::Token(Token::Comma),
                        Expectation::Token(Token::EParen),
                        Expectation::Label("a preprocessor macro argument"),
                    ],
                }
            }
            PreprocessorError::InvalidVersionSpecifier((
                spec_string,
                spec_span,
            )) => VerboseError {
                valid: true,
                span: spec_span,
                found: Some(Token::SimpleIdentifier(spec_string)),
                expected: vec![Expectation::Label("a valid version specifier")],
            },
            PreprocessorError::IncompleteDirective(span) => VerboseError {
                valid: true,
                span: span,
                found: None,
                expected: vec![Expectation::Label("a complete directive")],
            },
            PreprocessorError::IncompleteDirectiveWithToken(
                err_spanned_token,
            ) => VerboseError {
                valid: true,
                span: err_spanned_token.1,
                found: Some(err_spanned_token.0),
                expected: vec![Expectation::Label(
                    "a complete directive or escaped newline after",
                )],
            },
            PreprocessorError::UndefinedMacro((macro_name, macro_span)) => {
                VerboseError {
                    valid: true,
                    span: macro_span,
                    found: Some(Token::TextMacro(macro_name)),
                    expected: vec![Expectation::Label("a previous definition")],
                }
            }
            PreprocessorError::NoMacroArguments((macro_name, macro_span)) => {
                VerboseError {
                    valid: true,
                    span: macro_span,
                    found: Some(Token::TextMacro(macro_name)),
                    expected: vec![Expectation::Label("arguments after")],
                }
            }
            PreprocessorError::IncompleteMacroWithToken(err_spanned_token) => {
                VerboseError {
                    valid: true,
                    span: err_spanned_token.1,
                    found: Some(err_spanned_token.0),
                    expected: vec![Expectation::Label(
                        "a complete macro argument or escaped newline after",
                    )],
                }
            }
            PreprocessorError::Error(verbose_error) => verbose_error,
            PreprocessorError::NewlineInDefine(newline_span) => VerboseError {
                valid: true,
                span: newline_span,
                found: Some(Token::Newline),
                expected: vec![Expectation::Label(
                    "a complete define (internal error)",
                )],
            },
            PreprocessorError::EndOfFunctionArgument(err_spanned_token) => {
                VerboseError {
                    valid: true,
                    span: err_spanned_token.1,
                    found: Some(err_spanned_token.0),
                    expected: vec![Expectation::Label(
                        "a complete function argument (internal error)",
                    )],
                }
            }
        }
    }
}

pub fn preprocess<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut enclosures: Vec<Token<'s>> = vec![];
    while let Some(mut spanned_token) = src.next() {
        match spanned_token.0 {
            Token::DirUndefineall => {
                configs.undefineall();
            }
            Token::DirBeginKeywords => {
                preprocess_keyword_standard(
                    src,
                    dest,
                    configs,
                    spanned_token.1,
                )?;
            }
            Token::DirDefine => {
                preprocess_define(src, configs, spanned_token.1)?;
            }
            Token::DirElse => {
                return Err(PreprocessorError::Else(spanned_token.1));
            }
            Token::DirElsif => {
                return Err(PreprocessorError::Elsif(spanned_token.1));
            }
            Token::DirEndKeywords => {
                return Err(PreprocessorError::EndKeywords(spanned_token.1));
            }
            Token::DirEndif => {
                return Err(PreprocessorError::Endif(spanned_token.1));
            }
            Token::DirIfdef => {
                preprocess_ifdef(src, dest, configs, spanned_token.1, true)?;
            }
            Token::DirIfndef => {
                preprocess_ifdef(src, dest, configs, spanned_token.1, false)?;
            }
            Token::TextMacro(macro_name) => {
                preprocess_macro(
                    src,
                    dest,
                    configs,
                    (macro_name, spanned_token.1),
                )?;
            }
            Token::Bslash if configs.in_define => loop {
                match src.next() {
                    None => dest.push_element(spanned_token),
                    Some(next_token) => match next_token.0 {
                        Token::Newline => dest.push_element(next_token),
                        Token::Bslash => {
                            dest.push_element(spanned_token);
                            spanned_token = next_token;
                            continue;
                        }
                        _ => {
                            dest.push_element(spanned_token);
                            dest.push_element(next_token)
                        }
                    },
                };
                break;
            },
            Token::Newline if configs.in_define => {
                return Err(PreprocessorError::NewlineInDefine(
                    spanned_token.1,
                ));
            }
            Token::Paren if configs.in_define_arg => {
                enclosures.push(Token::Paren);
                dest.push_element(spanned_token);
            }
            Token::Bracket if configs.in_define_arg => {
                enclosures.push(Token::Bracket);
                dest.push_element(spanned_token);
            }
            Token::Brace if configs.in_define_arg => {
                enclosures.push(Token::Brace);
                dest.push_element(spanned_token);
            }
            Token::EParen if configs.in_define_arg => match enclosures.pop() {
                Some(Token::Paren) => dest.push_element(spanned_token),
                None => {
                    return Err(PreprocessorError::EndOfFunctionArgument(
                        spanned_token,
                    ));
                }
                _ => {
                    return Err(PreprocessorError::IncompleteMacroWithToken(
                        spanned_token,
                    ));
                }
            },
            Token::EBracket if configs.in_define_arg => {
                match enclosures.pop() {
                    Some(Token::Bracket) => dest.push_element(spanned_token),
                    _ => {
                        return Err(
                            PreprocessorError::IncompleteMacroWithToken(
                                spanned_token,
                            ),
                        );
                    }
                }
            }
            Token::EBrace if configs.in_define_arg => match enclosures.pop() {
                Some(Token::Brace) => dest.push_element(spanned_token),
                _ => {
                    return Err(PreprocessorError::IncompleteMacroWithToken(
                        spanned_token,
                    ));
                }
            },
            Token::Comma if configs.in_define_arg => {
                if enclosures.is_empty() {
                    return Err(PreprocessorError::EndOfFunctionArgument(
                        spanned_token,
                    ));
                } else {
                    dest.push_element(spanned_token)
                }
            }
            token if token.keyword_replace(&configs.curr_standard) => {
                let new_token = SpannedToken(
                    Token::SimpleIdentifier(token.as_str()),
                    spanned_token.1,
                );
                dest.push_element(new_token)
            }
            _ => dest.push_element(spanned_token),
        }
    }
    Ok(())
}
