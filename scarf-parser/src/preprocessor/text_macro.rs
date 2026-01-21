// =======================================================================
// text_macro.rs
// =======================================================================
// Preprocessing for macro instantiations

use crate::*;
use std::collections::HashMap;
use std::vec::IntoIter;

fn get_text_macro_args<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    define_span: &Span<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(Vec<Vec<SpannedToken<'s>>>, Span<'s>), PreprocessorError<'s>> {
    let paren_span = loop {
        match src.next() {
            Some(SpannedToken(Token::Paren, paren_span)) => {
                break paren_span;
            }
            Some(SpannedToken(Token::Newline, _)) => (),
            _ => {
                return Err(PreprocessorError::NoMacroArguments((
                    define_span.clone(),
                    text_macro,
                )));
            }
        }
    };
    let mut arg_vec: Vec<Vec<SpannedToken<'s>>> = vec![];
    let end_span = loop {
        let mut new_arg: Vec<SpannedToken<'s>> = vec![];
        configs.in_define_arg = true;
        let result = preprocess(src, &mut new_arg, configs);
        configs.in_define_arg = false;
        match result {
            Ok(()) => {
                return Err(PreprocessorError::IncompleteDirective(paren_span));
            }
            Err(PreprocessorError::EndOfFunctionArgument(SpannedToken(
                Token::EParen,
                eparen_span,
            ))) => {
                arg_vec.push(new_arg);
                break eparen_span;
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
    };
    let mut overall_span = text_macro.1;
    overall_span.bytes.end = end_span.bytes.end;
    Ok((arg_vec, overall_span))
}

fn resolve_text_macro_args<'s>(
    specified_args: Vec<Vec<SpannedToken<'s>>>,
    original_args: Vec<(SpannedString<'s>, Option<Vec<SpannedToken<'s>>>)>,
    define_span: &Span<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<
    HashMap<&'s str, (Span<'s>, Vec<SpannedToken<'s>>)>,
    PreprocessorError<'s>,
> {
    if specified_args.len() > original_args.len() {
        return Err(PreprocessorError::TooManyMacroArguments((
            define_span.clone(),
            (
                text_macro.0,
                original_args.len(),
                specified_args.len(),
                text_macro.1,
            ),
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
                        define_span.clone(),
                        (arg_name.0, text_macro.1),
                    )));
                }
            },
        }
    }
    Ok(resolved_args)
}

fn get_identifier_substitute<'a>(
    arg_name: &'a str,
    replacement_tokens: &Vec<SpannedToken<'a>>,
) -> Result<&'a str, PreprocessorError<'a>> {
    let replacement_tokens_len = replacement_tokens.len();
    if replacement_tokens_len > 1 {
        // Only currently support a max of one token
        let err_span = replacement_tokens.first().unwrap().1.clone();
        return Err(PreprocessorError::InvalidIdentifierFormation((
            arg_name, err_span,
        )));
    } else if replacement_tokens_len == 1 {
        let replacement_token = replacement_tokens.first().unwrap().clone();
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
            | Token::EscapedIdentifier(text) => Ok(text),
            Token::OnelineComment(_)
            | Token::BlockComment(_)
            | Token::PreprocessorIdentifier(_)
            | Token::TextMacro(_)
            | Token::StringLiteral(_)
            | Token::TripleQuoteStringLiteral(_)
            | Token::Newline => {
                return Err(PreprocessorError::InvalidIdentifierFormation((
                    arg_name,
                    replacement_token.1,
                )));
            }
            other => Ok(other.as_str()),
        }
    } else {
        Ok("")
    }
}

