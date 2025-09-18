// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{opt, repeat};

pub fn attribute_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttributeInstance<'s>, VerboseError<'s>> {
    (
        token(Token::ParenStar),
        attr_spec_parser,
        repeat(0.., (token(Token::Comma), attr_spec_parser)),
        token(Token::StarEparen),
    )
        .map(|(a, b, c, d)| AttributeInstance(a, b, c, d))
        .parse_next(input)
}

pub fn attr_spec_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttrSpec<'s>, VerboseError<'s>> {
    (
        attr_name_parser,
        opt((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b)| AttrSpec(a, b))
        .parse_next(input)
}

pub fn attr_name_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AttrName<'s>, VerboseError<'s>> {
    identifier_parser.map(|a| AttrName(a)).parse_next(input)
}
