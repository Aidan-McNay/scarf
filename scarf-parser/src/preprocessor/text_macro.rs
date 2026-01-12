// =======================================================================
// text_macro.rs
// =======================================================================
// Preprocessing for macro instantiations

use crate::*;
use std::collections::HashMap;
use std::iter::Peekable;
use std::vec::IntoIter;

fn get_text_macro_args<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<Vec<Vec<SpannedToken<'s>>>, PreprocessorError<'s>> {
    let paren_span = loop {
        match src.next() {
            Some(SpannedToken(Token::Paren, paren_span)) => {
                break paren_span;
            }
            Some(SpannedToken(Token::Newline, _)) => (),
            _ => {
                return Err(PreprocessorError::NoMacroArguments(text_macro));
            }
        }
    };
    let mut arg_vec: Vec<Vec<SpannedToken<'s>>> = vec![];
    loop {
        let mut new_arg: Vec<SpannedToken<'s>> = vec![];
        configs.in_define_arg = true;
        let result = preprocess(src, &mut Some(&mut new_arg), configs);
        configs.in_define_arg = false;
        match result {
            Ok(()) => {
                return Err(PreprocessorError::IncompleteDirective(paren_span));
            }
            Err(PreprocessorError::EndOfFunctionArgument(SpannedToken(
                Token::EParen,
                _,
            ))) => {
                arg_vec.push(new_arg);
                break;
            }
            Err(PreprocessorError::EndOfFunctionArgument(SpannedToken(
                Token::Comma,
                _,
            ))) => {
                arg_vec.push(new_arg);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(arg_vec)
}

fn resolve_text_macro_args<'s>(
    specified_args: Vec<Vec<SpannedToken<'s>>>,
    original_args: Vec<(SpannedString<'s>, Option<Vec<SpannedToken<'s>>>)>,
    text_macro: (&'s str, Span<'s>),
) -> Result<
    HashMap<&'s str, (Span<'s>, Vec<SpannedToken<'s>>)>,
    PreprocessorError<'s>,
> {
    if specified_args.len() > original_args.len() {
        return Err(PreprocessorError::TooManyMacroArguments((
            text_macro.0,
            original_args.len(),
            specified_args.len(),
            text_macro.1,
        )));
    }
    let mut specified_args_iter = specified_args.into_iter();
    let mut resolved_args = HashMap::new();
    for (arg_name, arg_tokens) in original_args.into_iter() {
        match specified_args_iter.next() {
            Some(specified_tokens) => {
                resolved_args
                    .insert(arg_name.0, (arg_name.1, specified_tokens));
            }
            None => match arg_tokens {
                Some(default_tokens) => {
                    resolved_args
                        .insert(arg_name.0, (arg_name.1, default_tokens));
                }
                None => {
                    return Err(PreprocessorError::MissingMacroArgument((
                        arg_name.0,
                        text_macro.1,
                    )));
                }
            },
        }
    }
    Ok(resolved_args)
}

fn replace_macro_arguments<'a>(
    original_stream: IntoIter<SpannedToken<'a>>,
    arguments: HashMap<&'a str, (Span<'a>, Vec<SpannedToken<'a>>)>,
    configs: &mut PreprocessConfigs<'a>,
) -> Result<Vec<SpannedToken<'a>>, PreprocessorError<'a>> {
    let mut result_vec = vec![];
    for token in original_stream {
        match token {
            SpannedToken(Token::SimpleIdentifier(id), _)
                if arguments.contains_key(id) =>
            {
                let (_, replacement_tokens) = arguments.get(id).unwrap();
                result_vec.extend(replacement_tokens.clone());
            }
            SpannedToken(Token::PreprocessorIdentifier(id), span) => {
                let components = id.split("``");
                let mut resulting_identifier = "".to_string();
                for component in components.into_iter() {
                    match arguments.get(component) {
                        Some((_, replacement_tokens)) => {
                            let replacement_tokens_len =
                                replacement_tokens.len();
                            if replacement_tokens_len > 1 {
                                // Only currently support a max of one token
                                let err_span = replacement_tokens
                                    .first()
                                    .unwrap()
                                    .1
                                    .clone();
                                return Err(PreprocessorError::InvalidIdentifierFormation((component, err_span)));
                            } else if replacement_tokens_len == 1 {
                                let replacement_token =
                                    replacement_tokens.first().unwrap().clone();
                                match replacement_token.0 {
                                    Token::UnsignedNumber(text)
                                    | Token::FixedPointNumber(text)
                                    | Token::BinaryNumber(text)
                                    | Token::OctalNumber(text)
                                    | Token::DecimalNumber(text)
                                    | Token::HexNumber(text)
                                    | Token::ScientificNumber(text)
                                    | Token::UnbasedUnsizedLiteral(text)
                                    | Token::SystemTfIdentifier(text)
                                    | Token::SimpleIdentifier(text)
                                    | Token::EscapedIdentifier(text)
                                    | Token::TimeUnit(text) => {
                                        resulting_identifier += text;
                                    }
                                    Token::OnelineComment(_)
                                    | Token::BlockComment(_)
                                    | Token::PreprocessorIdentifier(_)
                                    | Token::TextMacro(_)
                                    | Token::StringLiteral(_)
                                    | Token::TripleQuoteStringLiteral(_)
                                    | Token::DirIncludeToolPath(_)
                                    | Token::Newline => {
                                        return Err(PreprocessorError::InvalidIdentifierFormation((component, replacement_token.1)));
                                    }
                                    other => {
                                        resulting_identifier += other.as_str()
                                    }
                                }
                            }
                        }
                        None => {
                            resulting_identifier += component;
                        }
                    }
                }
                result_vec.push(SpannedToken(
                    Token::SimpleIdentifier(
                        configs.retain_string(resulting_identifier),
                    ),
                    span,
                ));
            }
            other_token => {
                result_vec.push(other_token);
            }
        }
    }
    Ok(result_vec)
}

pub fn preprocess_macro<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(), PreprocessorError<'s>> {
    match configs.get_macro_tokens(text_macro.0) {
        Some((mut token_vec, define_args)) => {
            if let Some(define_args) = define_args {
                let function_args =
                    get_text_macro_args(src, configs, text_macro.clone())?;
                token_vec = replace_macro_arguments(
                    token_vec.into_iter(),
                    resolve_text_macro_args(
                        function_args,
                        define_args,
                        text_macro,
                    )?,
                    configs,
                )?;
            }
            preprocess(&mut token_vec.into_iter().peekable(), dest, configs)
        }
        None => Err(PreprocessorError::UndefinedMacro(text_macro)),
    }
}
