// =======================================================================
// callbacks.rs
// =======================================================================
// The callbacks used to lex a SystemVerilog source

use crate::*;
use logos::Lexer;

#[derive(Logos)]
enum StringToken {
    #[token(r#"""#)]
    Delimeter,
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
    #[token(r#"""""#)]
    Delimeter,
    #[regex(r#"[^\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos, Debug)]
enum PreprocessorStringToken {
    #[token(r#"`""#)]
    Delimeter,
    #[token(r#"`\`""#)]
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
    #[regex(r#"[^\r\n\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum PreprocessorMultilineStringToken {
    #[token(r#"`""""#)]
    Delimeter,
    #[token(r#"`\`""#)]
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
    #[regex(r#"[^\n\r\\]"#)]
    #[regex(r#"\\([ -~]|[0-7]{1,3})"#)]
    #[regex(r#"\\x[0-9a-fA-F]{1,2}"#)]
    Other,
}

#[derive(Logos)]
enum BlockCommentToken {
    #[token("*/")]
    Delimeter,
    #[regex(r".")]
    Other,
}

pub fn oneline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    lex.slice().strip_prefix("//")
}

pub fn block_comment<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a str, String> {
    let start_span = lex.span();
    let mut block_comment_lexer = lex.clone().morph::<BlockCommentToken>();
    while let Some(string_token) = block_comment_lexer.next() {
        match string_token {
            Ok(BlockCommentToken::Delimeter) => {
                let end_span = block_comment_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string);
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = block_comment_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated block comment".to_string());
            }
        }
    }
    let end_span = block_comment_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated block comment".to_string())
}

pub fn string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a str, String> {
    let start_span = lex.span();
    let mut string_lexer = lex.clone().morph::<StringToken>();
    while let Some(string_token) = string_lexer.next() {
        match string_token {
            Ok(StringToken::Delimeter) => {
                let end_span = string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string);
            }
            Ok(StringToken::Newline) => {
                let end_span = string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated string literal".to_string());
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated string literal".to_string());
            }
        }
    }
    let end_span = string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated string literal".to_string())
}

pub fn multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a str, String> {
    let start_span = lex.span();
    let mut multiline_string_lexer =
        lex.clone().morph::<MultilineStringToken>();
    while let Some(string_token) = multiline_string_lexer.next() {
        match string_token {
            Ok(MultilineStringToken::Delimeter) => {
                let end_span = multiline_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string);
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = multiline_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err("Unterminated multiline string literal".to_string());
            }
        }
    }
    let end_span = multiline_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated multiline string literal".to_string())
}

pub fn preprocessor_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a str, String> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorStringToken::Delimeter) => {
                let end_span = preprocessor_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string);
            }
            Ok(PreprocessorStringToken::Newline) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor string literal".to_string()
                );
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor string literal".to_string()
                );
            }
        }
    }
    let end_span = preprocessor_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated preprocessor string literal".to_string())
}

pub fn preprocessor_multiline_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Result<&'a str, String> {
    let start_span = lex.span();
    let mut preprocessor_string_lexer =
        lex.clone().morph::<PreprocessorMultilineStringToken>();
    while let Some(string_token) = preprocessor_string_lexer.next() {
        match string_token {
            Ok(PreprocessorMultilineStringToken::Delimeter) => {
                let end_span = preprocessor_string_lexer.span();
                let string = &lex.source()[start_span.end..end_span.start];
                lex.bump(end_span.end - start_span.end);
                return Ok(string);
            }
            Ok(PreprocessorMultilineStringToken::Newline) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor multiline string literal"
                        .to_string(),
                );
            }
            Ok(_) => (),
            Err(_) => {
                let end_span = preprocessor_string_lexer.span();
                lex.bump(end_span.start - start_span.end);
                return Err(
                    "Unterminated preprocessor multiline string literal"
                        .to_string(),
                );
            }
        }
    }
    let end_span = preprocessor_string_lexer.span();
    lex.bump(end_span.end - start_span.end);
    Err("Unterminated preprocessor multiline string literal".to_string())
}

pub fn text_macro<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    lex.slice().strip_prefix("`")
}
