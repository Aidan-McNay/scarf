// =======================================================================
// implicit_nettype.rs
// =======================================================================
// Preprocessing for default nettypes

use crate::Span;
use crate::*;
use std::iter::Peekable;

#[derive(Clone)]
pub enum DefaultNettype {
    Wire,
    Tri,
    Tri0,
    Tri1,
    Wand,
    Triand,
    Wor,
    Trior,
    Trireg,
    Uwire,
    None,
}

fn get_nettype<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span<'s>,
) -> Result<DefaultNettype, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        Token::Wire => Ok(DefaultNettype::Wire),
        Token::Tri => Ok(DefaultNettype::Tri),
        Token::Tri0 => Ok(DefaultNettype::Tri0),
        Token::Tri1 => Ok(DefaultNettype::Tri1),
        Token::Wand => Ok(DefaultNettype::Wand),
        Token::Triand => Ok(DefaultNettype::Triand),
        Token::Wor => Ok(DefaultNettype::Wor),
        Token::Trior => Ok(DefaultNettype::Trior),
        Token::Trireg => Ok(DefaultNettype::Trireg),
        Token::Uwire => Ok(DefaultNettype::Uwire),
        Token::SimpleIdentifier("none") => Ok(DefaultNettype::None),
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![
                Expectation::Token(Token::Wire),
                Expectation::Token(Token::Tri),
                Expectation::Token(Token::Tri0),
                Expectation::Token(Token::Tri1),
                Expectation::Token(Token::Wand),
                Expectation::Token(Token::Triand),
                Expectation::Token(Token::Wor),
                Expectation::Token(Token::Trior),
                Expectation::Token(Token::Trireg),
                Expectation::Token(Token::Uwire),
                Expectation::Label("'none'"),
            ],
        })),
    }
}

pub fn preprocess_default_nettype<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let default_nettype = get_nettype(src, directive_span.clone())?;
    configs.add_default_nettype(directive_span, default_nettype);
    Ok(())
}
