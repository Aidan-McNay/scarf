// =======================================================================
// identifiers.rs
// =======================================================================
// AST Nodes from 1800-2023 A.9.3

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerIdentifier(pub Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassIdentifier(pub Identifier);

pub type EscapedIdentifier = String;

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleIdentifier(pub Identifier);

#[derive(Clone, Debug, PartialEq)]
pub enum Identifier {
    SimpleIdentifier(Box<SimpleIdentifier>),
    EscapedIdentifier(Box<EscapedIdentifier>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceIdentifier(pub Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageIdentifier(pub Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramIdentifier(pub Identifier);

pub type SimpleIdentifier = String;
