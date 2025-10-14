// =======================================================================
// covergroup_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.11

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SelectExpression<'a> {
    Condition(Box<SelectCondition<'a>>),
    Not(
        Box<(
            Metadata<'a>, // !
            SelectCondition<'a>,
        )>,
    ),
    And(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // &&
            SelectExpression<'a>,
        )>,
    ),
    Or(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // ||
            SelectExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Metadata<'a>, // (
            SelectExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
    With(
        Box<(
            SelectExpression<'a>,
            Metadata<'a>, // with
            Metadata<'a>, // (
            WithCovergroupExpression<'a>,
            Metadata<'a>, // )
            Option<(
                Metadata<'a>, // matches
                IntegerCovergroupExpression<'a>,
            )>,
        )>,
    ),
    CrossIdentifier(Box<CrossIdentifier<'a>>),
    CrossSet(
        Box<(
            CrossSetExpression<'a>,
            Option<(
                Metadata<'a>, // matches
                IntegerCovergroupExpression<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectCondition<'a>(
    pub Metadata<'a>, // binsof
    pub Metadata<'a>, // (
    pub BinsExpression<'a>,
    pub Metadata<'a>, // )
    pub  Option<(
        Metadata<'a>, // intersect
        Metadata<'a>, // {
        CovergroupRangeList<'a>,
        Metadata<'a>, // }
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum BinsExpression<'a> {
    Variable(Box<VariableIdentifier<'a>>),
    CoverPoint(
        Box<(
            CoverPointIdentifier<'a>,
            Option<(
                Metadata<'a>, // .
                BinIdentifier<'a>,
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupRangeList<'a>(
    pub CovergroupValueRange<'a>,
    pub  Vec<(
        Metadata<'a>, // ,
        CovergroupValueRange<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum CovergroupValueRange<'a> {
    Expr(Box<CovergroupExpression<'a>>),
    ExprRange(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // :
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarLow(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // $
            Metadata<'a>, // :
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    DollarHigh(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
            Metadata<'a>, // ]
        )>,
    ),
    AbsoluteTolerance(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // +/-
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    RelativeTolerance(
        Box<(
            Metadata<'a>, // [
            CovergroupExpression<'a>,
            Metadata<'a>, // +%-
            CovergroupExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithCovergroupExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct SetCovergroupExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerCovergroupExpression<'a> {
    Expression(Box<CovergroupExpression<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CrossSetExpression<'a>(pub CovergroupExpression<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct CovergroupExpression<'a>(pub Expression<'a>);
