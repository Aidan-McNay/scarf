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
    pub span: Span,
    pub found: Option<Token<'s>>,
    pub expected: Vec<Expectation<'s>>,
}
impl<'s> ParserError<Tokens<'s>> for VerboseError<'s> {
    type Inner = Self;
    fn from_input(input: &Tokens<'s>) -> Self {
        match input.peek_token() {
            Some(token) => VerboseError {
                span: token.1.clone(),
                found: Some(token.0),
                expected: vec![],
            },
            None => VerboseError {
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

#[derive(Debug)]
pub enum Expectation<'s> {
    Token(Token<'s>),
    Label(&'s str),
    EOI,
}
