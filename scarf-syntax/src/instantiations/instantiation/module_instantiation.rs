// =======================================================================
// module_instantiation.rs
// =======================================================================
// AST Nodes from 1800-2023 A.4.1.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NameOfInstance<'a>(
    pub InstanceIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
);
