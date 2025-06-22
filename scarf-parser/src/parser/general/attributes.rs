// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn attribute_instance_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, AttributeInstance<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    token(Token::ParenStar)
        .then(attr_spec_parser(constant_expression_parser.clone()))
        .then(
            token(Token::Comma)
                .then(attr_spec_parser(constant_expression_parser))
                .repeated()
                .collect::<Vec<(Metadata<'a>, AttrSpec<'a>)>>(),
        )
        .then(token(Token::StarEparen))
        .map(|(((a, b), c), d)| AttributeInstance(a, b, c, d))
        .boxed()
}

pub fn attr_spec_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, AttrSpec<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    attr_name_parser()
        .then(token(Token::Eq).then(constant_expression_parser).or_not())
        .map(|(a, b)| AttrSpec(a, b))
        .boxed()
}

pub fn attr_name_parser<'a, I>() -> impl Parser<'a, I, AttrName<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| AttrName(a)).boxed()
}
