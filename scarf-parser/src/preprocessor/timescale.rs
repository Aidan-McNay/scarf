// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for timescale directives

use crate::*;
use scarf_syntax::SpanRelation;
use std::iter::Peekable;

#[derive(Clone, Debug)]
pub enum TimescaleValue {
    One,
    Ten,
    Hundred,
}

#[derive(Clone, Debug)]
pub enum TimescaleUnit {
    S,
    MS,
    US,
    NS,
    PS,
    FS,
}

#[derive(Clone, Debug)]
pub struct Timescale<'a> {
    def_span: Span<'a>,
    pub unit: (TimescaleValue, TimescaleUnit),
    pub precision: (TimescaleValue, TimescaleUnit),
}

impl<'a> Timescale<'a> {
    pub const fn new(
        def_span: Span<'a>,
        unit: (TimescaleValue, TimescaleUnit),
        precision: (TimescaleValue, TimescaleUnit),
    ) -> Timescale<'a> {
        Timescale {
            def_span,
            unit,
            precision,
        }
    }

    pub fn is_valid(&self, delay_span: &Span<'a>) -> bool {
        self.def_span.compare(delay_span) == SpanRelation::Earlier
    }
}

fn get_timescale<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    def_span: Span<'s>,
) -> Result<(TimescaleValue, TimescaleUnit), PreprocessorError<'s>> {
    // Return a token, as well as indicating whether it's the
    // end of the define
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(def_span));
    };
    let timescale_value = match spanned_token.0 {
        Token::UnsignedNumber("1") => TimescaleValue::One,
        Token::UnsignedNumber("10") => TimescaleValue::Ten,
        Token::UnsignedNumber("100") => TimescaleValue::Hundred,
        _ => {
            return Err(PreprocessorError::Error(VerboseError {
                valid: true,
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("1, 10, or 100")],
            }));
        }
    };
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(def_span));
    };
    let timescale_unit = match spanned_token.0 {
        Token::TimeUnit("s") => TimescaleUnit::S,
        Token::TimeUnit("ms") => TimescaleUnit::MS,
        Token::TimeUnit("us") => TimescaleUnit::US,
        Token::TimeUnit("ns") => TimescaleUnit::NS,
        Token::TimeUnit("ps") => TimescaleUnit::PS,
        Token::TimeUnit("fs") => TimescaleUnit::FS,
        _ => {
            return Err(PreprocessorError::Error(VerboseError {
                valid: true,
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label("a unit of time")],
            }));
        }
    };
    Ok((timescale_value, timescale_unit))
}

fn get_divider<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    def_span: Span<'s>,
) -> Result<Span<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(def_span));
    };
    match spanned_token.0 {
        Token::Slash => Ok(spanned_token.1),
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![Expectation::Token(Token::Slash)],
        })),
    }
}

pub fn preprocess_timescale<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    directive_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let timeunit = get_timescale(src, directive_span.clone())?;
    let _ = get_divider(src, directive_span.clone())?;
    let timeprecision = get_timescale(src, directive_span.clone())?;
    configs.add_timescale(directive_span, timeunit, timeprecision);
    Ok(())
}
