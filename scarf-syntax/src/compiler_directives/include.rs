// =======================================================================
// include.rs
// =======================================================================
// Syntax for include directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum IncludeCompilerDirective<'a> {
    DoubleQuotes(
        (
            Span<'a>, // `include
            &'a str,
        ),
    ),
    AngleBracket(
        (
            Span<'a>, // `include
            Span<'a>, // <
            &'a str,
            Span<'a>, // >
        ),
    ),
}
