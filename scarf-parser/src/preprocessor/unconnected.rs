// =======================================================================
// unconnected.rs
// =======================================================================
// Preprocessing for directives affecting unconnected ports

use crate::Span;
use crate::*;
use std::iter::Peekable;

#[derive(Clone)]
pub enum UnconnectedDrive {
    PullUp,
    PullDown,
    NoUnconnected,
}

fn get_unconnected_drive<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    directive_span: Span<'s>,
) -> Result<UnconnectedDrive, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(directive_span));
    };
    match spanned_token.0 {
        Token::Pull0 => Ok(UnconnectedDrive::PullDown),
        Token::Pull1 => Ok(UnconnectedDrive::PullUp),
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![Expectation::Label("a preprocessor macro name")],
        })),
    }
}

pub fn preprocess_unconnected_drive<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let unconnected_drive = get_unconnected_drive(src, directive_span.clone())?;
    configs.add_unconnected_drive(directive_span, unconnected_drive);
    Ok(())
}

pub fn preprocess_nounconnected_drive<'s>(
    configs: &mut PreprocessConfigs<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    configs
        .add_unconnected_drive(directive_span, UnconnectedDrive::NoUnconnected);
    Ok(())
}
