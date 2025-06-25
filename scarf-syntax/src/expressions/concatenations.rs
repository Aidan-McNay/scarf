// =======================================================================
// concatenations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.8.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Concatenation<'a>(
    pub Metadata<'a>, // {
    pub Expression<'a>,
    pub Vec<(Metadata<'a>, Expression<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ConstantExpression<'a>,
    pub Vec<(Metadata<'a>, ConstantExpression<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantMultipleConcatenation<'a>(
    pub Metadata<'a>, // {
    pub ConstantExpression<'a>,
    pub ConstantConcatenation<'a>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct MultipleConcatenation<'a>(
    pub Metadata<'a>, // {
    pub Expression<'a>,
    pub Concatenation<'a>,
    pub Metadata<'a>, // }
);

pub type StreamingConcatenation<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum StreamOperator<'a> {
    Right(Metadata<'a>),
    Left(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyUnpackedArrayConcatenation<'a>(
    pub Metadata<'a>, // {
    pub Metadata<'a>, // }
);
