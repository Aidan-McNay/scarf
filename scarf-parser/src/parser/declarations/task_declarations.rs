// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn final_specifier_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, FinalSpecifier<'a>, ParserError<'a>> + Clone {
    token(Token::Colon)
        .then(token(Token::Final))
        .map(|(a, b)| FinalSpecifier(a, b))
        .boxed()
}
