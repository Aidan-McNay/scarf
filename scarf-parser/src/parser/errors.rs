// =======================================================================
// errors.rs
// =======================================================================
// The type of errors used for AST parsing

use crate::*;
use lexer::{Span, Token};
use winnow::{
    error::{AddContext, ParserError},
    stream::Stream,
};

#[derive(Debug)]
pub struct VerboseError<'s> {
    pub valid: bool,
    pub span: Span,
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
                //  - Only compare start, since they'd belong to the
                //    same token if the same
                if self.span.start > other.span.start {
                    self
                } else if self.span.start < other.span.start {
                    other
                } else {
                    self.expected.append(&mut other.expected);
                    self
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
                //  - Only compare start, since they'd belong to the
                //    same token if the same
                if self.span.start > other.span.start {
                    ()
                } else if self.span.start < other.span.start {
                    *self = other
                } else {
                    self.expected.append(&mut other.expected);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expectation<'s> {
    Token(Token<'s>),
    Label(&'s str),
    EOI,
}
