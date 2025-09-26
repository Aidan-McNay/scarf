// =======================================================================
// expressions.rs
// =======================================================================
// Parsing for 1800-2023 A.8.3
//
// Pratt parsing adapted from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt};

pub fn inc_or_dec_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IncOrDecExpression<'s>, VerboseError<'s>> {
    let _preop_parser = (
        inc_or_dec_operator_parser,
        attribute_instance_vec_parser,
        variable_lvalue_parser,
    )
        .map(|(a, b, c)| IncOrDecExpression::Preop(Box::new((a, b, c))));
    let _postop_parser = (
        variable_lvalue_parser,
        attribute_instance_vec_parser,
        inc_or_dec_operator_parser,
    )
        .map(|(a, b, c)| IncOrDecExpression::Postop(Box::new((a, b, c))));
    alt((_preop_parser, _postop_parser)).parse_next(input)
}

pub fn conditional_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConditionalExpression<'s>, VerboseError<'s>> {
    (
        cond_predicate_parser,
        token(Token::Quest),
        attribute_instance_vec_parser,
        expression_parser,
        token(Token::Colon),
        expression_parser,
    )
        .map(|(a, b, c, d, e, f)| ConditionalExpression(a, b, c, d, e, f))
        .parse_next(input)
}

enum BinaryOrTernaryOp<'a> {
    Binary(BinaryOperator<'a>),
    Ternary(Metadata<'a>),
}

#[inline(always)]
fn left_assoc(bp: u8) -> (u8, u8) {
    let scaled_bp = bp * 2;
    (scaled_bp - 1, scaled_bp)
}

#[inline(always)]
fn right_assoc(bp: u8) -> (u8, u8) {
    let scaled_bp = bp * 2;
    (scaled_bp, scaled_bp - 1)
}

#[inline]
fn binary_operator_binding_power<'s>(binop: &BinaryOperator<'s>) -> (u8, u8) {
    match binop {
        BinaryOperator::StarStar(_) => left_assoc(13),
        BinaryOperator::Star(_) => left_assoc(12),
        BinaryOperator::Slash(_) => left_assoc(12),
        BinaryOperator::Percent(_) => left_assoc(12),
        BinaryOperator::Plus(_) => left_assoc(11),
        BinaryOperator::Minus(_) => left_assoc(11),
        BinaryOperator::GtGt(_) => left_assoc(10),
        BinaryOperator::LtLt(_) => left_assoc(10),
        BinaryOperator::GtGtGt(_) => left_assoc(10),
        BinaryOperator::LtLtLt(_) => left_assoc(10),
        BinaryOperator::Lt(_) => left_assoc(9),
        BinaryOperator::LtEq(_) => left_assoc(9),
        BinaryOperator::Gt(_) => left_assoc(9),
        BinaryOperator::GtEq(_) => left_assoc(9),
        BinaryOperator::EqEq(_) => left_assoc(8),
        BinaryOperator::ExclEq(_) => left_assoc(8),
        BinaryOperator::EqEqEq(_) => left_assoc(8),
        BinaryOperator::ExclEqEq(_) => left_assoc(8),
        BinaryOperator::EqEqQuest(_) => left_assoc(8),
        BinaryOperator::ExclEqQuest(_) => left_assoc(8),
        BinaryOperator::Amp(_) => left_assoc(7),
        BinaryOperator::Caret(_) => left_assoc(6),
        BinaryOperator::CaretTilde(_) => left_assoc(6),
        BinaryOperator::TildeCaret(_) => left_assoc(6),
        BinaryOperator::Pipe(_) => left_assoc(5),
        BinaryOperator::AmpAmp(_) => left_assoc(4),
        BinaryOperator::PipePipe(_) => left_assoc(3),
        BinaryOperator::MinusGt(_) => right_assoc(1),
        BinaryOperator::LtMinusGt(_) => right_assoc(1),
    }
}

#[inline(always)]
fn ternary_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(2)
}

