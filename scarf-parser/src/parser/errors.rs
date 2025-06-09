// =======================================================================
// errors.rs
// =======================================================================
// The type of errors used for AST parsing

use crate::*;
use chumsky::prelude::*;

pub type ParserError<'a> = extra::Err<Rich<'a, Token<'a>>>;
