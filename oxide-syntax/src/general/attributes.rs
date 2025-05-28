// =======================================================================
// attributes.rs
// =======================================================================
// AST Nodes from 1800-2023 A.9.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeInstance(Vec<AttrSpec>);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrSpec(AttrName, Option<ConstantExpression>);

#[derive(Clone, Debug, PartialEq)]
pub struct AttrName(Identifier);
