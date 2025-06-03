// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.2

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceText<'a>(
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<Description<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum Description<'a> {
    ModuleDeclaration(Box<ModuleDeclaration<'a>>),
    UdpDeclaration(Box<UdpDeclaration>),
    InterfaceDeclaration(Box<InterfaceDeclaration<'a>>),
    ProgramDeclaration(Box<ProgramDeclaration<'a>>),
    PackageDeclaration(Box<PackageDeclaration<'a>>),
    DescriptionPackageItem(Box<DescriptionPackageItem<'a>>),
    DescriptionBindDirective(Box<DescriptionBindDirective<'a>>),
    ConfigDeclaration(Box<ConfigDeclaration>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionPackageItem<'a>(pub Vec<AttributeInstance<'a>>, pub PackageItem);

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionBindDirective<'a>(pub Vec<AttributeInstance<'a>>, pub BindDirective);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleDeclaration<'a> {
    ModuleDeclarationNonansi(Box<ModuleDeclarationNonansi<'a>>),
    ModuleDeclarationAnsi(Box<ModuleDeclarationAnsi<'a>>),
    ModuleDeclarationWildcard(Box<ModuleDeclarationWildcard<'a>>),
    ModuleDeclarationExternNonansi(Box<ModuleDeclarationExternNonansi<'a>>),
    ModuleDeclarationExternAnsi(Box<ModuleDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationNonansi<'a>(
    pub ModuleNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ModuleItem>,
    pub Option<ModuleIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationAnsi<'a>(
    pub ModuleAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortModuleItem>,
    pub Option<ModuleIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ModuleItem>,
    pub Option<ModuleIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternNonansi<'a>(pub ModuleNonansiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternAnsi<'a>(pub ModuleAnsiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKeyword {
    Module,
    Macromodule,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceDeclaration<'a> {
    InterfaceDeclarationNonansi(Box<InterfaceDeclarationNonansi<'a>>),
    InterfaceDeclarationAnsi(Box<InterfaceDeclarationAnsi<'a>>),
    InterfaceDeclarationWildcard(Box<InterfaceDeclarationWildcard<'a>>),
    InterfaceDeclarationExternNonansi(Box<InterfaceDeclarationExternNonansi<'a>>),
    InterfaceDeclarationExternAnsi(Box<InterfaceDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationNonansi<'a>(
    pub InterfaceNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<InterfaceItem>,
    pub Option<InterfaceIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationAnsi<'a>(
    pub InterfaceAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortInterfaceItem>,
    pub Option<InterfaceIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub InterfaceIdentifier<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<InterfaceItem>,
    pub Option<InterfaceIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternNonansi<'a>(pub InterfaceNonansiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternAnsi<'a>(pub InterfaceAnsiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Lifetime>,
    pub InterfaceIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Lifetime>,
    pub InterfaceIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramDeclaration<'a> {
    ProgramDeclarationNonansi(Box<ProgramDeclarationNonansi<'a>>),
    ProgramDeclarationAnsi(Box<ProgramDeclarationAnsi<'a>>),
    ProgramDeclarationWildcard(Box<ProgramDeclarationWildcard<'a>>),
    ProgramDeclarationExternNonansi(Box<ProgramDeclarationExternNonansi<'a>>),
    ProgramDeclarationExternAnsi(Box<ProgramDeclarationExternAnsi<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationNonansi<'a>(
    pub ProgramNonansiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ProgramItem>,
    pub Option<ProgramIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationAnsi<'a>(
    pub ProgramAnsiHeader<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<NonPortProgramItem>,
    pub Option<ProgramIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationWildcard<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub ProgramIdentifier<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<ProgramItem>,
    pub Option<ProgramIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternNonansi<'a>(pub ProgramNonansiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternAnsi<'a>(pub ProgramAnsiHeader<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNonansiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Lifetime>,
    pub ProgramIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramAnsiHeader<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Lifetime>,
    pub ProgramIdentifier<'a>,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclaration<'a>(
    pub CheckerIdentifier<'a>,
    pub Option<CheckerPortList>,
    pub Vec<(Vec<AttributeInstance<'a>>, CheckerOrGenerateItem)>,
    pub Option<CheckerIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDeclaration<'a>(
    pub Option<Virtual>,
    pub Option<FinalSpecifier>,
    pub ClassIdentifier<'a>,
    pub Option<ParameterPortList>,
    pub Option<(ClassType, Option<ClassDeclarationExtensionArguments>)>,
    pub Option<Vec<InterfaceClassType>>,
    pub Vec<ClassItem>,
    pub Option<ClassIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ClassDeclarationExtensionArguments {
    ListOfArguments(Box<ListOfArguments>),
    Default,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassDeclaration<'a>(
    pub ClassIdentifier<'a>,
    pub Option<ParameterPortList>,
    pub Option<Vec<InterfaceClassType>>,
    pub Vec<InterfaceClassItem>,
    pub Option<ClassIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageDeclaration<'a>(
    pub Vec<AttributeInstance<'a>>,
    pub Option<Lifetime>,
    pub PackageIdentifier<'a>,
    pub Option<TimeunitsDeclaration<'a>>,
    pub Vec<(Vec<AttributeInstance<'a>>, PackageItem)>,
    pub Option<PackageIdentifier<'a>>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TimeunitsDeclaration<'a> {
    Timeunit(TimeLiteral<'a>, Option<TimeLiteral<'a>>),
    Timeprecision(TimeLiteral<'a>),
    Timeunitprecision(TimeLiteral<'a>, TimeLiteral<'a>),
    Timeprecisionunit(TimeLiteral<'a>, TimeLiteral<'a>),
}
