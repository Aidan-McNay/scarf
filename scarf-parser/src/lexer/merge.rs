// =======================================================================
// merge.rs
// =======================================================================
// Merge lexed tokens that represent one syntactic token

use crate::*;
use logos::Span as ByteSpan;
use logos::{Lexer, SpannedIter};

#[derive(Clone)]
pub struct TokenMerge<'s> {
    tokens: SpannedIter<'s, Token<'s>>,
    prev_was_num: bool,
    prev_include_span: Option<ByteSpan>,
    text: &'s str,
}

impl<'s> TokenMerge<'s> {
    pub fn new(src_lexer: Lexer<'s, Token<'s>>, src_text: &'s str) -> Self {
        Self {
            tokens: src_lexer.spanned(),
            prev_was_num: false,
            prev_include_span: None,
            text: src_text,
        }
    }

    fn get_include_path(
        &mut self,
        include_span: ByteSpan,
    ) -> (Result<Token<'s>, String>, ByteSpan) {
        let start_span = match self.tokens.next() {
            Some((Ok(Token::SimpleIdentifier(id_text)), id_span)) => {
                return (Ok(Token::SimpleIdentifier(id_text)), id_span);
            }
            Some((Ok(Token::Lt), lt_span)) => lt_span,
            Some((other_token, err_span)) => {
                return (other_token, err_span); // Deal with at the preprocessor
            }
            _ => return (Err("No include path".to_owned()), include_span),
        };
        loop {
            match self.tokens.next() {
                Some((Ok(Token::Gt), end_span)) => {
                    let include_path_text =
                        &self.text[start_span.end..end_span.start];
                    let overall_span = ByteSpan {
                        start: start_span.start,
                        end: end_span.end,
                    };
                    return (
                        Ok(Token::DirIncludeToolPath(include_path_text)),
                        overall_span,
                    );
                }
                Some((Ok(Token::Newline), _)) | None => {
                    return (
                        Err("Include path with no ending".to_owned()),
                        start_span,
                    );
                }
                _ => (),
            }
        }
    }
}

impl<'s> Iterator for TokenMerge<'s> {
    type Item = (Result<Token<'s>, String>, ByteSpan);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(include_span) = &self.prev_include_span {
            let result = Some(self.get_include_path(include_span.clone()));
            self.prev_include_span = None;
            return result;
        }
        match self.tokens.next() {
            Some((Ok(Token::UnsignedNumber(num_str)), num_span)) => {
                self.prev_was_num = true;
                Some((Ok(Token::UnsignedNumber(num_str)), num_span))
            }
            Some((Ok(Token::FixedPointNumber(num_str)), num_span)) => {
                self.prev_was_num = true;
                Some((Ok(Token::FixedPointNumber(num_str)), num_span))
            }
            Some((Ok(Token::DirInclude), include_span)) => {
                self.prev_was_num = false;
                self.prev_include_span = Some(include_span.clone());
                Some((Ok(Token::DirInclude), include_span))
            }
            Some((Ok(Token::SimpleIdentifier(text)), span)) => {
                if self.prev_was_num {
                    self.prev_was_num = false;
                    match text {
                        "s" | "ms" | "us" | "ns" | "ps" | "fs" => {
                            Some((Ok(Token::TimeUnit(text)), span))
                        }
                        _ => Some((Ok(Token::SimpleIdentifier(text)), span)),
                    }
                } else {
                    Some((Ok(Token::SimpleIdentifier(text)), span))
                }
            }
            other => {
                self.prev_was_num = false;
                other
            }
        }
    }
}
