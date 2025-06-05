// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TimeLiteral<'a> {
    TimeLiteralUnsigned(Box<(UnsignedNumber<'a>, TimeUnit<'a>)>),
    TimeLiteralFixedPoint(Box<(FixedPointNumber<'a>, TimeUnit<'a>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit<'a> {
    S(Metadata<'a>),
    MS(Metadata<'a>),
    US(Metadata<'a>),
    NS(Metadata<'a>),
    PS(Metadata<'a>),
    FS(Metadata<'a>),
}

pub type ConstantSelect<'a> = ();
