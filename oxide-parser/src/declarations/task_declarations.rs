// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn final_specifier_parser<'a>() -> impl Parser<'a, &'a str, FinalSpecifier, ParserError<'a>> {
    just(':')
        .then_ignore(sep())
        .then_ignore(just("final"))
        .to(())
}
