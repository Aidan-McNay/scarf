// =======================================================================
// errors.rs
// =======================================================================
// The type of errors used for AST parsing

use chumsky::prelude::*;

pub type ParserError<'a> = extra::Err<Rich<'a, char>>;
