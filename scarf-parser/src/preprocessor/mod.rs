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
use std::iter::Peekable;
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

pub fn preprocess<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut enclosures: Vec<Token<'s>> = vec![];
    if configs.in_define || configs.in_define_arg {
        while let Some(mut spanned_token) = src.next() {
            match spanned_token.0 {
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
                Token::TextMacro(macro_name) if configs.in_define_arg => {
                    preprocess_macro(
                        src,
                        dest,
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
                        dest,
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
                Token::DirUnderscoreFile => dest.push_element(SpannedToken(
                    Token::SimpleIdentifier(configs.get_file(&spanned_token.1)),
                    spanned_token.1,
                )),
                Token::DirUnderscoreLine => dest.push_element(SpannedToken(
                    Token::UnsignedNumber(configs.get_line(&spanned_token.1)),
                    spanned_token.1,
                )),
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
