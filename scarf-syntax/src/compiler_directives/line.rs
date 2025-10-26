// =======================================================================
// line.rs
// =======================================================================
// Syntax for line directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LineCompilerDirective<'a>(
    pub Span,            // `line
    pub (&'a str, Span), // number
    pub (&'a str, Span), // filename
    pub (&'a str, Span), // level
);
