// =======================================================================
// case_statements.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.7

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RangeList<'a>(pub ValueRange<'a>, pub Vec<(Metadata<'a>, ValueRange<'a>)>);

#[derive(Clone, Debug, PartialEq)]
pub enum ValueRange<'a> {
    Expression(Box<Expression<'a>>),
    Slice(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // :
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarLow(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // $
            Metadata<'a>, // :
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarHigh(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
            Metadata<'a>, // ]
        )>,
    ),
    AbsoluteTolerance(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // +/-
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    RelativeTolerance(
        Box<(
            Metadata<'a>, // [
            Expression<'a>,
            Metadata<'a>, // +%-
            Expression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}
