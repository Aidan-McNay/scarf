// =======================================================================
// expressions.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.3

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantExpression<'a> {
    Primary(Box<ConstantPrimary<'a>>),
    Unary(
        Box<(
            UnaryOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ConstantPrimary<'a>,
        )>,
    ),
    Binary(
        Box<(
            ConstantExpression<'a>,
            BinaryOperator<'a>,
            Vec<AttributeInstance<'a>>,
            ConstantExpression<'a>,
        )>,
    ),
    Ternary(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>, // ?
            Vec<AttributeInstance<'a>>,
            ConstantExpression<'a>,
            Metadata<'a>, // :
            ConstantExpression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantMintypmaxExpression<'a> {
    Single(Box<ConstantExpression<'a>>),
    MinTypMax(
        Box<(
            ConstantExpression<'a>,
            Metadata<'a>,
            ConstantExpression<'a>,
            Metadata<'a>,
            ConstantExpression<'a>,
        )>,
    ),
}

pub type ConstantParamExpression<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantRangeExpression<'a> {
    Expression(Box<ConstantExpression<'a>>),
    PartSelectRange(Box<ConstantPartSelectRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantPartSelectRange<'a> {
    Range(Box<ConstantRange<'a>>),
    IndexedRange(Box<ConstantIndexedRange<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantRange<'a>(
    pub ConstantExpression<'a>,
    pub Metadata<'a>, // :
    pub ConstantExpression<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ConstantIndexedRange<'a> {
    Plus(Box<(ConstantExpression<'a>, Metadata<'a>, ConstantExpression<'a>)>),
    Minus(Box<(ConstantExpression<'a>, Metadata<'a>, ConstantExpression<'a>)>),
}

pub type Expression<'a> = Metadata<'a>;
