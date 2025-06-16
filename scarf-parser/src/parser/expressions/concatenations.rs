// =======================================================================
// concatenations.rs
// =======================================================================
// Parsing for 1800-2023 A.8.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn streaming_concatenation_parser<'a, I>()
-> impl Parser<'a, I, StreamingConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
