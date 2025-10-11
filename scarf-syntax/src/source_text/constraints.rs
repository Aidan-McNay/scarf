// =======================================================================
// constraints.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.10

use crate::*;

pub type ConstraintBlock<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionOrDist<'a>(
    pub Expression<'a>,
    pub  Option<(
        Metadata<'a>, // dist
        Metadata<'a>, // {
        DistList<'a>,
        Metadata<'a>, // }
    )>,
);

pub type DistList<'a> = ();
