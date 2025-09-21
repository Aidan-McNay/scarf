// =======================================================================
// clocking_block.rs
// =======================================================================
// Parsing for 1800-2023 A.6.11

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn cycle_delay_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelay<'s>, VerboseError<'s>> {
    let _integral_parser = (token(Token::PoundPound), integral_number_parser)
        .map(|(a, b)| CycleDelay::Integral(Box::new((a, b))));
    let _identifier_parser = (token(Token::PoundPound), identifier_parser)
        .map(|(a, b)| CycleDelay::Identifier(Box::new((a, b))));
    let _expression_parser = (
        token(Token::PoundPound),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| CycleDelay::Expression(Box::new((a, b, c, d))));
    alt((_integral_parser, _identifier_parser, _expression_parser))
        .parse_next(input)
}
