// =======================================================================
// type_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.1.3

pub type PackageImportDeclaration = ();

#[derive(Clone, Debug, PartialEq)]
pub enum Lifetime {
    Static,
    Automatic,
}
