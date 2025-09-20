// =======================================================================
// procedural_blocks_and_assignments.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.2

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OperatorAssignment<'a>(
    pub VariableLvalue<'a>,
    pub AssignmentOperator<'a>,
    pub Expression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum AssignmentOperator<'a> {
    Eq(Metadata<'a>),
    PlusEq(Metadata<'a>),
    MinusEq(Metadata<'a>),
    StarEq(Metadata<'a>),
    SlashEq(Metadata<'a>),
    PercentEq(Metadata<'a>),
    AmpEq(Metadata<'a>),
    PipeEq(Metadata<'a>),
    CaretEq(Metadata<'a>),
    LtLtEq(Metadata<'a>),
    GtGtEq(Metadata<'a>),
    LtLtLtEq(Metadata<'a>),
    GtGtGtEq(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableAssignment<'a>(
    pub VariableLvalue<'a>,
    pub Metadata<'a>, // =
    pub Expression<'a>,
);
