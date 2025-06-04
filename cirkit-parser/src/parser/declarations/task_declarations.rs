// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn final_specifier_parser<'a, I>() -> impl Parser<'a, I, FinalSpecifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::Final = e => Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }
    }
    .then(extra_node_parser())
    .map(|(metadata, b)| (replace_nodes(metadata, b),))
}
