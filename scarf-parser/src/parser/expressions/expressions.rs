// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3
//
// Unlike other parsers, we cache the expression parsers due to their
// heavy use

use crate::*;
use chumsky::{input::MapExtra, pratt::*, prelude::*};
use scarf_syntax::*;

pub fn inc_or_dec_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, IncOrDecExpression<'a>, ParserError<'a>> + Clone {
    let _attribute_instance_vec_parser =
        attribute_instance_parser(constant_expression_parser(expression_parser.clone()))
            .repeated()
            .collect::<Vec<AttributeInstance>>();
    let _preop_parser = inc_or_dec_operator_parser()
        .then(_attribute_instance_vec_parser.clone())
        .then(variable_lvalue_parser(expression_parser.clone()))
        .map(|((a, b), c)| IncOrDecExpression::Preop(Box::new((a, b, c))));
    let _postop_parser = variable_lvalue_parser(expression_parser.clone())
        .then(_attribute_instance_vec_parser)
        .then(inc_or_dec_operator_parser())
        .map(|((a, b), c)| IncOrDecExpression::Postop(Box::new((a, b, c))));
    choice((_preop_parser, _postop_parser)).boxed()
}

pub fn conditional_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConditionalExpression<'a>, ParserError<'a>> + Clone {
    let _attribute_instance_vec_parser =
        attribute_instance_parser(constant_expression_parser(expression_parser.clone()))
            .repeated()
            .collect::<Vec<AttributeInstance>>();
    cond_predicate_parser(expression_parser.clone())
        .then(token(Token::Quest))
        .then(_attribute_instance_vec_parser)
        .then(expression_parser.clone())
        .then(token(Token::Colon))
        .then(expression_parser)
        .map(|(((((a, b), c), d), e), f)| ConditionalExpression(a, b, c, d, e, f))
        .boxed()
}

