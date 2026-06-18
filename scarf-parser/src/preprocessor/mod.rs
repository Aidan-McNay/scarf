// =======================================================================
// mod.rs
// =======================================================================
//! Preprocessing a token stream, elaborating compiler directives

pub mod cache;
pub(crate) mod conditional_compilation;
pub(crate) mod define;
pub(crate) mod error;
pub(crate) mod implicit_nettype;
pub(crate) mod include;
pub(crate) mod keywords;
pub(crate) mod line;
pub mod state;
pub(crate) mod text_macro;
pub(crate) mod timescale;
pub(crate) mod unconnected;
use crate::*;
pub use cache::*;
use conditional_compilation::*;
use define::*;
pub use error::*;
pub(crate) use implicit_nettype::DefaultNettype;
use implicit_nettype::*;
use include::*;
use keywords::*;
use line::*;
pub use state::*;
use std::collections::VecDeque;
use text_macro::*;
use timescale::*;
pub(crate) use timescale::{Timescale, TimescaleUnit, TimescaleValue};
pub(crate) use unconnected::UnconnectedDrive;
use unconnected::*;

/// A peekable, extendable iterator over tokens.
///
/// This iterator extends `<T>` by keeping track of an additional
/// stack of tokens at the front, allowing users to peek the next
/// token, as well as push tokens to be iterated on next (such as
/// when expanding a preprocessor definition)
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

    pub fn prepend_tokens<I>(&mut self, extra_tokens: I)
    where
        I: Iterator<Item = SpannedToken<'s>>
            + ExactSizeIterator
            + std::iter::DoubleEndedIterator,
    {
        self.extras.reserve(extra_tokens.len());
        for extra_token in extra_tokens.rev() {
            self.extras.push_front(extra_token);
        }
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

pub(crate) fn preprocess_helper<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let mut enclosures: Vec<Token<'s>> = vec![];
    if state.in_define() || state.in_define_arg() {
        while let Some(mut spanned_token) = src.next() {
            match spanned_token.0 {
                Token::Bslash => loop {
                    match src.next() {
                        None => dest.push(spanned_token),
                        Some(next_token) => match next_token.0 {
                            Token::Newline => (),
                            Token::Bslash => {
                                dest.push(spanned_token);
                                spanned_token = next_token;
                                continue;
                            }
                            _ => {
                                dest.push(spanned_token);
                                dest.push(next_token)
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
                Token::Paren if state.in_define_arg() => {
                    enclosures.push(Token::Paren);
                    dest.push(spanned_token);
                }
                Token::Bracket if state.in_define_arg() => {
                    enclosures.push(Token::Bracket);
                    dest.push(spanned_token);
                }
                Token::Brace if state.in_define_arg() => {
                    enclosures.push(Token::Brace);
                    dest.push(spanned_token);
                }
                Token::EParen if state.in_define_arg() => {
                    match enclosures.pop() {
                        Some(Token::Paren) => dest.push(spanned_token),
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
                Token::EBracket if state.in_define_arg() => {
                    match enclosures.pop() {
                        Some(Token::Bracket) => dest.push(spanned_token),
                        _ => {
                            return Err(
                                PreprocessorError::IncompleteMacroWithToken(
                                    spanned_token,
                                ),
                            );
                        }
                    }
                }
                Token::EBrace if state.in_define_arg() => {
                    match enclosures.pop() {
                        Some(Token::Brace) => dest.push(spanned_token),
                        _ => {
                            return Err(
                                PreprocessorError::IncompleteMacroWithToken(
                                    spanned_token,
                                ),
                            );
                        }
                    }
                }
                Token::Comma if state.in_define_arg() => {
                    if enclosures.is_empty() {
                        return Err(PreprocessorError::EndOfFunctionArgument(
                            spanned_token,
                        ));
                    } else {
                        dest.push(spanned_token)
                    }
                }
                Token::BlockComment(_) | Token::OnelineComment(_) => (),
                Token::TextMacro(macro_name) if state.in_define_arg() => {
                    preprocess_macro(
                        src,
                        state,
                        cache,
                        (macro_name, spanned_token.1),
                    )?;
                }
                _ => dest.push(spanned_token),
            }
        }
        Ok(())
    } else {
        while let Some(spanned_token) = src.next() {
            match spanned_token.0 {
                Token::DirResetall => {
                    state.reset_all(spanned_token.1);
                }
                Token::DirInclude => {
                    let include_span = cache.retain_span(spanned_token.1);
                    preprocess_include(src, dest, state, cache, include_span)?;
                }
                Token::DirUndefineall => {
                    state.undefineall();
                }
                Token::DirBeginKeywords => {
                    preprocess_keyword_standard(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token.1,
                    )?;
                }
                Token::DirDefine => {
                    preprocess_define(src, state, cache, spanned_token.1)?;
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
                        state,
                        cache,
                        spanned_token.1,
                        true,
                    )?;
                }
                Token::DirIfndef => {
                    preprocess_ifdef(
                        src,
                        dest,
                        state,
                        cache,
                        spanned_token.1,
                        false,
                    )?;
                }
                Token::TextMacro(macro_name) => {
                    preprocess_macro(
                        src,
                        state,
                        cache,
                        (macro_name, spanned_token.1),
                    )?;
                }
                Token::DirUndef => {
                    preprocess_undefine(src, state, spanned_token.1)?;
                }
                Token::DirTimescale => {
                    preprocess_timescale(src, state, cache, spanned_token.1)?;
                }
                Token::DirDefaultNettype => {
                    preprocess_default_nettype(
                        src,
                        state,
                        cache,
                        spanned_token.1,
                    )?;
                }
                Token::DirUnconnectedDrive => {
                    preprocess_unconnected_drive(src, state, spanned_token.1)?;
                }
                Token::DirNounconnectedDrive => {
                    preprocess_nounconnected_drive(state, spanned_token.1)?;
                }
                Token::DirCelldefine => {
                    state.add_cell_define(true, spanned_token.1);
                }
                Token::DirEndcelldefine => {
                    state.add_cell_define(false, spanned_token.1);
                }
                Token::DirLine => {
                    preprocess_line(src, state, cache, spanned_token.1)?;
                }
                Token::DirUnderscoreFile => dest.push(SpannedToken(
                    Token::StringLiteral(
                        state.get_line_directive_file(&spanned_token.1),
                    ),
                    spanned_token.1,
                )),
                Token::DirUnderscoreLine => dest.push(SpannedToken(
                    Token::UnsignedNumber(
                        state.get_line_directive_line(&spanned_token.1, cache),
                    ),
                    spanned_token.1,
                )),
                Token::BlockComment(_)
                | Token::OnelineComment(_)
                | Token::Newline => {
                    #[cfg(feature = "parse_lossless")]
                    {
                        dest.push(spanned_token)
                    }
                }
                token if token.keyword_replace(&state.curr_standard) => {
                    let new_token = SpannedToken(
                        Token::SimpleIdentifier(token.as_str()),
                        spanned_token.1,
                    );
                    dest.push(new_token)
                }
                _ => dest.push(spanned_token),
            }
        }
        Ok(())
    }
}

pub(crate) fn preprocess_single<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<Option<SpannedToken<'s>>, PreprocessorError<'s>> {
    loop {
        match src.next() {
            None => {
                break Ok(None);
            }
            Some(SpannedToken(Token::BlockComment(_), _)) => (),
            Some(SpannedToken(Token::TextMacro(macro_name), macro_span)) => {
                preprocess_macro(src, state, cache, (macro_name, macro_span))?;
            }
            other => break Ok(other),
        }
    }
}

pub fn preprocess<'s>(
    src: impl Iterator<Item = SpannedToken<'s>>,
    state: &mut PreprocessorState<'s>,
    cache: &'s PreprocessorCache<'s>,
) -> Result<Vec<SpannedToken<'s>>, PreprocessorError<'s>> {
    let mut token_iter = TokenIterator::new(src);
    let mut dest = Vec::new();
    preprocess_helper(&mut token_iter, &mut dest, state, cache)?;
    Ok(dest)
}
