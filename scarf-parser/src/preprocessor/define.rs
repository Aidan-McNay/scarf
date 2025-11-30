// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;
use std::iter::Peekable;

fn get_define_name<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span,
) -> Result<SpannedString<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => {
            Ok(SpannedString(id_str, spanned_token.1))
        }
        Token::EscapedIdentifier(id_str) => {
            Ok(SpannedString(id_str, spanned_token.1))
        }
        _ => {
            return Err(PreprocessorError::Error(VerboseError {
                valid: true,
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a preprocessor macro name")],
            }));
        }
    }
}

fn get_define_function_args<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span,
) -> Result<
    Option<
        Vec<(
            SpannedString<'s>,
            Option<(
                Span, // =
                SpannedString<'s>,
            )>,
        )>,
    >,
    PreprocessorError<'s>,
> {
    let Some(spanned_token) = src.peek() else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        _ => Ok(None),
    }
}

fn get_define_body<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<Option<Vec<SpannedToken<'s>>>, PreprocessorError<'s>> {
    let mut define_body: Vec<SpannedToken<'s>> = vec![];
    configs.in_define = true;
    let result = preprocess(src, &mut Some(&mut define_body), configs);
    configs.in_define = false;
    match result {
        Ok(()) => Ok(Some(define_body)),
        Err(PreprocessorError::NewlineInDefine(newline_span)) => {
            define_body.push(SpannedToken(Token::Newline, newline_span));
            Ok(Some(define_body))
        }
        Err(err) => Err(err),
    }
}

pub fn preprocess_define<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    define_span: Span,
) -> Result<(), PreprocessorError<'s>> {
    let define_name = get_define_name(src, define_span.clone())?;
    let function_args = get_define_function_args(src, define_span)?;
    let define_text = get_define_body(src, configs)?;
    let define_body = match function_args {
        Some(arg_vec) => DefineBody::Function(DefineFunction {
            args: arg_vec,
            body: define_text,
        }),
        None => match define_text {
            Some(text_vec) => DefineBody::Text(text_vec),
            None => DefineBody::Empty(),
        },
    };
    configs.define(define_name.0, define_name.1, define_body);
    Ok(())
}
