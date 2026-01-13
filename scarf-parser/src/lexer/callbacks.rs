// =======================================================================
// callbacks.rs
// =======================================================================
// The callbacks used to lex a SystemVerilog source

use crate::*;
use logos::Lexer;
use regex::Regex;
use std::sync::LazyLock;

static ONELINE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//([^\n]*)").unwrap());
static STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#""(([^"\r\n\\]|\\[\x00-\x7F]|\\[0-7]{1,3}|\\x[0-9a-fA-F]{1,2})*)""#,
    )
    .unwrap()
});
static PREPROCESSOR_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"`"(([^"\r\n\\]|\\[\x00-\x7F]|\\[0-7]{1,3}|\\x[0-9a-fA-F]{1,2})*(`\\`"([^"\r\n\\]|\\[\x00-\x7F]|\\[0-7]{1,3}|\\x[0-9a-fA-F]{1,2})*)*)`""#,
    )
    .unwrap()
});
static TEXT_MACRO_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`(.*)").unwrap());

pub fn oneline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re_match = ONELINE_REGEX.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}

pub fn string_literal<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re_match = STRING_REGEX.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}

pub fn preprocessor_string_literal<'a>(
    lex: &mut Lexer<'a, Token<'a>>,
) -> Option<&'a str> {
    let re_match = PREPROCESSOR_STRING_REGEX.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}

pub fn text_macro<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re_match = TEXT_MACRO_REGEX.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}
