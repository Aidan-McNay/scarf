// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn attribute_instance_parser<'a, I>()
-> impl Parser<'a, I, AttributeInstance<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    token(Token::ParenStar)
        .then(attr_spec_parser())
        .then(
            token(Token::Comma)
                .then(attr_spec_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, AttrSpec<'a>)>>(),
        )
        .then(token(Token::StarEparen))
        .map(|(((a, b), c), d)| AttributeInstance(a, b, c, d))
}

pub fn attr_spec_parser<'a, I>() -> impl Parser<'a, I, AttrSpec<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    attr_name_parser()
        .then(token(Token::Eq).then(constant_expression_parser()).or_not())
        .map(|(a, b)| AttrSpec(a, b))
}

pub fn attr_name_parser<'a, I>() -> impl Parser<'a, I, AttrName<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| AttrName(a))
}
