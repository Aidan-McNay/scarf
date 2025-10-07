// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.2.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CastingType<'a> {
    SimpleType(Box<SimpleType<'a>>),
    ConstantPrimary(Box<ConstantPrimary<'a>>),
    Signing(Box<Signing<'a>>),
    String(Box<Metadata<'a>>),
    Const(Box<Metadata<'a>>),
}

pub type DataType<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeOrImplicit<'a> {
    DataType(DataType<'a>),
    ImplicitDataType(ImplicitDataType<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplicitDataType<'a>(
    pub Option<Signing<'a>>,
    pub Vec<PackedDimension<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassScope<'a>(
    pub ClassType<'a>,
    pub Metadata<'a>, // ::
);

pub type ClassType<'a> = ();
pub type InterfaceClassType = ();

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerType<'a> {
    Vector(Box<IntegerVectorType<'a>>),
    Atom(Box<IntegerAtomType<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerAtomType<'a> {
    Byte(Metadata<'a>),
    Shortint(Metadata<'a>),
    Int(Metadata<'a>),
    Longint(Metadata<'a>),
    Integer(Metadata<'a>),
    Time(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerVectorType<'a> {
    Bit(Metadata<'a>),
    Logic(Metadata<'a>),
    Reg(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonIntegerType<'a> {
    Shortreal(Metadata<'a>),
    Real(Metadata<'a>),
    Realtime(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NetType<'a> {
    Supply0(Metadata<'a>),
    Supply1(Metadata<'a>),
    Tri(Metadata<'a>),
    Triand(Metadata<'a>),
    Trior(Metadata<'a>),
    Trireg(Metadata<'a>),
    Tri0(Metadata<'a>),
    Tri1(Metadata<'a>),
    Uwire(Metadata<'a>),
    Wire(Metadata<'a>),
    Wand(Metadata<'a>),
    Wor(Metadata<'a>),
}

pub type NetPortType<'a> = ();
pub type VariablePortType<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub enum Signing<'a> {
    Signed(Metadata<'a>),
    Unsigned(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SimpleType<'a> {
    Integer(Box<IntegerType<'a>>),
    NonInteger(Box<NonIntegerType<'a>>),
    PsType(Box<PsTypeIdentifier<'a>>),
    PsParameter(Box<PsParameterIdentifier<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SoftOrTagged<'a> {
    Soft(Metadata<'a>),
    Tagged(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructUnion<'a> {
    Struct(Metadata<'a>),
    Union(Metadata<'a>, Option<SoftOrTagged<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataTypeOrVoid<'a> {
    DataType(Box<DataType<'a>>),
    Void(Box<Metadata<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeReference<'a> {
    Expression(
        Box<(
            Metadata<'a>, // type
            Metadata<'a>, // (
            Expression<'a>,
            Metadata<'a>, // )
        )>,
    ),
    DataTypeOrIncompleteClassScopedType(
        Box<(
            Metadata<'a>, // type
            Metadata<'a>, // (
            DataTypeOrIncompleteClassScopedType<'a>,
            Metadata<'a>, // )
        )>,
    ),
}

pub type DataTypeOrIncompleteClassScopedType<'a> = ();
