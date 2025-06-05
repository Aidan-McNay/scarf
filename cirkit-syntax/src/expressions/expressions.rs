// =======================================================================
// expressions.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.3

use crate::*;

pub type ConstantExpression<'a> = ();
pub type ConstantParamExpression<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantRange<'a>(
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // :
    pub ConstantExpression<'a>,
);

pub type Expression<'a> = ();
