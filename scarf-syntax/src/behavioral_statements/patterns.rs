// =======================================================================
// patterns.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.7.1

use crate::*;

pub type AssignmentPatternExpressionType<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternNetLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub NetLvalue<'a>,
    pub Vec<(Metadata<'a>, NetLvalue<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternVariableLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub VariableLvalue<'a>,
    pub Vec<(Metadata<'a>, VariableLvalue<'a>)>,
    pub Metadata<'a>, // }
);
