// =======================================================================
// text_macro.rs
// =======================================================================
// Preprocessing for macro instantiations

use crate::*;
use std::iter::Peekable;

pub fn get_text_macro_args<'s>(
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

pub fn preprocess_macro<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(), PreprocessorError<'s>> {
    match configs.get_macro_tokens(text_macro.0) {
        Some((token_vec, is_function)) => {
            if is_function {
                let _function_args =
                    get_text_macro_args(src, configs, text_macro)?;
            }
            preprocess(&mut token_vec.into_iter().peekable(), dest, configs)
        }
        None => Err(PreprocessorError::UndefinedMacro(text_macro)),
    }
}
