// =======================================================================
// conditional_compilation.rs
// =======================================================================
// Syntax for conditional compilation directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IfdefDirective<'a>(
    pub Span<'a>, // `ifdef
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct IfndefDirective<'a>(
    pub Span<'a>, // `ifndef
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ElsifDirective<'a>(
    pub Span<'a>, // `elsif
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct EndifDirective<'a>(
    pub Span<'a>, // `endif
);

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefCondition<'a> {
    TextMacro(Box<TextMacroIdentifier<'a>>),
    ParenMacro(
        Box<(
            Span<'a>, // (
            IfdefMacroExpression<'a>,
            Span<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefMacroExpression<'a> {
    Text(Box<TextMacroIdentifier<'a>>),
    Operator(
        Box<(
            IfdefMacroExpression<'a>,
            BinaryLogicalOperator<'a>,
            IfdefMacroExpression<'a>,
        )>,
    ),
    Not(
        Box<(
            Span<'a>, // !
            IfdefMacroExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Span<'a>, // (
            IfdefMacroExpression<'a>,
            Span<'a>, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryLogicalOperator<'a> {
    AmpAmp(Span<'a>),
    PipePipe(Span<'a>),
    Implication(Span<'a>),
    Equivalence(Span<'a>),
}
