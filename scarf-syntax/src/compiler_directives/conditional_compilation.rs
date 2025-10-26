// =======================================================================
// conditional_compilation.rs
// =======================================================================
// Syntax for conditional compilation directives

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IfdefDirective<'a>(
    pub Span, // `ifdef
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct IfndefDirective<'a>(
    pub Span, // `ifndef
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ElsifDirective<'a>(
    pub Span, // `elsif
    pub IfdefCondition<'a>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct EndifDirective(
    pub Span, // `endif
);

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefCondition<'a> {
    TextMacro(Box<TextMacroIdentifier<'a>>),
    ParenMacro(
        Box<(
            Span, // (
            IfdefMacroExpression<'a>,
            Span, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IfdefMacroExpression<'a> {
    Text(Box<TextMacroIdentifier<'a>>),
    Operator(
        Box<(
            IfdefMacroExpression<'a>,
            BinaryLogicalOperator,
            IfdefMacroExpression<'a>,
        )>,
    ),
    Not(
        Box<(
            Span, // !
            IfdefMacroExpression<'a>,
        )>,
    ),
    Paren(
        Box<(
            Span, // (
            IfdefMacroExpression<'a>,
            Span, // )
        )>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryLogicalOperator {
    AmpAmp(Span),
    PipePipe(Span),
    Implication(Span),
    Equivalence(Span),
}
