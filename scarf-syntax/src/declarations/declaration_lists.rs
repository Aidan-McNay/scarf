// =======================================================================
// declaration_lists.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.3

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfInterfaceIdentifiers<'a>(
    pub InterfaceIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
    pub  Vec<(
        Metadata<'a>, // ,
        InterfaceIdentifier<'a>,
        Vec<UnpackedDimension<'a>>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfParamAssignments<'a>(
    pub ParamAssignment<'a>,
    pub Vec<(Metadata<'a>, ParamAssignment<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfPortIdentifiers<'a>(
    pub PortIdentifier<'a>,
    pub Vec<UnpackedDimension<'a>>,
    pub  Vec<(
        Metadata<'a>, // ,
        PortIdentifier<'a>,
        Vec<UnpackedDimension<'a>>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfSpecparamAssignments<'a>(
    pub SpecparamAssignment<'a>,
    pub Vec<(Metadata<'a>, SpecparamAssignment<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfTypeAssignments<'a>(
    pub TypeAssignment<'a>,
    pub Vec<(Metadata<'a>, TypeAssignment<'a>)>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfVariableIdentifiers<'a>(
    pub VariableIdentifier<'a>,
    pub Vec<VariableDimension<'a>>,
    pub  Vec<(
        Metadata<'a>, // ,
        VariableIdentifier<'a>,
        Vec<VariableDimension<'a>>,
    )>,
);
