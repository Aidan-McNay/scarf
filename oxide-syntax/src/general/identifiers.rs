// =======================================================================
// identifiers.rs
// =======================================================================
// AST Nodes from 1800-2023 A.9.3

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerIdentifier(Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassIdentifier(Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleIdentifier(Identifier);

pub type Identifier = ();

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceIdentifier(Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageIdentifier(Identifier);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramIdentifier(Identifier);
