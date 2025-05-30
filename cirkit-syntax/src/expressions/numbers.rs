// =======================================================================
// numbers.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.7

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointNumber(pub UnsignedNumber, pub UnsignedNumber);

pub type UnsignedNumber = String;
