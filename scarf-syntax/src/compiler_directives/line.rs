// =======================================================================
// line.rs
// =======================================================================
// Syntax for line directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LineCompilerDirective<'a>(
    pub Span<'a>,            // `line
    pub (&'a str, Span<'a>), // number
    pub (&'a str, Span<'a>), // filename
    pub (&'a str, Span<'a>), // level
);
