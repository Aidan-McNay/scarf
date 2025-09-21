// =======================================================================
// parallel_and_sequential_blocks.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.3

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionBlock<'a> {
    Basic(Box<StatementOrNull<'a>>),
    Conditional(
        Box<(
            Option<Statement<'a>>,
            Metadata<'a>, // else
            StatementOrNull<'a>,
        )>,
    ),
}
