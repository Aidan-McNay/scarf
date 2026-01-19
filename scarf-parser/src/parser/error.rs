// =======================================================================
// error.rs
// =======================================================================
// The type of errors used for AST parsing

use crate::*;
use core::ops::Range;
use lexer::Token;
use scarf_syntax::*;
use std::fs;
use winnow::{
    error::{AddContext, ParserError},
    stream::Stream,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expectation<'s> {
    Token(Token<'s>),
    Label(&'s str),
    EOI,
}

#[derive(Debug)]
pub struct VerboseError<'s> {
    pub valid: bool,
    pub span: Span<'s>,
    pub found: Option<Token<'s>>,
    pub expected: Vec<Expectation<'s>>,
}
impl<'s> VerboseError<'s> {
    pub fn is_eoi(&self) -> bool {
        self.found.is_none()
    }
}
impl<'s> Default for VerboseError<'s> {
    fn default() -> Self {
        VerboseError {
            valid: false,
            span: Span::default(),
            found: None,
            expected: vec![],
        }
    }
}
impl<'s> ParserError<Tokens<'s>> for VerboseError<'s> {
    type Inner = Self;
    fn from_input(input: &Tokens<'s>) -> Self {
        match input.peek_token() {
            Some(token) => VerboseError {
                valid: true,
                span: token.1.clone(),
                found: Some(token.0),
                expected: vec![],
            },
            None => VerboseError {
                valid: true,
                span: Span::default(),
                found: None,
                expected: vec![],
            },
        }
    }
    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
    fn or(mut self, mut other: Self) -> Self {
        // Check for invalid errors
        if !self.valid {
            return other;
        }
        if !other.valid {
            return self;
        }
        // Prefer errors that got to the end of the input
        match (self.found, other.found) {
            (None, Some(_)) => self,
            (Some(_), None) => other,
            (None, None) => {
                self.expected.append(&mut other.expected);
                self
            }
            (Some(_), Some(_)) => {
                // Prefer the one with a later span (a.k.a. got farther)
                match self.span.compare(&other.span) {
                    SpanRelation::Later => self,
                    SpanRelation::Earlier => other,
                    SpanRelation::Same => {
                        self.expected.append(&mut other.expected);
                        self
                    }
                }
            }
        }
    }
}
impl<'s> AddContext<Tokens<'s>, Token<'s>> for VerboseError<'s> {
    fn add_context(
        mut self,
        _input: &Tokens<'s>,
        _token_start: &<Tokens<'s> as Stream>::Checkpoint,
        _context: Token<'s>,
    ) -> Self {
        self.expected.push(Expectation::Token(_context));
        self
    }
}
impl<'s> AddContext<Tokens<'s>, &'s str> for VerboseError<'s> {
    fn add_context(
        mut self,
        _input: &Tokens<'s>,
        _token_start: &<Tokens<'s> as Stream>::Checkpoint,
        _context: &'s str,
    ) -> Self {
        self.expected.push(Expectation::Label(_context));
        self
    }
}

fn format_expectation<'s>(pattern: &Expectation<'s>) -> String {
    match pattern {
        Expectation::Token(token) => token.to_string(),
        Expectation::Label(label) => label.to_string(),
        Expectation::EOI => "end of input".to_string(),
    }
}

fn format_reason<'s>(error: &VerboseError<'s>) -> String {
    let found_str = match error.found {
        Some(tok) => tok.to_string(),
        None => "end of input".to_owned(),
    };
    let mut dedup_expected: Vec<Expectation<'s>> = vec![];
    for expected in error.expected.iter() {
        if !dedup_expected.contains(expected) {
            dedup_expected.push(expected.clone());
        }
    }
    let expected_str = match &dedup_expected[..] {
        [] => "something else".to_owned(),
        [expected] => format_expectation(expected),
        _ => {
            let mut temp_expected_str = String::new();
            for expected in &dedup_expected[..dedup_expected.len() - 1] {
                temp_expected_str
                    .push_str(format_expectation(expected).as_str());
                temp_expected_str.push_str(", ");
            }
            temp_expected_str.push_str("or ");
            temp_expected_str.push_str(
                format_expectation(dedup_expected.last().unwrap()).as_str(),
            );
            temp_expected_str
        }
    };
    format!("found {}, expected {}", found_str, expected_str)
}

fn format_reason_short<'s>(error: &VerboseError<'s>) -> String {
    match error.found {
        Some(tok) => format!("Didn't expect {}", tok.to_string()),
        None => "Didn't expect end of input".to_owned(),
    }
}

impl<'s> From<VerboseError<'s>>
    for Report<'s, (String, std::ops::Range<usize>)>
{
    fn from(value: VerboseError<'s>) -> Self {
        let error_span = if value.is_eoi() {
            let file_len = fs::metadata(value.span.file)
                .expect("TODO: Handle file read error")
                .len();
            let byte_span = Range {
                start: file_len as usize,
                end: file_len as usize,
            };
            Span {
                file: value.span.file,
                bytes: byte_span,
                expanded_from: None,
                included_from: value.span.included_from,
            }
        } else {
            value.span.clone()
        };
        let mut report = Report::build(
            ReportKind::Error,
            (error_span.file.to_string(), error_span.bytes.clone()),
        )
        .with_code("P1")
        .with_config(
            ariadne::Config::new().with_index_type(ariadne::IndexType::Byte),
        )
        .with_message(format_reason(&value));
        report = attach_span_label(
            error_span,
            Color::Red,
            format_reason_short(&value),
            report,
        );
        report.finish()
    }
}

impl<'s> VerboseError<'s> {
    pub fn or_in_place(&mut self, mut other: Self) {
        // Check for invalid errors
        if !self.valid {
            *self = other;
            return;
        }
        if !other.valid {
            return;
        }
        // Prefer errors that got to the end of the input
        match (self.found, other.found) {
            (None, Some(_)) => (),
            (Some(_), None) => *self = other,
            (None, None) => {
                self.expected.append(&mut other.expected);
            }
            (Some(_), Some(_)) => {
                // Prefer the one with a later span (a.k.a. got farther)
                match self.span.compare(&other.span) {
                    SpanRelation::Later => (),
                    SpanRelation::Earlier => *self = other,
                    SpanRelation::Same => {
                        self.expected.append(&mut other.expected);
                    }
                }
            }
        }
    }
}
