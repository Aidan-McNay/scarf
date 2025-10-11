// =======================================================================
// assertion_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.10

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, fail, opt, repeat};

pub fn concurrent_assertion_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionItem<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn concurrent_assertion_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConcurrentAssertionStatement<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn assertion_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionItemDeclaration<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

fn basic_sequence_expr_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    let _start_delay_parser = (
        cycle_delay_range_parser,
        sequence_expr_parser,
        repeat(0.., (cycle_delay_range_parser, sequence_expr_parser)),
    )
        .map(|(a, b, c)| SequenceExpr::StartDelay(Box::new((a, b, c))));
    let _expr_parser = (expression_or_dist_parser, opt(boolean_abbrev_parser))
        .map(|(a, b)| SequenceExpr::Expr(Box::new((a, b))));
    let _paren_parser = (
        token(Token::Paren),
        sequence_expr_parser,
        repeat(0.., (token(Token::Comma), sequence_match_item_parser)),
        token(Token::EParen),
        opt(sequence_abbrev_parser),
    )
        .map(|(a, b, c, d, e)| SequenceExpr::Paren(Box::new((a, b, c, d, e))));
    let _first_match_parser = (
        token(Token::FirstMatch),
        token(Token::Paren),
        sequence_expr_parser,
        repeat(0.., (token(Token::Comma), sequence_match_item_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| {
            SequenceExpr::FirstMatch(Box::new((a, b, c, d, e)))
        });
    let _clocking_parser = (clocking_event_parser, sequence_expr_parser)
        .map(|(a, b)| SequenceExpr::Clocking(Box::new((a, b))));
    alt((
        _start_delay_parser,
        _expr_parser,
        _paren_parser,
        _first_match_parser,
        _clocking_parser,
    ))
    .parse_next(input)
}

enum SequencePrattOp<'a> {
    CycleDelayRange(CycleDelayRange<'a>),
    Within(Metadata<'a>),
    Intersect(Metadata<'a>),
    And(Metadata<'a>),
    Or(Metadata<'a>),
}

#[inline(always)]
fn throughout_binding_power<'s>() -> u8 {
    no_assoc(10)
}

#[inline(always)]
fn within_binding_power<'s>() -> (u8, u8) {
    left_assoc(9)
}

#[inline(always)]
fn intersect_binding_power<'s>() -> (u8, u8) {
    left_assoc(8)
}

#[inline(always)]
fn and_binding_power<'s>() -> (u8, u8) {
    left_assoc(6)
}

#[inline(always)]
fn or_binding_power<'s>() -> (u8, u8) {
    left_assoc(5)
}

fn sequence_expr_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    let mut lhs = alt((
        basic_sequence_expr_parser,
        (
            expression_or_dist_parser,
            token(Token::Throughout),
            |input: &mut Tokens<'s>| {
                sequence_expr_bp_parser(input, throughout_binding_power())
            },
        )
            .map(|(a, b, c)| SequenceExpr::Throughout(Box::new((a, b, c)))),
    ))
    .parse_next(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            cycle_delay_range_parser.map(|a| {
                (SequencePrattOp::CycleDelayRange(a), 200) // bp not used
            }),
            token(Token::Within).verify_map(|a| {
                let (l_bp, r_bp) = within_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Within(a), r_bp))
            }),
            token(Token::Intersect).verify_map(|a| {
                let (l_bp, r_bp) = intersect_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Intersect(a), r_bp))
            }),
            token(Token::And).verify_map(|a| {
                let (l_bp, r_bp) = and_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::And(a), r_bp))
            }),
            token(Token::Or).verify_map(|a| {
                let (l_bp, r_bp) = or_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SequencePrattOp::Or(a), r_bp))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            SequencePrattOp::CycleDelayRange(cycle_delay_range) => {
                let next_sequence_expr = sequence_expr_parser(input)?;
                let later_delays = repeat(
                    0..,
                    (cycle_delay_range_parser, sequence_expr_parser),
                )
                .parse_next(input)?;
                SequenceExpr::Delay(Box::new((
                    lhs,
                    cycle_delay_range,
                    next_sequence_expr,
                    later_delays,
                )))
            }
            SequencePrattOp::Within(within) => {
                let rhs = sequence_expr_bp_parser(input, r_bp)?;
                SequenceExpr::Within(Box::new((lhs, within, rhs)))
            }
            SequencePrattOp::Intersect(intersect) => {
                let rhs = sequence_expr_bp_parser(input, r_bp)?;
                SequenceExpr::Intersect(Box::new((lhs, intersect, rhs)))
            }
            SequencePrattOp::And(and) => {
                let rhs = sequence_expr_bp_parser(input, r_bp)?;
                SequenceExpr::And(Box::new((lhs, and, rhs)))
            }
            SequencePrattOp::Or(or) => {
                let rhs = sequence_expr_bp_parser(input, r_bp)?;
                SequenceExpr::Or(Box::new((lhs, or, rhs)))
            }
        }
    }
}

