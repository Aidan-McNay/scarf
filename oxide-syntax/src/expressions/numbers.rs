// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.7

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointNumber(UnsignedNumber, UnsignedNumber);

pub type UnsignedNumber = ();
