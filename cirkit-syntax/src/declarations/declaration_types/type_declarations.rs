// =======================================================================
// type_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.1.3

use crate::*;

pub type PackageImportDeclaration = ();

#[derive(Clone, Debug, PartialEq)]
pub enum ForwardType<'a> {
    Enum(Metadata<'a>),
    Struct(Metadata<'a>),
    Union(Metadata<'a>),
    Class(Metadata<'a>),
    InterfaceClass(Metadata<'a>, Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lifetime<'a> {
    Static(Metadata<'a>),
    Automatic(Metadata<'a>),
}
