// =======================================================================
// specify_path_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.7.2

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIdentifier<'a> {
    Posedge(Metadata<'a>),
    Negedge(Metadata<'a>),
    Edge(Metadata<'a>),
}
