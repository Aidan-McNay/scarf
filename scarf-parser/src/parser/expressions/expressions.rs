// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3

use crate::*;
use chumsky::input::MapExtra;
use chumsky::pratt::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_expression_parser<'a, I>()
-> impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let mut parser = Recursive::declare();
    let _attribute_instance_vec_parser = attribute_instance_parser(parser.clone())
        .repeated()
        .collect::<Vec<AttributeInstance>>();
    let _primary_parser =
        constant_primary_parser(parser.clone()).map(|a| ConstantExpression::Primary(Box::new(a)));
    let _unary_parser = unary_operator_parser()
        .then(_attribute_instance_vec_parser.clone())
        .then(constant_primary_parser(parser.clone()))
        .map(|((a, b), c)| ConstantExpression::Unary(Box::new((a, b, c))));
    let _binop = |c, b: fn(Metadata<'a>) -> BinaryOperator<'a>| {
        token(c)
            .map(move |a| b(a))
            .then(_attribute_instance_vec_parser.clone())
    };
    let _binop_map =
        |l,
         (op, attr),
         r,
         _: &mut MapExtra<'a, '_, I, extra::Err<Rich<'a, Token<'a>>>>|
         -> ConstantExpression { ConstantExpression::Binary(Box::new((l, op, attr, r))) };
    let _binary_parser_1 = parser.clone().pratt((
        infix(
            left(11),
            _binop(Token::StarStar, BinaryOperator::StarStar as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(10),
            _binop(Token::Star, BinaryOperator::Star as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(10),
            _binop(Token::Slash, BinaryOperator::Slash as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(10),
            _binop(Token::Percent, BinaryOperator::Percent as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(9),
            _binop(Token::Plus, BinaryOperator::Plus as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(9),
            _binop(Token::Minus, BinaryOperator::Minus as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(8),
            _binop(Token::LtLt, BinaryOperator::LtLt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(8),
            _binop(Token::GtGt, BinaryOperator::GtGt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(8),
            _binop(Token::LtLtLt, BinaryOperator::LtLtLt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(8),
            _binop(Token::GtGtGt, BinaryOperator::GtGtGt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(7),
            _binop(Token::Lt, BinaryOperator::Lt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(7),
            _binop(Token::Gt, BinaryOperator::Gt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(7),
            _binop(Token::LtEq, BinaryOperator::LtEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(7),
            _binop(Token::GtEq, BinaryOperator::GtEq as fn(_) -> _),
            _binop_map,
        ),
    ));
    let _binary_parser_2 = parser.clone().pratt((
        infix(
            left(6),
            _binop(Token::EqEq, BinaryOperator::EqEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(Token::ExclEq, BinaryOperator::ExclEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(Token::EqEqEq, BinaryOperator::EqEqEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(Token::ExclEqEq, BinaryOperator::ExclEqEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(Token::EqEqQuest, BinaryOperator::EqEqQuest as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(
                Token::ExclEqQuest,
                BinaryOperator::ExclEqQuest as fn(_) -> _,
            ),
            _binop_map,
        ),
        infix(
            left(5),
            _binop(Token::Amp, BinaryOperator::Amp as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(Token::Caret, BinaryOperator::Caret as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(Token::TildeCaret, BinaryOperator::TildeCaret as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(Token::CaretTilde, BinaryOperator::CaretTilde as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(3),
            _binop(Token::Pipe, BinaryOperator::Pipe as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(2),
            _binop(Token::AmpAmp, BinaryOperator::AmpAmp as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(1),
            _binop(Token::PipePipe, BinaryOperator::PipePipe as fn(_) -> _),
            _binop_map,
        ),
    ));
    let _binary_parser = choice((_binary_parser_1, _binary_parser_2));
    let _ternary_parser = parser
        .clone()
        .then(token(Token::Quest))
        .then(_attribute_instance_vec_parser.clone())
        .then(parser.clone())
        .then(token(Token::Colon))
        .then(parser.clone())
        .map(|(((((a, b), c), d), e), f)| {
            ConstantExpression::Ternary(Box::new((a, b, c, d, e, f)))
        });
    let _binary_parser_3 = parser.clone().pratt((
        infix(
            left(1),
            _binop(Token::MinusGt, BinaryOperator::MinusGt as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(1),
            _binop(Token::LtMinusGt, BinaryOperator::LtMinusGt as fn(_) -> _),
            _binop_map,
        ),
    ));
    parser.define(choice((
        _primary_parser,
        _unary_parser,
        _binary_parser,
        _ternary_parser,
        _binary_parser_3,
    )));
    parser.boxed()
}

pub fn constant_mintypmax_expression_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantMintypmaxExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        constant_expression_parser
            .clone()
            .then(token(Token::Colon))
            .then(constant_expression_parser.clone())
            .then(token(Token::Colon))
            .then(constant_expression_parser.clone())
            .map(|((((a, b), c), d), e)| {
                ConstantMintypmaxExpression::MinTypMax(Box::new((a, b, c, d, e)))
            }),
        constant_expression_parser.map(|a| ConstantMintypmaxExpression::Single(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_param_expression_parser<'a, I>(
    _constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, ConstantParamExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn constant_range_expression_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantRangeExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        constant_expression_parser
            .clone()
            .map(|a| ConstantRangeExpression::Expression(Box::new(a))),
        constant_part_select_range_parser(constant_expression_parser)
            .map(|a| ConstantRangeExpression::PartSelectRange(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_part_select_range_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantPartSelectRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        constant_range_parser(constant_expression_parser.clone())
            .map(|a| ConstantPartSelectRange::Range(Box::new(a))),
        constant_indexed_range_parser(constant_expression_parser)
            .map(|a| ConstantPartSelectRange::IndexedRange(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_indexed_range_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantIndexedRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _plus_parser = constant_expression_parser
        .clone()
        .then(token(Token::PlusColon))
        .then(constant_expression_parser.clone())
        .map(|((a, b), c)| ConstantIndexedRange::Plus(Box::new((a, b, c))));
    let _minus_parser = constant_expression_parser
        .clone()
        .then(token(Token::MinusColon))
        .then(constant_expression_parser.clone())
        .map(|((a, b), c)| ConstantIndexedRange::Minus(Box::new((a, b, c))));
    choice((_plus_parser, _minus_parser)).boxed()
}

pub fn constant_range_parser<'a, I>(
    constant_expression_parser: impl Parser<'a, I, ConstantExpression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConstantRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    constant_expression_parser
        .clone()
        .then(token(Token::Colon))
        .then(constant_expression_parser)
        .map(|((a, b), c)| ConstantRange(a, b, c))
        .boxed()
}

pub fn expression_parser<'a, I>() -> impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Error)
}
