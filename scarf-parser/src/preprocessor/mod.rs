// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the preprocessor

pub mod conditional_compilation;
pub mod configs;
pub mod define;
use crate::*;
pub use conditional_compilation::*;
pub use configs::*;
pub use define::*;
use std::iter::Peekable;

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
    Endif(Span),
    NoEndif(Token<'a>, Span),
    Elsif(Span),
    Else(Span),
    NewlineInDefine(Span),
    IncompleteDirective(Span),
    Error(VerboseError<'a>),
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
            PreprocessorError::NewlineInDefine(newline_span) => VerboseError {
                valid: true,
                span: newline_span,
                found: Some(Token::Newline),
                expected: vec![Expectation::Label(
                    "a complete define (internal error)",
                )],
            },
            PreprocessorError::IncompleteDirective(span) => VerboseError {
                valid: true,
                span: span,
                found: None,
                expected: vec![Expectation::Label("a complete directive")],
            },
            PreprocessorError::Error(verbose_error) => verbose_error,
        }
    }
}

pub fn preprocess<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<(), PreprocessorError<'s>> {
    while let Some(mut spanned_token) = src.next() {
        match spanned_token.0 {
            Token::DirUndefineall => {
                configs.undefineall();
            }
            Token::DirDefine => {
                let define_span = spanned_token.1;
                preprocess_define(src, configs, define_span)?;
            }
            Token::DirElse => {
                let err_span = spanned_token.1.clone();
                return Err(PreprocessorError::Else(err_span));
            }
            Token::DirElsif => {
                let err_span = spanned_token.1.clone();
                return Err(PreprocessorError::Elsif(err_span));
            }
            Token::DirEndif => {
                let err_span = spanned_token.1.clone();
                return Err(PreprocessorError::Endif(err_span));
            }
            Token::DirIfdef => {
                let ifdef_span = spanned_token.1.clone();
                preprocess_ifdef(src, dest, configs, ifdef_span, true)?;
            }
            Token::DirIfndef => {
                let ifndef_span = spanned_token.1.clone();
                preprocess_ifdef(src, dest, configs, ifndef_span, false)?;
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
            _ => dest.push_element(spanned_token),
        }
    }
    Ok(())
}
