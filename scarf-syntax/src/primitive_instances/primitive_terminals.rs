// =======================================================================
// primitive_terminals.rs
// =======================================================================
// AST Nodes from 1800-2023 A.3.3

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnableTerminal(pub Expression);

#[derive(Clone, Debug, PartialEq)]
pub struct InoutTerminal<'a>(pub NetLvalue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InputTerminal(pub Expression);

#[derive(Clone, Debug, PartialEq)]
pub struct NcontrolTerminal(pub Expression);

#[derive(Clone, Debug, PartialEq)]
pub struct OutputTerminal<'a>(pub NetLvalue<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct PcontrolTerminal(pub Expression);
