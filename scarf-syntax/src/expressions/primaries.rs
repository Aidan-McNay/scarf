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

#[derive(Clone, Debug, PartialEq)]
pub enum ImplicitClassHandle<'a> {
    This(Metadata<'a>),
    Super(Metadata<'a>),
    ThisSuper(Metadata<'a>, Metadata<'a>, Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBitSelect<'a>(
    pub  Vec<(
        Metadata<'a>, // [
        ConstantExpression<'a>,
        Metadata<'a>, // ]
    )>,
);
pub type ConstantSelect<'a> = ();