fn constant_expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<ConstantExpression<'s>, VerboseError<'s>> {
    let mut lhs = alt((
        constant_primary_parser
            .map(|a| ConstantExpression::Primary(Box::new(a))),
        (
            unary_operator_parser,
            attribute_instance_vec_parser,
            constant_primary_parser,
        )
            .map(|(a, b, c)| ConstantExpression::Unary(Box::new((a, b, c)))),
    ))
    .parse_next(input)?;
    loop {
        let (op, r_bp) = alt((
            binary_operator_parser.verify_map(|a| {
                let (l_bp, r_bp) = binary_operator_binding_power(&a);
                if l_bp < min_bp {
                    return None;
                }
                Some((BinaryOrTernaryOp::Binary(a), r_bp))
            }),
            token(Token::Quest).verify_map(|a| {
                let (l_bp, r_bp) = ternary_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((BinaryOrTernaryOp::Ternary(a), r_bp))
            }),
        ))
        .parse_next(input)?;
        lhs = match op {
            BinaryOrTernaryOp::Binary(binop) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let rhs = constant_expression_bp_parser(input, r_bp)?;
                ConstantExpression::Binary(Box::new((lhs, binop, attrs, rhs)))
            }
            BinaryOrTernaryOp::Ternary(quest) => {
                let attrs = attribute_instance_vec_parser(input)?;
                let mhs = constant_expression_bp_parser(input, 0)?;
                let colon = token(Token::Colon)(input)?;
                let rhs = constant_expression_bp_parser(input, r_bp)?;
                ConstantExpression::Ternary(Box::new((
                    lhs, quest, attrs, mhs, colon, rhs,
                )))
            }
        }
    }
}

pub fn constant_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantExpression<'s>, VerboseError<'s>> {
    constant_expression_bp_parser(input, 0)
}

