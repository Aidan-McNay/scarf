// =======================================================================
// conditional_statements.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.6

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CondPredicate<'a>(
    pub ExpressionOrCondPattern<'a>,
    pub Vec<(Metadata<'a>, ExpressionOrCondPattern<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionOrCondPattern<'a> {
    Expression(Box<Expression<'a>>),
    CondPattern(Box<CondPattern<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CondPattern<'a>(pub Expression<'a>, pub Metadata<'a>, pub Pattern<'a>);
