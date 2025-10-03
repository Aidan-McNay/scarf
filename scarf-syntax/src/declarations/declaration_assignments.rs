// =======================================================================
// declaration_assignments.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NetDeclAssignment<'a>(
    pub NetIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        Expression<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ParamAssignment<'a>(
    pub ParameterIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Option<(
        Metadata<'a>, // =
        ConstantParamExpression<'a>,
    )>,
);

pub type SpecparamAssignment<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAssignment<'a>(
    pub TypeIdentifier<'a>,
    pub Option<(Metadata<'a>, DataTypeOrIncompleteClassScopedType<'a>)>,
);

pub type VariableDeclAssignment<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum ClassNew<'a> {
    Args(
        Box<(
            Option<ClassScope<'a>>,
            Metadata<'a>, // new
            Option<(
                Metadata<'a>, // (
                ListOfArguments<'a>,
                Metadata<'a>, // )
            )>,
        )>,
    ),
    Expression(
        Box<(
            Metadata<'a>, // new
            Expression<'a>,
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicArrayNew<'a>(
    pub Metadata<'a>, // new
    pub Metadata<'a>, // [
    pub Expression<'a>,
    pub Metadata<'a>, // ]
    pub  Option<(
        Metadata<'a>, // (
        Expression<'a>,
        Metadata<'a>, // )
    )>,
);
