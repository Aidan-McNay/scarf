// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn attribute_instance_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, AttributeInstance<'a>, ParserError<'a>> + Clone {
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

pub fn attr_spec_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, AttrSpec<'a>, ParserError<'a>> + Clone {
    attr_name_parser()
        .then(token(Token::Eq).then(constant_expression_parser).or_not())
        .map(|(a, b)| AttrSpec(a, b))
        .boxed()
}

pub fn attr_name_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, AttrName<'a>, ParserError<'a>> + Clone {
    identifier_parser().map(|a| AttrName(a)).boxed()
}
