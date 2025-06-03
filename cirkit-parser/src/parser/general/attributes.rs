// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;
use std::iter;

pub fn attribute_instance_parser<'a, I>()
-> impl Parser<'a, I, AttributeInstance<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    attr_spec_parser()
        .map(|a| iter::once(a).collect())
        .foldl(
            just(Token::Comma)
                .ignore_then(attr_spec_parser())
                .repeated(),
            foldl_vector,
        )
        .map(|a| AttributeInstance(a))
}

pub fn attr_spec_parser<'a, I>() -> impl Parser<'a, I, AttrSpec<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    attr_name_parser()
        .then(
            just(Token::Eq)
                .ignore_then(constant_expression_parser())
                .or_not(),
        )
        .map(|(a, b)| AttrSpec(a, b))
}

pub fn attr_name_parser<'a, I>() -> impl Parser<'a, I, AttrName<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| AttrName(a))
}
