// =======================================================================
// statements.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementOrNull<'a> {
    Statement(Box<Statement<'a>>),
    Null(
        Box<(
            Vec<AttributeInstance<'a>>,
            Metadata<'a>, // ;
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'a>(
    pub  Option<(
        BlockIdentifier<'a>,
        Metadata<'a>, // :
    )>,
    pub Vec<AttributeInstance<'a>>,
    pub StatementItem<'a>,
);

pub type StatementItem<'a> = ();
pub type FunctionStatement<'a> = ();
