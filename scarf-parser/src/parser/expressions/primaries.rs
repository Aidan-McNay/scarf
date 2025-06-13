// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

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
        Token::TimeUnit(unit) = e if unit == "s" => TimeUnit::S(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ms" => TimeUnit::MS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "us" => TimeUnit::US(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ns" => TimeUnit::NS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ps" => TimeUnit::PS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "fs" => TimeUnit::FS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
    }
    .labelled("a time unit")
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

pub fn implicit_class_handle_parser<'a, I>()
-> impl Parser<'a, I, ImplicitClassHandle<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _this_parser = token(Token::This).map(|a| ImplicitClassHandle::This(a));
    let _super_parser = token(Token::Super).map(|a| ImplicitClassHandle::Super(a));
    let _this_super_parser = token(Token::This)
        .then(token(Token::Period))
        .then(token(Token::Super))
        .map(|((a, b), c)| ImplicitClassHandle::ThisSuper(a, b, c));
    choice((_this_parser, _super_parser, _this_super_parser))
}

pub fn constant_bit_select_parser<'a, I>()
-> impl Parser<'a, I, ConstantBitSelect<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Bracket)
        .then(constant_expression_parser())
        .then(token(Token::EBracket))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Metadata<'a>, ConstantExpression<'a>, Metadata<'a>)>>()
        .map(|a| ConstantBitSelect(a))
}

pub fn constant_select_parser<'a, I>() -> impl Parser<'a, I, ConstantSelect<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
