// =======================================================================
// patterns.rs
// =======================================================================
// AST Nodes from 1800-2023 A.6.7.1

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<'a> {
    Parentheses(Box<(Metadata<'a>, Pattern<'a>, Metadata<'a>)>),
    VariableIdentifier(Box<(Metadata<'a>, VariableIdentifier<'a>)>),
    Wildcard(Box<(Metadata<'a>, Metadata<'a>)>),
    ConstantExpression(Box<ConstantExpression<'a>>),
    TaggedMember(Box<(Metadata<'a>, MemberIdentifier<'a>, Option<Pattern<'a>>)>),
    MultiPattern(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            Pattern<'a>,
            Vec<(
                Metadata<'a>, // ,
                Pattern<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
    MultiIdentifierPattern(
        Box<(
            Metadata<'a>, // '
            Metadata<'a>, // {
            MemberIdentifier<'a>,
            Metadata<'a>, // :
            Pattern<'a>,
            Vec<(
                Metadata<'a>, // ,
                MemberIdentifier<'a>,
                Metadata<'a>, // :
                Pattern<'a>,
            )>,
            Metadata<'a>, // }
        )>,
    ),
}

pub type AssignmentPatternExpression<'a> = ();
pub type AssignmentPatternExpressionType<'a> = ();

pub type ConstantAssignmentPatternExpression<'a> = ();

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternNetLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub NetLvalue<'a>,
    pub Vec<(Metadata<'a>, NetLvalue<'a>)>,
    pub Metadata<'a>, // }
);

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentPatternVariableLvalue<'a>(
    pub Metadata<'a>, // '
    pub Metadata<'a>, // {
    pub VariableLvalue<'a>,
    pub Vec<(Metadata<'a>, VariableLvalue<'a>)>,
    pub Metadata<'a>, // }
);
