// =======================================================================
// parallel_and_sequential_blocks.rs
// =======================================================================
// Parsing for 1800-2023 A.6.3

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt};

pub fn action_block_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ActionBlock<'s>, VerboseError<'s>> {
    alt((
        statement_or_null_parser.map(|a| ActionBlock::Basic(Box::new(a))),
        (
            opt(statement_parser),
            token(Token::Else),
            statement_or_null_parser,
        )
            .map(|(a, b, c)| ActionBlock::Conditional(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}
