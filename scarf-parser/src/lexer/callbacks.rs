// =======================================================================
// callbacks.rs
// =======================================================================
// The callbacks used to lex a SystemVerilog source

use crate::*;
use logos::Lexer;
use regex::Regex;

pub fn oneline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re = Regex::new(r"//([^\n]*)").unwrap();
    let re_match = re.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}

pub fn string_literal<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re = Regex::new(
        r#""(([^"\r\n\\]|\\[\x00-\x7F]|\\[0-7]{1,3}|\\x[0-9a-fA-F]{1,2})*)""#,
    )
    .unwrap();
    let re_match = re.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}

pub fn text_macro<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let re = Regex::new(r"`(.*)").unwrap();
    let re_match = re.captures(lex.slice()).unwrap();
    match re_match.get(1) {
        Some(text) => Some(text.as_str()),
        None => None,
    }
}
