// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.7

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn number_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Number<'a>, ParserError<'a>> + Clone
{
    choice((
        integral_number_parser().map(|a| Number::Integral(Box::new(a))),
        real_number_parser().map(|a| Number::Real(Box::new(a))),
    ))
    .labelled("a number")
    .boxed()
}

pub fn integral_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, IntegralNumber<'a>, ParserError<'a>> + Clone {
    choice((
        decimal_number_parser().map(|a| IntegralNumber::Decimal(Box::new(a))),
        octal_number_parser().map(|a| IntegralNumber::Octal(Box::new(a))),
        binary_number_parser().map(|a| IntegralNumber::Binary(Box::new(a))),
        hex_number_parser().map(|a| IntegralNumber::Hex(Box::new(a))),
    ))
    .labelled("an integral number")
    .boxed()
}

pub fn decimal_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, DecimalNumber<'a>, ParserError<'a>> + Clone {
    let _sized_parser = select! {
        Token::DecimalNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .then(extra_node_parser())
    .map(|((text, metadata), b)| {
        DecimalNumber::Sized(Box::new((text, replace_nodes(metadata, b))))
    });
    choice((
        _sized_parser,
        unsigned_number_parser().map(|a| DecimalNumber::Unsized(Box::new(a))),
    ))
    .labelled("a decimal number")
}

pub fn binary_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, BinaryNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::BinaryNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("a binary number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| BinaryNumber(text, replace_nodes(metadata, b)))
}

pub fn octal_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, OctalNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::OctalNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("an octal number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| OctalNumber(text, replace_nodes(metadata, b)))
}

pub fn hex_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, HexNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::HexNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("a hex number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| HexNumber(text, replace_nodes(metadata, b)))
}

pub fn real_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, RealNumber<'a>, ParserError<'a>> + Clone {
    choice((
        fixed_point_number_parser().map(|a| RealNumber::FixedPoint(Box::new(a))),
        scientific_number_parser().map(|a| RealNumber::Scientific(Box::new(a))),
    ))
    .labelled("a real number")
    .boxed()
}

pub fn fixed_point_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, FixedPointNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::FixedPointNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("a fixed-point number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| FixedPointNumber(text, replace_nodes(metadata, b)))
}

pub fn scientific_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ScientificNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::ScientificNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("a scientific-notation number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| ScientificNumber(text, replace_nodes(metadata, b)))
}

pub fn unsigned_number_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, UnsignedNumber<'a>, ParserError<'a>> + Clone {
    select! {
        Token::UnsignedNumber(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("an unsigned number")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| UnsignedNumber(text, replace_nodes(metadata, b)))
    .boxed()
}

pub fn unbased_unsized_literal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, UnbasedUnsizedLiteral<'a>, ParserError<'a>> + Clone {
    select! {
        Token::UnbasedUnsizedLiteral(text) = e => (text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("an unsized literal")
    .then(extra_node_parser())
    .map(|((text, metadata), b)| UnbasedUnsizedLiteral(text, replace_nodes(metadata, b)))
}
