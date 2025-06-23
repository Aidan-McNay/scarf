// =======================================================================
// primaries.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.4

use crate::*;

pub type ConstantPrimary<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum PrimaryLiteral<'a> {
    Number(Box<Number<'a>>),
    TimeLiteral(Box<TimeLiteral<'a>>),
    UnbasedUnsizedLiteral(Box<UnbasedUnsizedLiteral<'a>>),
    StringLiteral(Box<StringLiteral<'a>>),
}

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

pub type Select<'a> = ();
pub type NonrangeSelect<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBitSelect<'a>(
    pub  Vec<(
        Metadata<'a>, // [
        ConstantExpression<'a>,
        Metadata<'a>, // ]
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantSelect<'a>(
    pub  Option<(
        Vec<(
            Metadata<'a>, // .
            MemberIdentifier<'a>,
            ConstantBitSelect<'a>,
        )>,
        Metadata<'a>, // .
        MemberIdentifier<'a>,
    )>,
    pub ConstantBitSelect<'a>,
    pub  Option<(
        Metadata<'a>, // [
        ConstantPartSelectRange<'a>,
        Metadata<'a>, // ]
    )>,
);
