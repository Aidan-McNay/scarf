// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn time_literal_parser<'a, I>() -> impl Parser<'a, I, TimeLiteral<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        fixed_point_number_parser().map(|a| {
            TimeLiteral::TimeLiteralFixedPoint(Box::new((a, TimeUnit::S(Metadata::default()))))
        }),
        unsigned_number_parser().map(|a| {
            TimeLiteral::TimeLiteralUnsigned(Box::new((a, TimeUnit::S(Metadata::default()))))
        }),
    ))
    .then(time_unit_parser())
    .map(|(a, b)| match a {
        TimeLiteral::TimeLiteralFixedPoint(box_value) => {
            TimeLiteral::TimeLiteralFixedPoint(Box::new((box_value.0, b)))
        }
        TimeLiteral::TimeLiteralUnsigned(box_value) => {
            TimeLiteral::TimeLiteralUnsigned(Box::new((box_value.0, b)))
        }
    })
}

fn time_unit_parser<'a, I>() -> impl Parser<'a, I, TimeUnit<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::TimeUnit(unit) = e => match unit {
            "s" => TimeUnit::S(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            "ms" => TimeUnit::MS(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            "us" => TimeUnit::US(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            "ns" => TimeUnit::NS(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            "ps" => TimeUnit::PS(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            "fs" => TimeUnit::FS(Metadata{
                span: convert_span(e.span()),
                extra_nodes: Vec::new()
            }),
            _ => panic!("Internal issue identifying time units")
        }
    }
    .then(extra_node_parser())
    .map(|(timeunit, b)| match timeunit {
        TimeUnit::S(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::MS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::US(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::NS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::PS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::FS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
    })
}
