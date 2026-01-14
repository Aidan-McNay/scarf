// =======================================================================
// callbacks.rs
// =======================================================================
// The callbacks used to lex a SystemVerilog source

use crate::*;
use logos::{Lexer, Span};

#[derive(Logos)]
enum StringToken {
    #[token(r#"""#, |lex| lex.span())]
    Delimeter(Span),
    #[token(r#"\\""#)]
    EscapedDelimeter,
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(r#"[^"\r\n\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum MultilineStringToken {
    #[token(r#"""""#, |lex| lex.span())]
    Delimeter(Span),
    #[regex(r#"[^\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum PreprocessorStringToken {
    #[token(r#"`""#, |lex| lex.span())]
    Delimeter(Span),
    #[token(r#"\\`\\""#)]
    EscapedDelimeter,
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(r#"[^"\r\n\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum PreprocessorMultilineStringToken {
    #[token(r#"`""""#, |lex| lex.span())]
    Delimeter(Span),
    #[token("\n")]
    #[token("\r")]
    #[token("\r\n")]
    #[token("\u{0085}")]
    #[token("\u{2028}")]
    #[token("\u{2029}")]
    Newline,
    #[token("\\\n")]
    #[token("\\\r")]
    #[token("\\\r\n")]
    #[token("\\\u{0085}")]
    #[token("\\\u{2028}")]
    #[token("\\\u{2029}")]
    EscapedNewline,
    #[regex(r#"[^\n\r\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum BlockCommentToken {
    #[token("*/", |lex| lex.span())]
    Delimeter(Span),
    #[regex(r".")]
    Other,
}

pub fn oneline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    lex.slice().strip_prefix("//")
}

pub fn block_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let start_span = lex.span();
    let mut block_comment_lexer = lex.clone().morph::<BlockCommentToken>();
    while let Some(string_token) = block_comment_lexer.next() {
        match string_token {
            Ok(BlockCommentToken::Delimeter(end_span)) => {
                let string = &lex.source()[start_span.end..end_span.start];
                *lex = block_comment_lexer.morph();
                return Some(string);
            }
            Ok(_) => (),
            Err(_) => {
                return None;
            }
        }
    }
    None
}

pub fn string_literal<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let start_span = lex.span();
    let mut string_lexer = lex.clone().morph::<StringToken>();
    while let Some(string_token) = string_lexer.next() {
        match string_token {
            Ok(StringToken::Delimeter(end_span)) => {
                let string = &lex.source()[start_span.end..end_span.start];
                *lex = string_lexer.morph();
                return Some(string);
            }
            Ok(StringToken::Newline) => {
                *lex = string_lexer.morph();
                return None;
            }
            Ok(_) => (),
            Err(_) => {
                return None;
            }
        }
    }
    None
}

pub fn multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Option<&'a str> {
    let start_span = lex.span();
    let mut multiline_string_lexer =
        lex.clone().morph::<MultilineStringToken>();
    while let Some(string_token) = multiline_string_lexer.next() {
        match string_token {
            Ok(MultilineStringToken::Delimeter(end_span)) => {
                let string = &lex.source()[start_span.end..end_span.start];
                *lex = multiline_string_lexer.morph();
                return Some(string);
            }
            Ok(_) => (),
            Err(_) => {
                return None;
            }
        }
    }
    None
}

pub fn preprocessor_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Option<&'a str> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorStringToken::Delimeter(end_span)) => {
                let string = &lex.source()[start_span.end..end_span.start];
                *lex = preprocessor_string_lexer.morph();
                return Some(string);
            }
            Ok(PreprocessorStringToken::Newline) => {
                *lex = preprocessor_string_lexer.morph();
                return None;
            }
            Ok(_) => (),
            Err(_) => {
                return None;
            }
        }
    }
    None
}

pub fn preprocessor_multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Option<&'a str> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorMultilineStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorMultilineStringToken::Delimeter(end_span)) => {
                let string = &lex.source()[start_span.end..end_span.start];
                *lex = preprocessor_string_lexer.morph();
                return Some(string);
            }
            Ok(PreprocessorMultilineStringToken::Newline) => {
                *lex = preprocessor_string_lexer.morph();
                return None;
            }
            Ok(_) => (),
            Err(_) => {
                return None;
            }
        }
    }
    None
}

pub fn text_macro<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    lex.slice().strip_prefix("`")
}
