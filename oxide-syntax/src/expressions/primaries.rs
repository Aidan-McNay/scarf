// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeLiteral {
    TimeLiteralUnsigned(Box<(UnsignedNumber, TimeUnit)>),
    TimeLiteralFixedPoint(Box<(FixedPointNumber, TimeUnit)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
    S,
    MS,
    US,
    NS,
    PS,
    FS,
}
