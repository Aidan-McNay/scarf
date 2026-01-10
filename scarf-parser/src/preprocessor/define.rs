// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;
use std::iter::Peekable;

fn get_define_token<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    err_span: Span<'s>,
) -> Result<(SpannedToken<'s>, bool), PreprocessorError<'s>> {
    // Return a token, as well as indicating whether it's the
    // end of the define
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(err_span));
    };
    let next_token = src.peek();
    match (spanned_token.0, next_token) {
        (Token::Bslash, Some(next_token)) => match next_token {
            SpannedToken(Token::Newline, _) => {
                let newline_token = src.next().unwrap();
                Ok((newline_token, false))
            }
            _ => Ok((spanned_token, false)),
        },
        (Token::Newline, _) => Ok((spanned_token, true)),
        (_, None) => Ok((spanned_token, true)),
        (_, Some(SpannedToken(Token::Newline, _))) => Ok((spanned_token, true)),
        (_, Some(_)) => Ok((spanned_token, false)),
    }
}

fn get_define_name<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span<'s>,
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
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![Expectation::Label("a preprocessor macro name")],
        })),
    }
}

fn get_define_function_args<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    define_span: Span<'s>,
) -> Result<
    Option<
        Vec<(
            SpannedString<'s>,
            Option<(
                Span<'s>, // =
                Vec<SpannedToken<'s>>,
            )>,
        )>,
    >,
    PreprocessorError<'s>,
> {
    let Some(spanned_token) = src.peek() else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        Token::Paren => {
            let paren_span = src.next().unwrap().1;
            let mut function_args: Vec<(
                SpannedString<'s>,
                Option<(Span, Vec<SpannedToken<'s>>)>,
            )> = vec![];
            'get_args: loop {
                let mut next_arg_token = match get_define_function_arg(
                    src,
                    &mut function_args,
                    configs,
                    paren_span.clone(),
                ) {
                    Ok(()) => {
                        let (next_token, eod) =
                            get_define_token(src, paren_span.clone())?;
                        if eod {
                            return Err(
                                PreprocessorError::IncompleteDirectiveWithToken(
                                    next_token,
                                ),
                            );
                        } else {
                            next_token
                        }
                    }
                    Err(PreprocessorError::EndOfFunctionArgument(
                        next_token,
                    )) => next_token,
                    Err(err) => {
                        return Err(err);
                    }
                };
                loop {
                    match next_arg_token {
                        SpannedToken(Token::EParen, _) => {
                            break 'get_args Ok(Some(function_args));
                        }
                        SpannedToken(Token::Comma, _) => {
                            continue 'get_args;
                        }
                        SpannedToken(Token::Newline, _) => {
                            let (next_token, eod) =
                                get_define_token(src, paren_span.clone())?;
                            next_arg_token = if eod {
                                return Err(
                                    PreprocessorError::IncompleteDirectiveWithToken(
                                        next_token,
                                    ),
                                );
                            } else {
                                next_token
                            }
                        }
                        SpannedToken(_, _) => {
                            break 'get_args Err(
                                PreprocessorError::InvalidDefineArgument(
                                    next_arg_token,
                                ),
                            );
                        }
                    }
                }
            }
        }
        _ => Ok(None),
    }
}

fn get_define_function_arg<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<(
        SpannedString<'s>,
        Option<(Span<'s>, Vec<SpannedToken<'s>>)>,
    )>,
    configs: &mut PreprocessConfigs<'s>,
    paren_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let arg_id = loop {
        match get_define_token(src, paren_span.clone())? {
            (err_token, true) => {
                return Err(PreprocessorError::IncompleteDirectiveWithToken(
                    err_token,
                ));
            }
            (SpannedToken(Token::SimpleIdentifier(id_str), id_span), _) => {
                break SpannedString(id_str, id_span);
            }
            (SpannedToken(Token::Newline, _), _) => {
                continue;
            }
            (err_spanned_token, _) => {
                return Err(PreprocessorError::InvalidDefineArgument(
                    err_spanned_token,
                ));
            }
        }
    };
    let eq_span = loop {
        match src.peek() {
            None => {
                return Err(PreprocessorError::IncompleteDirective(paren_span));
            }
            Some(SpannedToken(Token::Eq, _)) => {
                let SpannedToken(_, eq_span) = src.next().unwrap();
                break eq_span;
            }
            Some(_) => {
                dest.push((arg_id, None));
                return Ok(());
            }
        }
    };
    let mut default_arg_text: Vec<SpannedToken<'s>> = vec![];
    configs.in_define = true;
    configs.in_define_arg = true;
    let result = preprocess(src, &mut Some(&mut default_arg_text), configs);
    configs.in_define = false;
    configs.in_define_arg = false;
    match result {
        Ok(()) => Err(PreprocessorError::IncompleteDirective(paren_span)),
        Err(PreprocessorError::EndOfFunctionArgument(spanned_token)) => {
            dest.push((arg_id, Some((eq_span, default_arg_text))));
            Err(PreprocessorError::EndOfFunctionArgument(spanned_token))
        }
        Err(err) => Err(err),
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
    define_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let define_name = get_define_name(src, define_span.clone())?;
    let function_args = get_define_function_args(src, configs, define_span)?;
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

pub fn preprocess_undefine<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    define_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let undefine_name = get_define_name(src, define_span)?;
    if !configs.undefine(undefine_name.0) {
        Err(PreprocessorError::NotPreviouslyDefinedMacro((
            undefine_name.0,
            undefine_name.1,
        )))
    } else {
        Ok(())
    }
}
