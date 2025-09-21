// =======================================================================
// specify_path_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.7.2

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn edge_identifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EdgeIdentifier<'s>, VerboseError<'s>> {
    alt((
        token(Token::Posedge).map(|a| EdgeIdentifier::Posedge(a)),
        token(Token::Negedge).map(|a| EdgeIdentifier::Negedge(a)),
        token(Token::Edge).map(|a| EdgeIdentifier::Edge(a)),
    ))
    .parse_next(input)
}
