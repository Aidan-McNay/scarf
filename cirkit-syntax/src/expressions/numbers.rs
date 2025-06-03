// =======================================================================
// numbers.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.7

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointNumber<'a>(pub UnsignedNumber<'a>, pub UnsignedNumber<'a>);

pub type UnsignedNumber<'a> = (&'a str, Metadata<'a>);