pub fn sequence_expr_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceExpr<'s>, VerboseError<'s>> {
    sequence_expr_bp_parser(input, 0)
}

pub fn cycle_delay_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelayRange<'s>, VerboseError<'s>> {
    let _primary_parser = (token(Token::PoundPound), constant_primary_parser)
        .map(|(a, b)| CycleDelayRange::Primary(Box::new((a, b))));
    let _range_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        cycle_delay_const_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Range(Box::new((a, b, c, d))));
    let _star_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        token(Token::Star),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Star(Box::new((a, b, c, d))));
    let _plus_parser = (
        token(Token::PoundPound),
        token(Token::Bracket),
        token(Token::Plus),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| CycleDelayRange::Plus(Box::new((a, b, c, d))));
    alt((_primary_parser, _range_parser, _star_parser, _plus_parser))
        .parse_next(input)
}

pub fn sequence_method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMethodCall<'s>, VerboseError<'s>> {
    (
        sequence_instance_parser,
        token(Token::Period),
        method_identifier_parser,
    )
        .map(|(a, b, c)| SequenceMethodCall(a, b, c))
        .parse_next(input)
}

pub fn sequence_match_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceMatchItem<'s>, VerboseError<'s>> {
    alt((
        operator_assignment_parser
            .map(|a| SequenceMatchItem::Operator(Box::new(a))),
        inc_or_dec_expression_parser
            .map(|a| SequenceMatchItem::IncOrDec(Box::new(a))),
        subroutine_call_parser
            .map(|a| SequenceMatchItem::Subroutine(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn sequence_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceInstance<'s>, VerboseError<'s>> {
    (
        ps_or_hierarchical_sequence_identifier_parser,
        opt((
            token(Token::Paren),
            opt(sequence_list_of_arguments_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b)| SequenceInstance(a, b))
        .parse_next(input)
}

pub fn sequence_list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceListOfArguments<'s>, VerboseError<'s>> {
    let _partial_identifier_parser = (
        opt(sequence_actual_arg_parser),
        repeat(0.., (token(Token::Comma), opt(sequence_actual_arg_parser))),
        repeat(
            0..,
            (
                token(Token::Comma),
                token(Token::Period),
                identifier_parser,
                token(Token::Paren),
                opt(sequence_actual_arg_parser),
                token(Token::EParen),
            ),
        ),
    )
        .map(|(a, b, c)| {
            SequenceListOfArguments::PartialIdentifier(Box::new((a, b, c)))
        });
    let _identifier_parser = (
        token(Token::Period),
        identifier_parser,
        token(Token::Paren),
        opt(sequence_actual_arg_parser),
        token(Token::EParen),
        repeat(
            0..,
            (
                token(Token::Comma),
                token(Token::Period),
                identifier_parser,
                token(Token::Paren),
                opt(sequence_actual_arg_parser),
                token(Token::EParen),
            ),
        ),
    )
        .map(|(a, b, c, d, e, f)| {
            SequenceListOfArguments::Identifier(Box::new((a, b, c, d, e, f)))
        });
    alt((_partial_identifier_parser, _identifier_parser)).parse_next(input)
}

pub fn sequence_actual_arg_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceActualArg<'s>, VerboseError<'s>> {
    alt((
        event_expression_parser.map(|a| SequenceActualArg::Event(Box::new(a))),
        sequence_expr_parser.map(|a| SequenceActualArg::Sequence(Box::new(a))),
        token(Token::Dollar).map(|a| SequenceActualArg::Dollar(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn boolean_abbrev_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BooleanAbbrev<'s>, VerboseError<'s>> {
    alt((
        consecutive_repetition_parser
            .map(|a| BooleanAbbrev::Consecutive(Box::new(a))),
        nonconsecutive_repetition_parser
            .map(|a| BooleanAbbrev::Nonconsecutive(Box::new(a))),
        goto_repetition_parser.map(|a| BooleanAbbrev::Goto(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn sequence_abbrev_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SequenceAbbrev<'s>, VerboseError<'s>> {
    consecutive_repetition_parser
        .map(|a| SequenceAbbrev(a))
        .parse_next(input)
}

pub fn consecutive_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConsecutiveRepetition<'s>, VerboseError<'s>> {
    let _expr_parser = (
        token(Token::Bracket),
        token(Token::Star),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| {
            ConsecutiveRepetition::Expr(Box::new((a, b, c, d)))
        });
    let _star_parser = (
        token(Token::Bracket),
        token(Token::Star),
        token(Token::EBracket),
    )
        .map(|(a, b, c)| ConsecutiveRepetition::Star(Box::new((a, b, c))));
    let _plus_parser = (
        token(Token::Bracket),
        token(Token::Plus),
        token(Token::EBracket),
    )
        .map(|(a, b, c)| ConsecutiveRepetition::Plus(Box::new((a, b, c))));
    alt((_expr_parser, _star_parser, _plus_parser)).parse_next(input)
}

pub fn nonconsecutive_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonconsecutiveRepetition<'s>, VerboseError<'s>> {
    (
        token(Token::Bracket),
        token(Token::Eq),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| NonconsecutiveRepetition(a, b, c, d))
        .parse_next(input)
}

pub fn goto_repetition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<GotoRepetition<'s>, VerboseError<'s>> {
    (
        token(Token::Bracket),
        token(Token::MinusGt),
        const_or_range_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d)| GotoRepetition(a, b, c, d))
        .parse_next(input)
}

pub fn const_or_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstOrRangeExpression<'s>, VerboseError<'s>> {
    alt((
        constant_expression_parser
            .map(|a| ConstOrRangeExpression::Expr(Box::new(a))),
        cycle_delay_const_range_expression_parser
            .map(|a| ConstOrRangeExpression::Range(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn cycle_delay_const_range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CycleDelayConstRangeExpression<'s>, VerboseError<'s>> {
    let _bounded_parser = (
        constant_expression_parser,
        token(Token::Colon),
        constant_expression_parser,
    )
        .map(|(a, b, c)| {
            CycleDelayConstRangeExpression::Bounded(Box::new((a, b, c)))
        });
    let _unbounded_parser = (
        constant_expression_parser,
        token(Token::Colon),
        token(Token::Dollar),
    )
        .map(|(a, b, c)| {
            CycleDelayConstRangeExpression::Unbounded(Box::new((a, b, c)))
        });
    alt((_bounded_parser, _unbounded_parser)).parse_next(input)
}

pub fn assertion_variable_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssertionVariableDeclaration<'s>, VerboseError<'s>> {
    (
        var_data_type_parser,
        list_of_variable_decl_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c)| AssertionVariableDeclaration(a, b, c))
        .parse_next(input)
}
