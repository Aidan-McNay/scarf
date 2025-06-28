// =======================================================================
// procedural_blocks_and_assignments.rs
// =======================================================================
// Parsing for 1800-2023 A.6.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn operator_assignment_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, OperatorAssignment<'a>, ParserError<'a>> + Clone {
    variable_lvalue_parser(expression_parser.clone())
        .then(assignment_operator_parser())
        .then(expression_parser)
        .map(|((a, b), c)| OperatorAssignment(a, b, c))
        .boxed()
}

pub fn assignment_operator_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, AssignmentOperator<'a>, ParserError<'a>> + Clone {
    choice((
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
    .boxed()
}