pub fn constant_mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantMintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            constant_expression_parser,
            token(Token::Colon),
            constant_expression_parser,
            token(Token::Colon),
            constant_expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                ConstantMintypmaxExpression::Mintypmax(Box::new((
                    a, b, c, d, e,
                )))
            }),
        constant_expression_parser
            .map(|a| ConstantMintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_param_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantParamExpression<'s>, VerboseError<'s>> {
    let _mintypmax_parser = constant_mintypmax_expression_parser
        .map(|a| ConstantParamExpression::Mintypmax(Box::new(a)));
    let _data_parser =
        data_type_parser.map(|a| ConstantParamExpression::Data(Box::new(a)));
    let _dollar_parser = token(Token::Dollar)
        .map(|a| ConstantParamExpression::Dollar(Box::new(a)));
    alt((_mintypmax_parser, _data_parser, _dollar_parser)).parse_next(input)
}

pub fn param_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ParamExpression<'s>, VerboseError<'s>> {
    let _mintypmax_parser = mintypmax_expression_parser
        .map(|a| ParamExpression::Mintypmax(Box::new(a)));
    let _data_parser =
        data_type_parser.map(|a| ParamExpression::Data(Box::new(a)));
    let _dollar_parser =
        token(Token::Dollar).map(|a| ParamExpression::Dollar(Box::new(a)));
    alt((_mintypmax_parser, _data_parser, _dollar_parser)).parse_next(input)
}

pub fn constant_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantRangeExpression<'s>, VerboseError<'s>> {
    alt((
        constant_expression_parser
            .map(|a| ConstantRangeExpression::Expression(Box::new(a))),
        constant_part_select_range_parser
            .map(|a| ConstantRangeExpression::PartSelectRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_part_select_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantPartSelectRange<'s>, VerboseError<'s>> {
    alt((
        constant_range_parser
            .map(|a| ConstantPartSelectRange::Range(Box::new(a))),
        constant_indexed_range_parser
            .map(|a| ConstantPartSelectRange::IndexedRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn constant_indexed_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantIndexedRange<'s>, VerboseError<'s>> {
    let _plus_parser = (
        constant_expression_parser,
        token(Token::PlusColon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantIndexedRange::Plus(Box::new((a, b, c))));
    let _minus_parser = (
        constant_expression_parser,
        token(Token::MinusColon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantIndexedRange::Minus(Box::new((a, b, c))));
    alt((_plus_parser, _minus_parser)).parse_next(input)
}

pub fn constant_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantRange<'s>, VerboseError<'s>> {
    (
        constant_expression_parser,
        token(Token::Colon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| ConstantRange(a, b, c))
        .parse_next(input)
}

// Paused here...

pub fn expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Expression<'s>, VerboseError<'s>> {
    let _primary_parser =
        primary_parser.map(|a| Expression::Primary(Box::new(a)));
    let _unary_parser = (
        unary_operator_parser,
        attribute_instance_vec_parser,
        primary_parser,
    )
        .map(|(a, b, c)| Expression::Unary(Box::new((a, b, c))));
    let _inc_or_dec_expression_parser = inc_or_dec_expression_parser
        .map(|a| Expression::IncOrDecExpression(Box::new(a)));
    let _operator_assignment_parser = (
        token(Token::Paren),
        operator_assignment_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| Expression::OperatorAssignment(Box::new((a, b, c))));
    // let _binary_parser =
    //     todo.map(|(a, b, c, d)| Expression::Binary(Box::new((a, b, c, d))));
    let _conditional_expression_parser = conditional_expression_parser
        .map(|a| Expression::ConditionalExpression(Box::new(a)));
    let _inside_expression_parser = inside_expression_parser
        .map(|a| Expression::InsideExpression(Box::new(a)));
    let _tagged_union_expression_parser = tagged_union_expression_parser
        .map(|a| Expression::TaggedUnionExpression(Box::new(a)));
    alt((
        _primary_parser,
        _unary_parser,
        _inc_or_dec_expression_parser,
        _operator_assignment_parser,
        // _binary_parser,
        _conditional_expression_parser,
        _inside_expression_parser,
        _tagged_union_expression_parser,
    ))
    .parse_next(input)
}

pub fn tagged_union_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaggedUnionExpression<'s>, VerboseError<'s>> {
    (
        token(Token::Tagged),
        member_identifier_parser,
        opt(primary_parser),
    )
        .map(|(a, b, c)| TaggedUnionExpression(a, b, c))
        .parse_next(input)
}

pub fn inside_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InsideExpression<'s>, VerboseError<'s>> {
    (
        expression_parser,
        token(Token::Inside),
        token(Token::Brace),
        range_list_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| InsideExpression(a, b, c, d, e))
        .parse_next(input)
}

pub fn mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            expression_parser,
            token(Token::Colon),
            expression_parser,
            token(Token::Colon),
            expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                MintypmaxExpression::Mintypmax(Box::new((a, b, c, d, e)))
            }),
        expression_parser.map(|a| MintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn module_path_conditional_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathConditionalExpression<'s>, VerboseError<'s>> {
    (
        module_path_expression_parser,
        token(Token::Quest),
        attribute_instance_vec_parser,
        module_path_expression_parser,
        token(Token::Colon),
        module_path_expression_parser,
    )
        .map(|(a, b, c, d, e, f)| {
            ModulePathConditionalExpression(a, b, c, d, e, f)
        })
        .parse_next(input)
}

pub fn module_path_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathExpression<'s>, VerboseError<'s>> {
    let _primary_parser = module_path_primary_parser
        .map(|a| ModulePathExpression::Primary(Box::new(a)));
    let _unary_parser = (
        unary_module_path_operator_parser,
        attribute_instance_vec_parser,
        module_path_primary_parser,
    )
        .map(|(a, b, c)| ModulePathExpression::Unary(Box::new((a, b, c))));
    // let _binary_parser = todo.map(|(a, b, c, d)| {
    //     ModulePathExpression::Binary(Box::new((a, b, c, d)))
    // });
    let _conditional_parser = module_path_conditional_expression_parser
        .map(|a| ModulePathExpression::Conditional(Box::new(a)));
    alt((
        _primary_parser,
        _unary_parser,
        // _binary_parser,
        _conditional_parser,
    ))
    .parse_next(input)
}

pub fn module_path_mintypmax_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathMintypmaxExpression<'s>, VerboseError<'s>> {
    alt((
        (
            module_path_expression_parser,
            token(Token::Colon),
            module_path_expression_parser,
            token(Token::Colon),
            module_path_expression_parser,
        )
            .map(|(a, b, c, d, e)| {
                ModulePathMintypmaxExpression::Mintypmax(Box::new((
                    a, b, c, d, e,
                )))
            }),
        module_path_expression_parser
            .map(|a| ModulePathMintypmaxExpression::Single(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn part_select_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PartSelectRange<'s>, VerboseError<'s>> {
    alt((
        constant_range_parser
            .map(|a| PartSelectRange::ConstantRange(Box::new(a))),
        indexed_range_parser
            .map(|a| PartSelectRange::IndexedRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn indexed_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IndexedRange<'s>, VerboseError<'s>> {
    alt((
        (
            expression_parser,
            token(Token::PlusColon),
            constant_expression_parser,
        )
            .map(|(a, b, c)| IndexedRange::Plus(Box::new((a, b, c)))),
        (
            expression_parser,
            token(Token::MinusColon),
            constant_expression_parser,
        )
            .map(|(a, b, c)| IndexedRange::Minus(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}
