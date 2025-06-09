// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.2.1

use crate::*;

pub type DataType<'a> = ();
pub type DataTypeOrImplicit<'a> = ();
pub type ClassType = ();
pub type InterfaceClassType = ();

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
pub enum SoftOrTagged<'a> {
    Soft(Metadata<'a>),
    Tagged(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StructUnion<'a> {
    Struct(Metadata<'a>),
    Union(Metadata<'a>, Option<SoftOrTagged<'a>>),
}

pub type DataTypeOrIncompleteClassScopedType<'a> = ();
