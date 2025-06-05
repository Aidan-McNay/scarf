// =======================================================================
// declaration_assignments.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ParamAssignment<'a>(
    pub ParameterIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub Option<(Metadata<'a>, ConstantParamExpression<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAssignment<'a>(
    pub TypeIdentifier<'a>,
    pub Option<(Metadata<'a>, DataTypeOrIncompleteClassScopedType<'a>)>,
);
