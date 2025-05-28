// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeLiteral {
    TimeLiteralUnsigned(Box<TimeLiteralUnsigned>),
    TimeLiteralFixedPoint(Box<TimeLiteralFixedPoint>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeLiteralUnsigned(UnsignedNumber, TimeUnit);

#[derive(Clone, Debug, PartialEq)]
pub struct TimeLiteralFixedPoint(FixedPointNumber, TimeUnit);

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
    S,
    MS,
    US,
    NS,
    PS,
    FS,
}
