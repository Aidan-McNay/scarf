// =======================================================================
// task_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.7

use crate::*;

pub type TfPortList<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct FinalSpecifier<'a>(
    pub Metadata<'a>, // :
    pub Metadata<'a>, // final
);
