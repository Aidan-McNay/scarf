// =======================================================================
// lib.rs
// =======================================================================
// The top-level lexing interface

pub mod tokens;
use logos::{Logos, SpannedIter};
pub use tokens::Token;

pub fn lex_sv<'a>(src: &'a str) -> SpannedIter<'a, Token> {
    Token::lexer(src).spanned()
}
