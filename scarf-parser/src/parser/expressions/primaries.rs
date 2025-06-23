// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_primary_parser<'a, I>(
    _constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, ConstantPrimary<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn primary_literal_parser<'a, I>()
-> impl Parser<'a, I, PrimaryLiteral<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        number_parser().map(|a| PrimaryLiteral::Number(Box::new(a))),
        time_literal_parser().map(|a| PrimaryLiteral::TimeLiteral(Box::new(a))),
        unbased_unsized_literal_parser()
            .map(|a| PrimaryLiteral::UnbasedUnsizedLiteral(Box::new(a))),
        string_literal_parser().map(|a| PrimaryLiteral::StringLiteral(Box::new(a))),
    ))
    .boxed()
}

pub fn time_literal_parser<'a, I>() -> impl Parser<'a, I, TimeLiteral<'a>, ParserError<'a>> + Clone
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
    .boxed()
}

fn time_unit_parser<'a, I>() -> impl Parser<'a, I, TimeUnit<'a>, ParserError<'a>> + Clone
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
    .boxed()
}

pub fn select_parser<'a, I>() -> impl Parser<'a, I, Select<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn nonrange_select_parser<'a, I>()
-> impl Parser<'a, I, NonrangeSelect<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn implicit_class_handle_parser<'a, I>()
-> impl Parser<'a, I, ImplicitClassHandle<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _this_parser = token(Token::This).map(|a| ImplicitClassHandle::This(a));
    let _super_parser = token(Token::Super).map(|a| ImplicitClassHandle::Super(a));
    let _this_super_parser = token(Token::This)
        .then(token(Token::Period))
        .then(token(Token::Super))
        .map(|((a, b), c)| ImplicitClassHandle::ThisSuper(a, b, c));
    choice((_this_parser, _super_parser, _this_super_parser)).boxed()
}

pub fn constant_bit_select_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantBitSelect<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Bracket)
        .then(constant_expression_parser)
        .then(token(Token::EBracket))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Metadata<'a>, ConstantExpression<'a>, Metadata<'a>)>>()
        .map(|a| ConstantBitSelect(a))
        .boxed()
}

pub fn constant_select_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantSelect<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _hierarchy_parser = token(Token::Period)
        .then(member_identifier_parser())
        .then(constant_bit_select_parser(
            constant_expression_parser.clone(),
        ))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Metadata<'a>, MemberIdentifier<'a>, ConstantBitSelect<'a>)>>()
        .then(token(Token::Period))
        .then(member_identifier_parser())
        .map(|((a, b), c)| (a, b, c));
    _hierarchy_parser
        .or_not()
        .then(constant_bit_select_parser(
            constant_expression_parser.clone(),
        ))
        .then(
            token(Token::Bracket)
                .then(constant_part_select_range_parser(
                    constant_expression_parser,
                ))
                .then(token(Token::EBracket))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .map(|((a, b), c)| ConstantSelect(a, b, c))
}
