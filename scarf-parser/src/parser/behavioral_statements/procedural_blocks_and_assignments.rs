// =======================================================================
// procedural_blocks_and_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.6.2

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn operator_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<OperatorAssignment<'s>, VerboseError<'s>> {
    (
        variable_lvalue_parser,
        assignment_operator_parser,
        expression_parser,
    )
        .map(|(a, b, c)| OperatorAssignment(a, b, c))
        .parse_next(input)
}

pub fn assignment_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::Eq).map(|a| AssignmentOperator::Eq(a)),
        token(Token::PlusEq).map(|a| AssignmentOperator::PlusEq(a)),
        token(Token::MinusEq).map(|a| AssignmentOperator::MinusEq(a)),
        token(Token::StarEq).map(|a| AssignmentOperator::StarEq(a)),
        token(Token::SlashEq).map(|a| AssignmentOperator::SlashEq(a)),
        token(Token::PercentEq).map(|a| AssignmentOperator::PercentEq(a)),
        token(Token::AmpEq).map(|a| AssignmentOperator::AmpEq(a)),
        token(Token::PipeEq).map(|a| AssignmentOperator::PipeEq(a)),
        token(Token::CaretEq).map(|a| AssignmentOperator::CaretEq(a)),
        token(Token::LtLtEq).map(|a| AssignmentOperator::LtLtEq(a)),
        token(Token::GtGtEq).map(|a| AssignmentOperator::GtGtEq(a)),
        token(Token::LtLtLtEq).map(|a| AssignmentOperator::LtLtLtEq(a)),
        token(Token::GtGtGtEq).map(|a| AssignmentOperator::GtGtGtEq(a)),
    ))
    .parse_next(input)
}
