// =======================================================================
// utils.rs
// =======================================================================
// Helper functions for implementing parsers

use crate::*;
use chumsky::prelude::*;

// Fold parsed text into a vector
pub fn foldl_vector<T>(mut a: Vec<T>, b: T) -> Vec<T> {
    a.push(b);
    a
}

// Separation between words
pub fn sep<'a>() -> impl Parser<'a, &'a str, (), ParserError<'a>> {
    text::whitespace().at_least(1)
}