fn get_string_substitute<'a>(
    id_name: &'a str,
    replacement_tokens: &Vec<SpannedToken<'a>>,
    str_to_append: &mut String,
) -> Result<(), PreprocessorError<'a>> {
    let replacement_tokens_len = replacement_tokens.len();
    if replacement_tokens_len > 1 {
        // Only currently support a max of one token
        let err_span = replacement_tokens.first().unwrap().1.clone();
        return Err(PreprocessorError::InvalidIdentifierFormation((
            id_name, err_span,
        )));
    } else if replacement_tokens_len == 1 {
        let replacement_token = replacement_tokens.first().unwrap().clone();
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
            | Token::EscapedIdentifier(text) => {
                *str_to_append += text;
                Ok(())
            }
            Token::StringLiteral(text) => {
                *str_to_append += "\"";
                *str_to_append += text;
                *str_to_append += "\"";
                Ok(())
            }
            Token::TripleQuoteStringLiteral(text) => {
                *str_to_append += "\"\"\"";
                *str_to_append += text;
                *str_to_append += "\"\"\"";
                Ok(())
            }
            Token::OnelineComment(_)
            | Token::BlockComment(_)
            | Token::PreprocessorIdentifier(_)
            | Token::TextMacro(_)
            | Token::Newline => {
                return Err(PreprocessorError::InvalidIdentifierFormation((
                    id_name,
                    replacement_token.1,
                )));
            }
            other => {
                *str_to_append += other.as_str();
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

fn get_replacement_string<'a>(
    mut initial_string: String,
    configs: &PreprocessConfigs<'a>,
    arguments: &HashMap<&'a str, (Span<'a>, Vec<SpannedToken<'a>>)>,
) -> Result<String, PreprocessorError<'a>> {
    // Replace definitions
    for define in configs.defines() {
        let define_id = format! {"`{}", define.name.0};
        if initial_string.contains(&define_id) {
            let replacement_string = match &define.body {
                DefineBody::Empty => "".to_string(),
                DefineBody::Text(tokens) => {
                    let mut token_str = "".to_string();
                    get_string_substitute(
                        define.name.0,
                        tokens,
                        &mut token_str,
                    )?;
                    token_str
                }
                DefineBody::Function(_) => {
                    todo!("Error for using functions here")
                }
            };
            initial_string =
                initial_string.replace(&define_id, replacement_string.as_str());
        }
    }
    // Replace arguments
    for argument in arguments.keys() {
        if initial_string.contains(argument) {
            let mut replacement_string = "".to_string();
            get_string_substitute(
                argument,
                &arguments.get(argument).unwrap().1,
                &mut replacement_string,
            )?;
            initial_string =
                initial_string.replace(argument, replacement_string.as_str())
        }
    }
    Ok(initial_string)
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
                            resulting_identifier += get_identifier_substitute(
                                component,
                                replacement_tokens,
                            )?;
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
            SpannedToken(Token::PreprocessorStringLiteral(id), span) => {
                result_vec.push(SpannedToken(
                    Token::StringLiteral(configs.retain_string(
                        get_replacement_string(
                            id.to_string(),
                            configs,
                            &arguments,
                        )?,
                    )),
                    span,
                ));
            }
            SpannedToken(
                Token::PreprocessorTripleQuoteStringLiteral(id),
                span,
            ) => {
                result_vec.push(SpannedToken(
                    Token::TripleQuoteStringLiteral(configs.retain_string(
                        get_replacement_string(
                            id.to_string(),
                            configs,
                            &arguments,
                        )?,
                    )),
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

struct SpanReplacer<'a> {
    text_macro_span: &'a Span<'a>,
    tokens: IntoIter<SpannedToken<'a>>,
}

impl<'a> Iterator for SpanReplacer<'a> {
    type Item = SpannedToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.next() {
            Some(token) => Some(self.update_span(token)),
            None => None,
        }
    }
}

impl<'a> SpanReplacer<'a> {
    fn new(
        text_macro_span: &'a Span<'a>,
        tokens: IntoIter<SpannedToken<'a>>,
    ) -> Self {
        Self {
            text_macro_span,
            tokens,
        }
    }

    fn update_span(&self, mut token: SpannedToken<'a>) -> SpannedToken<'a> {
        token.1.expanded_from = Some(self.text_macro_span);
        token
    }
}

pub fn preprocess_macro<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    text_macro: (&'s str, Span<'s>),
) -> Result<(), PreprocessorError<'s>> {
    match configs.get_macro_tokens(text_macro.0) {
        Some((define_span, (mut token_vec, define_args))) => {
            let mut macro_span = text_macro.1.clone();
            if let Some(define_args) = define_args {
                let (function_args, function_macro_span) = get_text_macro_args(
                    src,
                    configs,
                    &define_span,
                    text_macro.clone(),
                )?;
                macro_span = function_macro_span;
                token_vec = replace_macro_arguments(
                    token_vec.into_iter(),
                    resolve_text_macro_args(
                        function_args,
                        define_args,
                        &define_span,
                        text_macro,
                    )?,
                    configs,
                )?;
            }
            let token_iter = SpanReplacer::new(
                configs.retain_span(macro_span),
                token_vec.into_iter(),
            );
            src.add_tokens(token_iter);
            Ok(())
        }
        None => Err(PreprocessorError::UndefinedMacro(text_macro)),
    }
}
