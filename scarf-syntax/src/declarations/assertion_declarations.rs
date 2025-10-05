// =======================================================================
// assertion_declarations.rs
// =======================================================================
// AST Nodes from 1800-2023 A.2.10

use crate::*;

pub type ConcurrentAssertionItem<'a> = ();
pub type ConcurrentAssertionStatement<'a> = ();

pub type SequenceMethodCall<'a> = Metadata<'a>;
pub type SequenceInstance<'a> = ();
