// =======================================================================
// module_items.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SeveritySystemTask<'a> {
    Fatal(Box<FatalSeveritySystemTask<'a>>),
    Error(Box<ErrorSeveritySystemTask<'a>>),
    Warning(Box<WarningSeveritySystemTask<'a>>),
    Info(Box<InfoSeveritySystemTask<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FatalSeveritySystemTask<'a>(
    pub Metadata<'a>, // $fatal
    pub  Option<(
        Metadata<'a>, // (
        FinishNumber<'a>,
        Option<(
            Metadata<'a>, // ,
            ListOfArguments<'a>,
        )>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorSeveritySystemTask<'a>(
    pub Metadata<'a>, // $error
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct WarningSeveritySystemTask<'a>(
    pub Metadata<'a>, // $warning
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct InfoSeveritySystemTask<'a>(
    pub Metadata<'a>, // $info
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub enum FinishNumber<'a> {
    Zero(Metadata<'a>),
    One(Metadata<'a>),
    Two(Metadata<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElaborationSeveritySystemTask<'a>(pub SeveritySystemTask<'a>);

pub type BindDirective = ();
pub type ModuleItem = ();
pub type NonPortModuleItem = ();
