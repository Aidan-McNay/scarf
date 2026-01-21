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
    prev_include_span: Option<ByteSpan>,
    text: &'s str,
}

impl<'s> TokenMerge<'s> {
    pub fn new(src_lexer: Lexer<'s, Token<'s>>, src_text: &'s str) -> Self {
        Self {
            tokens: src_lexer.spanned(),
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
            Some((Ok(Token::DirInclude), include_span)) => {
                self.prev_include_span = Some(include_span.clone());
                Some((Ok(Token::DirInclude), include_span))
            }
            other => other,
        }
    }
}
