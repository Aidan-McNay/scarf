// =======================================================================
// text_macro.rs
// =======================================================================
// Syntax for text macros

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroDefinition<'a>(
    pub Span, // `define
    pub TextMacroName<'a>,
    pub MacroText,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroName<'a>(
    pub TextMacroIdentifier<'a>,
    pub  Option<(
        Span, // (
        ListOfFormalArguments<'a>,
        Span, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfFormalArguments<'a>(
    pub FormalArgument<'a>,
    pub  Vec<(
        Span, // ,
        FormalArgument<'a>,
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct FormalArgument<'a>(
    pub (&'a str, Span), // identifier
    pub  Option<(
        Span, // =
        Span, // default_text
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroIdentifier<'a>(pub (&'a str, Span));

#[derive(Clone, Debug, PartialEq)]
pub struct MacroText(pub Span);

#[derive(Clone, Debug, PartialEq)]
pub struct TextMacroUsage<'a>(
    pub (&'a str, Span), // `text_macro_identifier
    pub  Option<(
        Span, // (
        ListOfActualArguments<'a>,
        Span, // )
    )>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ListOfActualArguments<'a>(
    pub ActualArgument<'a>,
    pub  Vec<(
        Span, // ,
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
    pub Span, // `undef
    pub TextMacroIdentifier<'a>,
);
