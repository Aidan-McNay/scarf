// =======================================================================
// identifiers.rs
// =======================================================================
// AST Nodes from 1800-2023 A.9.3

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassIdentifier<'a>(pub Identifier<'a>);

pub type EscapedIdentifier<'a> = &'a str;

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum Identifier<'a> {
    SimpleIdentifier(SimpleIdentifier<'a>),
    EscapedIdentifier(EscapedIdentifier<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageIdentifier<'a>(pub Identifier<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramIdentifier<'a>(pub Identifier<'a>);

pub type SimpleIdentifier<'a> = &'a str;
