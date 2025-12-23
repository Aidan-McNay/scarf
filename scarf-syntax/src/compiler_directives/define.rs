// =======================================================================
// define.rs
// =======================================================================
// Syntax for preprocessor definitions

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroDefinition<'a>(
    pub Span<'a>, // `define
    pub TextMacroName<'a>,
    pub Option<MacroText<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroName<'a>(
    pub TextMacroIdentifier<'a>,
    pub  Option<(
        Span<'a>, // (
        ListOfFormalArguments<'a>,
        Span<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfFormalArguments<'a>(
    pub FormalArgument<'a>,
    pub  Vec<(
        Span<'a>, // ,
        FormalArgument<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct FormalArgument<'a>(
    pub (&'a str, Span<'a>), // identifier
    pub  Option<(
        Span<'a>, // =
        Span<'a>, // default_text
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroIdentifier<'a>(pub &'a str, pub Span<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct MacroText<'a>(pub Span<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroUsage<'a>(
    pub (&'a str, Span<'a>), // `text_macro_identifier
    pub  Option<(
        Span<'a>, // (
        ListOfActualArguments<'a>,
        Span<'a>, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfActualArguments<'a>(
    pub ActualArgument<'a>,
    pub  Vec<(
        Span<'a>, // ,
        ActualArgument<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ActualArgument<'a> {
    Expr(Box<Expression<'a>>),
    Empty(),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UndefineCompilerDirective<'a>(
    pub Span<'a>, // `undef
    pub TextMacroIdentifier<'a>,
);
