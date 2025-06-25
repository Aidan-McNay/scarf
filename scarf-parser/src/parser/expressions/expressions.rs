// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3

use crate::*;
use chumsky::input::MapExtra;
use chumsky::pratt::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn inc_or_dec_expression_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, IncOrDecExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _preop_parser = inc_or_dec_operator_parser()
        .then(attribute_instance_vec_parser())
        .then(variable_lvalue_parser(expression_parser.clone()))
        .map(|((a, b), c)| IncOrDecExpression::Preop(Box::new((a, b, c))));
    let _postop_parser = variable_lvalue_parser(expression_parser.clone())
        .then(attribute_instance_vec_parser())
        .then(inc_or_dec_operator_parser())
        .map(|((a, b), c)| IncOrDecExpression::Postop(Box::new((a, b, c))));
    choice((_preop_parser, _postop_parser)).boxed()
}

pub fn conditional_expression_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, ConditionalExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    cond_predicate_parser(expression_parser.clone())
        .then(token(Token::Quest))
        .then(attribute_instance_vec_parser())
        .then(expression_parser.clone())
        .then(token(Token::Colon))
        .then(expression_parser)
        .map(|(((((a, b), c), d), e), f)| ConditionalExpression(a, b, c, d, e, f))
        .boxed()
}

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
                ConstantMintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))
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

pub fn tagged_union_expression_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, TaggedUnionExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Tagged)
        .then(member_identifier_parser())
        .then(primary_parser(expression_parser).or_not())
        .map(|((a, b), c)| TaggedUnionExpression(a, b, c))
        .boxed()
}

pub fn inside_expression_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, InsideExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    expression_parser
        .clone()
        .then(token(Token::Inside))
        .then(token(Token::Brace))
        .then(range_list_parser(expression_parser))
        .then(token(Token::EBrace))
        .map(|((((a, b), c), d), e)| InsideExpression(a, b, c, d, e))
        .boxed()
}

pub fn mintypmax_expression_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, MintypmaxExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
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

pub fn module_path_conditional_expression_parser<'a, I>(
    module_path_expression_parser: impl Parser<'a, I, ModulePathExpression<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, ModulePathConditionalExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
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

pub fn module_path_expression_parser<'a, I>()
-> impl Parser<'a, I, ModulePathExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
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
    let _binop_map = |l,
                      (op, attr),
                      r,
                      _: &mut MapExtra<'a, '_, I, extra::Err<Rich<'a, Token<'a>>>>|
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

pub fn module_path_mintypmax_expression_parser<'a, I>(
    module_path_expression_parser: impl Parser<'a, I, ModulePathExpression<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, I, ModulePathMintypmaxExpression<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
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

pub fn part_select_range_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, PartSelectRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        constant_range_parser(constant_expression_parser())
            .map(|a| PartSelectRange::ConstantRange(Box::new(a))),
        indexed_range_parser(expression_parser).map(|a| PartSelectRange::IndexedRange(Box::new(a))),
    ))
    .boxed()
}

pub fn indexed_range_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, IndexedRange<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        expression_parser
            .clone()
            .then(token(Token::PlusColon))
            .then(constant_expression_parser())
            .map(|((a, b), c)| IndexedRange::Plus(Box::new((a, b, c)))),
        expression_parser
            .clone()
            .then(token(Token::MinusColon))
            .then(constant_expression_parser())
            .map(|((a, b), c)| IndexedRange::Minus(Box::new((a, b, c)))),
    ))
    .boxed()
}
