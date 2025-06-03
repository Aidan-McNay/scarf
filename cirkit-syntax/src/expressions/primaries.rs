// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeLiteral<'a> {
    TimeLiteralUnsigned(Box<(UnsignedNumber<'a>, TimeUnit)>),
    TimeLiteralFixedPoint(Box<(FixedPointNumber<'a>, TimeUnit)>),
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
