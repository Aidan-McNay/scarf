// =======================================================================
// module_items.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.4

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SystemSeverityTask<'a> {
    Fatal(Box<FatalSystemSeverityTask<'a>>),
    Error(Box<ErrorSystemSeverityTask<'a>>),
    Warning(Box<WarningSystemSeverityTask<'a>>),
    Info(Box<InfoSystemSeverityTask<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FatalSystemSeverityTask<'a>(
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
pub struct ErrorSystemSeverityTask<'a>(
    pub Metadata<'a>, // $error
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct WarningSystemSeverityTask<'a>(
    pub Metadata<'a>, // $warning
    pub  Option<(
        Metadata<'a>, // (
        Option<ListOfArguments<'a>>,
        Metadata<'a>, // )
    )>,
    pub Metadata<'a>, // ;
);

#[derive(Clone, Debug, PartialEq)]
pub struct InfoSystemSeverityTask<'a>(
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
pub struct ElaborationSystemSeverityTask<'a>(pub SystemSeverityTask<'a>);

pub type BindDirective = ();
pub type ModuleItem = ();
pub type NonPortModuleItem = ();