pub fn constant_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantExpression<'a>, ParserError<'a>> + Clone {
    let mut parser = Recursive::declare();
    let _attribute_instance_vec_parser = attribute_instance_parser(parser.clone())
        .repeated()
        .collect::<Vec<AttributeInstance>>();
    let _primary_parser = constant_primary_parser(parser.clone(), expression_parser.clone())
        .map(|a| ConstantExpression::Primary(Box::new(a)));
    let _unary_parser = unary_operator_parser()
        .then(_attribute_instance_vec_parser.clone())
        .then(constant_primary_parser(parser.clone(), expression_parser))
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
         _: &mut MapExtra<'a, '_, ParserInput<'a>, extra::Err<Rich<'a, Token<'a>>>>|
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

pub fn constant_mintypmax_expression_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantMintypmaxExpression<'a>, ParserError<'a>> + Clone {
    choice((
        constant_expression_parser
            .clone()
            .then(token(Token::Colon))
            .then(constant_expression_parser.clone())
            .then(token(Token::Colon))
            .then(constant_expression_parser.clone())
            .map(|((((a, b), c), d), e)| {
                ConstantMintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))
            }),
        constant_expression_parser.map(|a| ConstantMintypmaxExpression::Single(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_param_expression_parser<'a>(
    _constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantParamExpression<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn constant_range_expression_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantRangeExpression<'a>, ParserError<'a>> + Clone {
    choice((
        constant_expression_parser
            .clone()
            .map(|a| ConstantRangeExpression::Expression(Box::new(a))),
        constant_part_select_range_parser(constant_expression_parser)
            .map(|a| ConstantRangeExpression::PartSelectRange(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_part_select_range_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantPartSelectRange<'a>, ParserError<'a>> + Clone {
    choice((
        constant_range_parser(constant_expression_parser.clone())
            .map(|a| ConstantPartSelectRange::Range(Box::new(a))),
        constant_indexed_range_parser(constant_expression_parser)
            .map(|a| ConstantPartSelectRange::IndexedRange(Box::new(a))),
    ))
    .boxed()
}

pub fn constant_indexed_range_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantIndexedRange<'a>, ParserError<'a>> + Clone {
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

pub fn constant_range_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantRange<'a>, ParserError<'a>> + Clone {
    constant_expression_parser
        .clone()
        .then(token(Token::Colon))
        .then(constant_expression_parser)
        .map(|((a, b), c)| ConstantRange(a, b, c))
        .boxed()
}

pub fn expression_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone {
    let mut parser = Recursive::declare();
    let _attribute_instance_vec_parser =
        attribute_instance_parser(constant_expression_parser(parser.clone()))
            .repeated()
            .collect::<Vec<AttributeInstance>>();
    let _primary_parser = primary_parser(parser.clone()).map(|a| Expression::Primary(Box::new(a)));
    let _unary_parser = unary_operator_parser()
        .then(_attribute_instance_vec_parser.clone())
        .then(primary_parser(parser.clone()))
        .map(|((a, b), c)| Expression::Unary(Box::new((a, b, c))));
    let _inc_or_dec_expression_parser = inc_or_dec_expression_parser(parser.clone())
        .map(|a| Expression::IncOrDecExpression(Box::new(a)));
    let _operator_assignment_parser = token(Token::Paren)
        .then(operator_assignment_parser(parser.clone()))
        .then(token(Token::EParen))
        .map(|((a, b), c)| Expression::OperatorAssignment(Box::new((a, b, c))));
    let _binop = |c, b: fn(Metadata<'a>) -> BinaryOperator<'a>| {
        token(c)
            .map(move |a| b(a))
            .then(_attribute_instance_vec_parser.clone())
    };
    let _binop_map =
        |l,
         (op, attr),
         r,
         _: &mut MapExtra<'a, '_, ParserInput<'a>, extra::Err<Rich<'a, Token<'a>>>>|
         -> Expression { Expression::Binary(Box::new((l, op, attr, r))) };
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
    let _conditional_expression_parser = conditional_expression_parser(parser.clone())
        .map(|a| Expression::ConditionalExpression(Box::new(a)));
    let _inside_expression_parser =
        inside_expression_parser(parser.clone()).map(|a| Expression::InsideExpression(Box::new(a)));
    let _tagged_union_expression_parser = tagged_union_expression_parser(parser.clone())
        .map(|a| Expression::TaggedUnionExpression(Box::new(a)));
    parser.define(choice((
        _primary_parser,
        _unary_parser,
        _inc_or_dec_expression_parser,
        _operator_assignment_parser,
        _binary_parser,
        _conditional_expression_parser,
        _inside_expression_parser,
        _tagged_union_expression_parser,
    )));
    parser.boxed()
}

pub fn tagged_union_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, TaggedUnionExpression<'a>, ParserError<'a>> + Clone {
    token(Token::Tagged)
        .then(member_identifier_parser())
        .then(primary_parser(expression_parser).or_not())
        .map(|((a, b), c)| TaggedUnionExpression(a, b, c))
        .boxed()
}

pub fn inside_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, InsideExpression<'a>, ParserError<'a>> + Clone {
    expression_parser
        .clone()
        .then(token(Token::Inside))
        .then(token(Token::Brace))
        .then(range_list_parser(expression_parser))
        .then(token(Token::EBrace))
        .map(|((((a, b), c), d), e)| InsideExpression(a, b, c, d, e))
        .boxed()
}

pub fn mintypmax_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, MintypmaxExpression<'a>, ParserError<'a>> + Clone {
    choice((
        expression_parser
            .clone()
            .then(token(Token::Colon))
            .then(expression_parser.clone())
            .then(token(Token::Colon))
            .then(expression_parser.clone())
            .map(|((((a, b), c), d), e)| MintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))),
        expression_parser.map(|a| MintypmaxExpression::Single(Box::new(a))),
    ))
    .boxed()
}

pub fn module_path_conditional_expression_parser<'a>(
    module_path_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ModulePathExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ModulePathConditionalExpression<'a>, ParserError<'a>> + Clone
{
    module_path_expression_parser
        .clone()
        .then(token(Token::Quest))
        .then(attribute_instance_vec_parser())
        .then(module_path_expression_parser.clone())
        .then(token(Token::Colon))
        .then(module_path_expression_parser)
        .map(|(((((a, b), c), d), e), f)| ModulePathConditionalExpression(a, b, c, d, e, f))
        .boxed()
}

pub fn module_path_expression_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModulePathExpression<'a>, ParserError<'a>> + Clone {
    let mut parser = Recursive::declare();
    let _primary_parser = module_path_primary_parser(parser.clone())
        .map(|a| ModulePathExpression::Primary(Box::new(a)));
    let _unary_parser = unary_module_path_operator_parser()
        .then(attribute_instance_vec_parser())
        .then(module_path_primary_parser(parser.clone()))
        .map(|((a, b), c)| ModulePathExpression::Unary(Box::new((a, b, c))));
    let _binop = |c, b: fn(Metadata<'a>) -> BinaryModulePathOperator<'a>| {
        token(c)
            .map(move |a| b(a))
            .then(attribute_instance_vec_parser())
    };
    let _binop_map =
        |l,
         (op, attr),
         r,
         _: &mut MapExtra<'a, '_, ParserInput<'a>, extra::Err<Rich<'a, Token<'a>>>>|
         -> ModulePathExpression {
            ModulePathExpression::Binary(Box::new((l, op, attr, r)))
        };
    let _binary_parser = parser.clone().pratt((
        infix(
            left(6),
            _binop(Token::EqEq, BinaryModulePathOperator::EqEq as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(6),
            _binop(
                Token::ExclEq,
                BinaryModulePathOperator::ExclEq as fn(_) -> _,
            ),
            _binop_map,
        ),
        infix(
            left(5),
            _binop(Token::Amp, BinaryModulePathOperator::Amp as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(Token::Caret, BinaryModulePathOperator::Caret as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(
                Token::CaretTilde,
                BinaryModulePathOperator::CaretTilde as fn(_) -> _,
            ),
            _binop_map,
        ),
        infix(
            left(4),
            _binop(
                Token::TildeCaret,
                BinaryModulePathOperator::TildeCaret as fn(_) -> _,
            ),
            _binop_map,
        ),
        infix(
            left(3),
            _binop(Token::Pipe, BinaryModulePathOperator::Pipe as fn(_) -> _),
            _binop_map,
        ),
        infix(
            left(2),
            _binop(
                Token::AmpAmp,
                BinaryModulePathOperator::AmpAmp as fn(_) -> _,
            ),
            _binop_map,
        ),
        infix(
            left(1),
            _binop(
                Token::PipePipe,
                BinaryModulePathOperator::PipePipe as fn(_) -> _,
            ),
            _binop_map,
        ),
    ));
    let _conditional_parser = module_path_conditional_expression_parser(parser.clone())
        .map(|a| ModulePathExpression::Conditional(Box::new(a)));
    parser.define(choice((
        _primary_parser,
        _unary_parser,
        _binary_parser,
        _conditional_parser,
    )));
    parser.boxed()
}

pub fn module_path_mintypmax_expression_parser<'a>(
    module_path_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ModulePathExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ModulePathMintypmaxExpression<'a>, ParserError<'a>> + Clone {
    choice((
        module_path_expression_parser
            .clone()
            .then(token(Token::Colon))
            .then(module_path_expression_parser.clone())
            .then(token(Token::Colon))
            .then(module_path_expression_parser.clone())
            .map(|((((a, b), c), d), e)| {
                ModulePathMintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))
            }),
        module_path_expression_parser.map(|a| ModulePathMintypmaxExpression::Single(Box::new(a))),
    ))
    .boxed()
}

pub fn part_select_range_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, PartSelectRange<'a>, ParserError<'a>> + Clone {
    choice((
        constant_range_parser(constant_expression_parser(expression_parser.clone()))
            .map(|a| PartSelectRange::ConstantRange(Box::new(a))),
        indexed_range_parser(expression_parser).map(|a| PartSelectRange::IndexedRange(Box::new(a))),
    ))
    .boxed()
}

pub fn indexed_range_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, IndexedRange<'a>, ParserError<'a>> + Clone {
    choice((
        expression_parser
            .clone()
            .then(token(Token::PlusColon))
            .then(constant_expression_parser(expression_parser.clone()))
            .map(|((a, b), c)| IndexedRange::Plus(Box::new((a, b, c)))),
        expression_parser
            .clone()
            .then(token(Token::MinusColon))
            .then(constant_expression_parser(expression_parser))
            .map(|((a, b), c)| IndexedRange::Minus(Box::new((a, b, c)))),
    ))
    .boxed()
}
