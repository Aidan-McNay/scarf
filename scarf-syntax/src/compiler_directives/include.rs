// =======================================================================
// include.rs
// =======================================================================
// Syntax for include directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum IncludeCompilerDirective<'a> {
    DoubleQuotes(
        (
            Span, // `include
            &'a str,
        ),
    ),
    AngleBracket(
        (
            Span, // `include
            Span, // <
            &'a str,
            Span, // >
        ),
    ),
}
