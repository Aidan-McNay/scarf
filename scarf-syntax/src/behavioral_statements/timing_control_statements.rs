// =======================================================================
// timing_control_statements.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.5

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum DelayControl<'a> {
    Value(
        Box<(
            Metadata<'a>, // #
            DelayValue<'a>,
        )>,
    ),
    Mintypmax(
        Box<(
            Metadata<'a>, // #
            Metadata<'a>, // (
            MintypmaxExpression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}
