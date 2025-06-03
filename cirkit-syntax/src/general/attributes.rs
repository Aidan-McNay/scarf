// =======================================================================
// attributes.rs
// =======================================================================
// AST Nodes from 1800-2023 A.9.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeInstance<'a>(pub Vec<AttrSpec<'a>>);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrSpec<'a>(pub AttrName<'a>, pub Option<ConstantExpression>);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrName<'a>(pub Identifier<'a>);
