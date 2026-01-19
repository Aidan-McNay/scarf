// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the preprocessor

pub mod conditional_compilation;
pub mod configs;
pub mod define;
pub mod error;
pub mod implicit_nettype;
pub mod include;
pub mod keywords;
pub mod line;
pub mod text_macro;
pub mod timescale;
pub mod unconnected;
use crate::*;
pub use conditional_compilation::*;
pub use configs::*;
pub use define::*;
pub use error::*;
pub use implicit_nettype::*;
pub use include::*;
pub use keywords::*;
pub use line::*;
use std::collections::VecDeque;
pub use text_macro::*;
pub use timescale::*;
pub use unconnected::*;

pub(crate) trait Pushable<T> {
    fn push_element(&mut self, item: T);
    fn reserve(&mut self, additional: usize);
}

impl<T> Pushable<T> for Option<&mut Vec<T>> {
    fn push_element(&mut self, item: T) {
        if let Some(inner_vec) = self {
            inner_vec.push(item);
        }
    }
    fn reserve(&mut self, additional: usize) {
        if let Some(inner_vec) = self {
            inner_vec.reserve(additional);
        }
    }
}

pub struct TokenIterator<'s, T: Iterator<Item = SpannedToken<'s>>> {
    iter: T,
    extras: VecDeque<SpannedToken<'s>>,
}

impl<'s, T: Iterator<Item = SpannedToken<'s>>> Iterator
    for TokenIterator<'s, T>
{
    type Item = SpannedToken<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(extra_token) = self.extras.pop_front() {
            Some(extra_token)
        } else {
            self.iter.next()
        }
    }
}

impl<'s, T: Iterator<Item = SpannedToken<'s>>> TokenIterator<'s, T> {
    pub fn new(iter: T) -> Self {
        Self {
            iter,
            extras: VecDeque::default(),
        }
    }

    pub fn add_tokens<I>(&mut self, extra_tokens: I)
    where
        I: Iterator<Item = SpannedToken<'s>>,
    {
        self.extras.extend(extra_tokens)
    }

    pub fn peek(&mut self) -> Option<&SpannedToken<'s>> {
        if self.extras.is_empty() {
            if let Some(next_token) = self.iter.next() {
                self.extras.push_back(next_token);
            }
        }
        self.extras.front()
    }
}

pub fn preprocess<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut enclosures: Vec<Token<'s>> = vec![];
    if configs.in_define || configs.in_define_arg {
        while let Some(mut spanned_token) = src.next() {
            match spanned_token.0 {
                Token::Bslash => loop {
                    match src.next() {
                        None => dest.push_element(spanned_token),
                        Some(next_token) => match next_token.0 {
                            Token::Newline => (),
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
                Token::Newline => {
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
                Token::EParen if configs.in_define_arg => {
                    match enclosures.pop() {
                        Some(Token::Paren) => dest.push_element(spanned_token),
                        None => {
                            return Err(
                                PreprocessorError::EndOfFunctionArgument(
                                    spanned_token,
                                ),
                            );
                        }
                        _ => {
                            return Err(
                                PreprocessorError::IncompleteMacroWithToken(
                                    spanned_token,
                                ),
                            );
                        }
                    }
                }
                Token::EBracket if configs.in_define_arg => {
                    match enclosures.pop() {
                        Some(Token::Bracket) => {
                            dest.push_element(spanned_token)
                        }
                        _ => {
                            return Err(
                                PreprocessorError::IncompleteMacroWithToken(
                                    spanned_token,
                                ),
                            );
                        }
                    }
                }
                Token::EBrace if configs.in_define_arg => {
                    match enclosures.pop() {
                        Some(Token::Brace) => dest.push_element(spanned_token),
                        _ => {
                            return Err(
                                PreprocessorError::IncompleteMacroWithToken(
                                    spanned_token,
                                ),
                            );
                        }
                    }
                }
                Token::Comma if configs.in_define_arg => {
                    if enclosures.is_empty() {
                        return Err(PreprocessorError::EndOfFunctionArgument(
                            spanned_token,
                        ));
                    } else {
                        dest.push_element(spanned_token)
                    }
                }
                Token::BlockComment(_) | Token::OnelineComment(_) => (),
                Token::TextMacro(macro_name) if configs.in_define_arg => {
                    preprocess_macro(
                        src,
                        configs,
                        (macro_name, spanned_token.1),
                    )?;
                }
                _ => dest.push_element(spanned_token),
            }
        }
        Ok(())
    } else {
        while let Some(spanned_token) = src.next() {
            match spanned_token.0 {
                Token::DirResetall => {
                    configs.reset_all(spanned_token.1);
                }
                Token::DirInclude => {
                    let include_span = configs.retain_span(spanned_token.1);
                    preprocess_include(src, dest, configs, include_span)?;
                }
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
                    return Err(PreprocessorError::EndKeywords(
                        spanned_token.1,
                    ));
                }
                Token::DirEndif => {
                    return Err(PreprocessorError::Endif(spanned_token.1));
                }
                Token::DirIfdef => {
                    preprocess_ifdef(
                        src,
                        dest,
                        configs,
                        spanned_token.1,
                        true,
                    )?;
                }
                Token::DirIfndef => {
                    preprocess_ifdef(
                        src,
                        dest,
                        configs,
                        spanned_token.1,
                        false,
                    )?;
                }
                Token::TextMacro(macro_name) => {
                    preprocess_macro(
                        src,
                        configs,
                        (macro_name, spanned_token.1),
                    )?;
                }
                Token::DirUndef => {
                    preprocess_undefine(src, configs, spanned_token.1)?;
                }
                Token::DirTimescale => {
                    preprocess_timescale(src, configs, spanned_token.1)?;
                }
                Token::DirDefaultNettype => {
                    preprocess_default_nettype(src, configs, spanned_token.1)?;
                }
                Token::DirUnconnectedDrive => {
                    preprocess_unconnected_drive(
                        src,
                        configs,
                        spanned_token.1,
                    )?;
                }
                Token::DirNounconnectedDrive => {
                    preprocess_nounconnected_drive(configs, spanned_token.1)?;
                }
                Token::DirCelldefine => {
                    configs.add_cell_define(true, spanned_token.1);
                }
                Token::DirEndcelldefine => {
                    configs.add_cell_define(false, spanned_token.1);
                }
                Token::DirLine => {
                    preprocess_line(src, configs, spanned_token.1)?;
                }
                Token::DirUnderscoreFile => dest.push_element(SpannedToken(
                    Token::StringLiteral(
                        configs.get_line_directive_file(&spanned_token.1),
                    ),
                    spanned_token.1,
                )),
                Token::DirUnderscoreLine => dest.push_element(SpannedToken(
                    Token::UnsignedNumber(
                        configs.get_line_directive_line(&spanned_token.1),
                    ),
                    spanned_token.1,
                )),
                Token::BlockComment(_)
                | Token::OnelineComment(_)
                | Token::Newline => {
                    #[cfg(feature = "parse_extras")]
                    {
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
}

pub(crate) fn preprocess_single<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<Option<SpannedToken<'s>>, PreprocessorError<'s>> {
    loop {
        match src.next() {
            None => {
                break Ok(None);
            }
            Some(SpannedToken(Token::BlockComment(_), _)) => (),
            Some(SpannedToken(Token::TextMacro(macro_name), macro_span)) => {
                preprocess_macro(src, configs, (macro_name, macro_span))?;
            }
            other => break Ok(other),
        }
    }
}
