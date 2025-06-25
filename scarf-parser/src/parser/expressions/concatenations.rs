// =======================================================================
// concatenations.rs
// =======================================================================
// Parsing for 1800-2023 A.8.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn concatenation_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, Concatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Brace)
        .then(expression_parser.clone())
        .then(
            token(Token::Comma)
                .then(expression_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, Expression<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| Concatenation(a, b, c, d))
        .boxed()
}

pub fn constant_concatenation_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Brace)
        .then(constant_expression_parser.clone())
        .then(
            token(Token::Comma)
                .then(constant_expression_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, ConstantExpression<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ConstantConcatenation(a, b, c, d))
        .boxed()
}

pub fn constant_multiple_concatenation_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantMultipleConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Brace)
        .then(constant_expression_parser.clone())
        .then(constant_concatenation_parser(constant_expression_parser))
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ConstantMultipleConcatenation(a, b, c, d))
        .boxed()
}

pub fn multiple_concatenation_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, MultipleConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Brace)
        .then(expression_parser.clone())
        .then(concatenation_parser(expression_parser))
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| MultipleConcatenation(a, b, c, d))
        .boxed()
}

pub fn streaming_concatenation_parser<'a, I>()
-> impl Parser<'a, I, StreamingConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn stream_operator_parser<'a, I>()
-> impl Parser<'a, I, StreamOperator<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::GtGt).map(|a| StreamOperator::Right(a)),
        token(Token::LtLt).map(|a| StreamOperator::Left(a)),
    ))
}

pub fn empty_unpacked_array_concatenation_parser<'a, I>()
-> impl Parser<'a, I, EmptyUnpackedArrayConcatenation<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Brace)
        .then(token(Token::EBrace))
        .map(|(a, b)| EmptyUnpackedArrayConcatenation(a, b))
}
