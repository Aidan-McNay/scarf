// =======================================================================
// merge.rs
// =======================================================================
// Merge lexed tokens that represent one syntactic token

use crate::*;
use logos::Span as ByteSpan;
use logos::{Lexer, SpannedIter};

pub struct TokenMerge<'s> {
    tokens: SpannedIter<'s, Token<'s>>,
    prev_num_span_end: Option<usize>,
    text: &'s str,
}

impl<'s> TokenMerge<'s> {
    pub fn new(src_lexer: Lexer<'s, Token<'s>>, src_text: &'s str) -> Self {
        Self {
            tokens: src_lexer.spanned(),
            prev_num_span_end: None,
            text: src_text,
        }
    }

    fn get_block_comment(
        &mut self,
        start_span: ByteSpan,
    ) -> (Result<Token<'s>, String>, ByteSpan) {
        loop {
            match self.tokens.next() {
                Some((Ok(Token::BlockCommentEnd), end_span)) => {
                    let block_comment_text =
                        &self.text[start_span.end..end_span.start];
                    let overall_span = ByteSpan {
                        start: start_span.start,
                        end: end_span.end,
                    };
                    return (
                        Ok(Token::BlockComment(block_comment_text)),
                        overall_span,
                    );
                }
                None => {
                    return (
                        Err("Block comment with no ending".to_owned()),
                        start_span,
                    );
                }
                _ => (),
            }
        }
    }

    fn get_triple_quote_string(
        &mut self,
        start_span: ByteSpan,
    ) -> (Result<Token<'s>, String>, ByteSpan) {
        loop {
            match self.tokens.next() {
                Some((Ok(Token::QuoteQuoteQuote), end_span)) => {
                    let string_text =
                        &self.text[start_span.end..end_span.start];
                    let overall_span = ByteSpan {
                        start: start_span.start,
                        end: end_span.end,
                    };
                    return (
                        Ok(Token::TripleQuoteStringLiteral(string_text)),
                        overall_span,
                    );
                }
                None => {
                    return (
                        Err("Triple-quote string with no ending".to_owned()),
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
        match self.tokens.next() {
            Some((Ok(Token::BlockCommentStart), start_span)) => {
                Some(self.get_block_comment(start_span))
            }
            Some((Ok(Token::QuoteQuoteQuote), start_span)) => {
                Some(self.get_triple_quote_string(start_span))
            }
            Some((Ok(Token::UnsignedNumber(num_str)), num_span)) => {
                self.prev_num_span_end = Some(num_span.end);
                Some((Ok(Token::UnsignedNumber(num_str)), num_span))
            }
            Some((Ok(Token::FixedPointNumber(num_str)), num_span)) => {
                self.prev_num_span_end = Some(num_span.end);
                Some((Ok(Token::FixedPointNumber(num_str)), num_span))
            }
            Some((Ok(Token::SimpleIdentifier(text)), span)) => {
                if let Some(num_span_end) = self.prev_num_span_end {
                    if num_span_end == span.start {
                        match text {
                            "s" | "ms" | "us" | "ns" | "ps" | "fs" => {
                                Some((Ok(Token::TimeUnit(text)), span))
                            }
                            _ => {
                                Some((Ok(Token::SimpleIdentifier(text)), span))
                            }
                        }
                    } else {
                        Some((Ok(Token::SimpleIdentifier(text)), span))
                    }
                } else {
                    Some((Ok(Token::SimpleIdentifier(text)), span))
                }
            }
            other => other,
        }
    }
}
