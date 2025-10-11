// =======================================================================
// assertion_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.10

use crate::*;

pub type ConcurrentAssertionItem<'a> = ();
pub type ConcurrentAssertionStatement<'a> = ();

pub type AssertionItemDeclaration<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceExpr<'a> {
    StartDelay(
        Box<(
            CycleDelayRange<'a>,
            SequenceExpr<'a>,
            Vec<(CycleDelayRange<'a>, SequenceExpr<'a>)>,
        )>,
    ),
    Delay(
        Box<(
            SequenceExpr<'a>,
            CycleDelayRange<'a>,
            SequenceExpr<'a>,
            Vec<(CycleDelayRange<'a>, SequenceExpr<'a>)>,
        )>,
    ),
    Expr(Box<(ExpressionOrDist<'a>, Option<BooleanAbbrev<'a>>)>),
    Inst(Box<(SequenceInstance<'a>, Option<SequenceAbbrev<'a>>)>),
    Paren(
        Box<(
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Vec<(
                Metadata<'a>, // ,
                SequenceMatchItem<'a>,
            )>,
            Metadata<'a>, // )
            Option<SequenceAbbrev<'a>>,
        )>,
    ),
    And(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // and
            SequenceExpr<'a>,
        )>,
    ),
    Intersect(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // intersect
            SequenceExpr<'a>,
        )>,
    ),
    Or(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // or
            SequenceExpr<'a>,
        )>,
    ),
    FirstMatch(
        Box<(
            Metadata<'a>, // first_match
            Metadata<'a>, // (
            SequenceExpr<'a>,
            Vec<(
                Metadata<'a>, // ,
                SequenceMatchItem<'a>,
            )>,
            Metadata<'a>, // )
        )>,
    ),
    Throughout(
        Box<(
            ExpressionOrDist<'a>,
            Metadata<'a>, // throughout
            SequenceExpr<'a>,
        )>,
    ),
    Within(
        Box<(
            SequenceExpr<'a>,
            Metadata<'a>, // within
            SequenceExpr<'a>,
        )>,
    ),
    Clocking(Box<(ClockingEvent<'a>, SequenceExpr<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelayRange<'a> {
    Primary(
        Box<(
            Metadata<'a>, // ##
            ConstantPrimary<'a>,
        )>,
    ),
    Range(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            CycleDelayConstRangeExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    Star(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            Metadata<'a>, // *
            Metadata<'a>, // ]
        )>,
    ),
    Plus(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // [
            Metadata<'a>, // +
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceMethodCall<'a>(
    pub SequenceInstance<'a>,
    pub Metadata<'a>, // .
    pub MethodIdentifier<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceMatchItem<'a> {
    Operator(Box<OperatorAssignment<'a>>),
    IncOrDec(Box<IncOrDecExpression<'a>>),
    Subroutine(Box<SubroutineCall<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceInstance<'a>(
    pub PsOrHierarchicalSequenceIdentifier<'a>,
    pub  Option<(
        Metadata<'a>, // (
        Option<SequenceListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceListOfArguments<'a> {
    PartialIdentifier(
        Box<(
            Option<SequenceActualArg<'a>>,
            Vec<(
                Metadata<'a>, // ,
                Option<SequenceActualArg<'a>>,
            )>,
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<SequenceActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Identifier(
        Box<(
            Metadata<'a>, // .
            Identifier<'a>,
            Metadata<'a>, // (
            Option<SequenceActualArg<'a>>,
            Metadata<'a>, // )
            Vec<(
                Metadata<'a>, // ,
                Metadata<'a>, // .
                Identifier<'a>,
                Metadata<'a>, // (
                Option<SequenceActualArg<'a>>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SequenceActualArg<'a> {
    Event(Box<EventExpression<'a>>),
    Sequence(Box<SequenceExpr<'a>>),
    Dollar(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BooleanAbbrev<'a> {
    Consecutive(Box<ConsecutiveRepetition<'a>>),
    Nonconsecutive(Box<NonconsecutiveRepetition<'a>>),
    Goto(Box<GotoRepetition<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceAbbrev<'a>(pub ConsecutiveRepetition<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum ConsecutiveRepetition<'a> {
    Expr(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // *
            ConstOrRangeExpression<'a>,
            Metadata<'a>, // ]
        )>,
    ),
    Star(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // *
            Metadata<'a>, // ]
        )>,
    ),
    Plus(
        Box<(
            Metadata<'a>, // [
            Metadata<'a>, // +
            Metadata<'a>, // ]
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonconsecutiveRepetition<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, // =
    pub ConstOrRangeExpression<'a>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub struct GotoRepetition<'a>(
    pub Metadata<'a>, // [
    pub Metadata<'a>, //  ->
    pub ConstOrRangeExpression<'a>,
    pub Metadata<'a>, // ]
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstOrRangeExpression<'a> {
    Expr(Box<ConstantExpression<'a>>),
    Range(Box<CycleDelayConstRangeExpression<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelayConstRangeExpression<'a> {
    Bounded(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // :
            ConstantExpression<'a>,
        )>,
    ),
    Unbounded(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // :
            Metadata<'a>, // $
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertionVariableDeclaration<'a>(
    pub VarDataType<'a>,
    pub ListOfVariableDeclAssignments<'a>,
    pub Metadata<'a>, // ;
);
