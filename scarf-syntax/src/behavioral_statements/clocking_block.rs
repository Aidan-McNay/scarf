// =======================================================================
// clocking_block.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.11

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CycleDelay<'a> {
    Integral(
        Box<(
            Metadata<'a>, // ##
            IntegralNumber<'a>,
        )>,
    ),
    Identifier(
        Box<(
            Metadata<'a>, // ##
            Identifier<'a>,
        )>,
    ),
    Expression(
        Box<(
            Metadata<'a>, // ##
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
        )>,
    ),
}
