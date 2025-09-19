// =======================================================================
// timing_control_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.5

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn delay_control_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DelayControl<'s>, VerboseError<'s>> {
    let _value_parser = (token(Token::Pound), delay_value_parser)
        .map(|(a, b)| DelayControl::Value(Box::new((a, b))));
    let _mintypmax_parser = (
        token(Token::Pound),
        token(Token::Paren),
        mintypmax_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| DelayControl::Mintypmax(Box::new((a, b, c, d))));
    alt((_value_parser, _mintypmax_parser)).parse_next(input)
}
