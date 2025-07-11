// =======================================================================
// declaration_ranges.rs
// =======================================================================
// Parsing for 1800-2023 A.2.5

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn unpacked_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, UnpackedDimension<'a>, ParserError<'a>> + Clone {
    let unpacked_range_parser = token(Token::Bracket)
        .then(constant_range_parser(constant_expression_parser(
            expression_parser(),
        )))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| UnpackedDimension::UnpackedRange(Box::new((a, b, c))));
    let unpacked_expression_parser = token(Token::Bracket)
        .then(constant_expression_parser(expression_parser()))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| UnpackedDimension::UnpackedExpression(Box::new((a, b, c))));
    choice((unpacked_range_parser, unpacked_expression_parser)).boxed()
}

pub fn packed_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, PackedDimension<'a>, ParserError<'a>> + Clone {
    let packed_range_parser = token(Token::Bracket)
        .then(constant_range_parser(constant_expression_parser(
            expression_parser(),
        )))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| PackedDimension::PackedRange(Box::new((a, b, c))));
    choice((
        packed_range_parser,
        unsized_dimension_parser().map(|a| PackedDimension::UnsizedDimension(Box::new(a))),
    ))
    .boxed()
}

pub fn associative_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, AssociativeDimension<'a>, ParserError<'a>> + Clone {
    let data_parser = token(Token::Bracket)
        .then(data_type_parser())
        .then(token(Token::EBracket))
        .map(|((a, b), c)| AssociativeDimension::Data(Box::new((a, b, c))));
    let star_parser = token(Token::Bracket)
        .then(token(Token::Star))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| AssociativeDimension::Star(Box::new((a, b, c))));
    choice((data_parser, star_parser)).boxed()
}

pub fn variable_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, VariableDimension<'a>, ParserError<'a>> + Clone {
    choice((
        unsized_dimension_parser().map(|a| VariableDimension::UnsizedDimension(Box::new(a))),
        unpacked_dimension_parser().map(|a| VariableDimension::UnpackedDimension(Box::new(a))),
        associative_dimension_parser()
            .map(|a| VariableDimension::AssociativeDimension(Box::new(a))),
        queue_dimension_parser().map(|a| VariableDimension::QueueDimension(Box::new(a))),
    ))
    .boxed()
}

pub fn queue_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, QueueDimension<'a>, ParserError<'a>> + Clone {
    token(Token::Bracket)
        .then(token(Token::Dollar))
        .then(
            token(Token::Colon)
                .then(constant_expression_parser(expression_parser()))
                .or_not(),
        )
        .then(token(Token::EBracket))
        .map(|(((a, b), c), d)| QueueDimension(a, b, c, d))
        .boxed()
}

pub fn unsized_dimension_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, UnsizedDimension<'a>, ParserError<'a>> + Clone {
    token(Token::Bracket)
        .then(token(Token::EBracket))
        .map(|(a, b)| UnsizedDimension(a, b))
        .boxed()
}
