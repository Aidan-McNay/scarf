// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.2

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceText(pub Option<TimeunitsDeclaration>, pub Vec<Description>);

#[derive(Clone, Debug, PartialEq)]
pub enum Description {
    ModuleDeclaration(Box<ModuleDeclaration>),
    UdpDeclaration(Box<UdpDeclaration>),
    InterfaceDeclaration(Box<InterfaceDeclaration>),
    ProgramDeclaration(Box<ProgramDeclaration>),
    PackageDeclaration(Box<PackageDeclaration>),
    DescriptionPackageItem(Box<DescriptionPackageItem>),
    DescriptionBindDirective(Box<DescriptionBindDirective>),
    ConfigDeclaration(Box<ConfigDeclaration>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionPackageItem(pub Vec<AttributeInstance>, pub PackageItem);

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionBindDirective(pub Vec<AttributeInstance>, pub BindDirective);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNonansiHeader(
    pub Vec<AttributeInstance>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleAnsiHeader(
    pub Vec<AttributeInstance>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleDeclaration {
    ModuleDeclarationNonansi(Box<ModuleDeclarationNonansi>),
    ModuleDeclarationAnsi(Box<ModuleDeclarationAnsi>),
    ModuleDeclarationWildcard(Box<ModuleDeclarationWildcard>),
    ModuleDeclarationExternNonansi(Box<ModuleDeclarationExternNonansi>),
    ModuleDeclarationExternAnsi(Box<ModuleDeclarationExternAnsi>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationNonansi(
    pub ModuleNonansiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<ModuleItem>,
    pub Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationAnsi(
    pub ModuleAnsiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<NonPortModuleItem>,
    pub Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationWildcard(
    pub Vec<AttributeInstance>,
    pub ModuleKeyword,
    pub Option<Lifetime>,
    pub ModuleIdentifier,
    pub Option<TimeunitsDeclaration>,
    pub Vec<ModuleItem>,
    pub Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternNonansi(pub ModuleNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternAnsi(pub ModuleAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKeyword {
    Module,
    Macromodule,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceDeclaration {
    InterfaceDeclarationNonansi(Box<InterfaceDeclarationNonansi>),
    InterfaceDeclarationAnsi(Box<InterfaceDeclarationAnsi>),
    InterfaceDeclarationWildcard(Box<InterfaceDeclarationWildcard>),
    InterfaceDeclarationExternNonansi(Box<InterfaceDeclarationExternNonansi>),
    InterfaceDeclarationExternAnsi(Box<InterfaceDeclarationExternAnsi>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationNonansi(
    pub InterfaceNonansiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<InterfaceItem>,
    pub Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationAnsi(
    pub InterfaceAnsiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<NonPortInterfaceItem>,
    pub Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationWildcard(
    pub Vec<AttributeInstance>,
    pub InterfaceIdentifier,
    pub Option<TimeunitsDeclaration>,
    pub Vec<InterfaceItem>,
    pub Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternNonansi(pub InterfaceNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternAnsi(pub InterfaceAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceNonansiHeader(
    pub Vec<AttributeInstance>,
    pub Option<Lifetime>,
    pub InterfaceIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceAnsiHeader(
    pub Vec<AttributeInstance>,
    pub Option<Lifetime>,
    pub InterfaceIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum ProgramDeclaration {
    ProgramDeclarationNonansi(Box<ProgramDeclarationNonansi>),
    ProgramDeclarationAnsi(Box<ProgramDeclarationAnsi>),
    ProgramDeclarationWildcard(Box<ProgramDeclarationWildcard>),
    ProgramDeclarationExternNonansi(Box<ProgramDeclarationExternNonansi>),
    ProgramDeclarationExternAnsi(Box<ProgramDeclarationExternAnsi>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationNonansi(
    pub ProgramNonansiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<ProgramItem>,
    pub Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationAnsi(
    pub ProgramAnsiHeader,
    pub Option<TimeunitsDeclaration>,
    pub Vec<NonPortProgramItem>,
    pub Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationWildcard(
    pub Vec<AttributeInstance>,
    pub ProgramIdentifier,
    pub Option<TimeunitsDeclaration>,
    pub Vec<ProgramItem>,
    pub Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternNonansi(pub ProgramNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternAnsi(pub ProgramAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNonansiHeader(
    pub Vec<AttributeInstance>,
    pub Option<Lifetime>,
    pub ProgramIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramAnsiHeader(
    pub Vec<AttributeInstance>,
    pub Option<Lifetime>,
    pub ProgramIdentifier,
    pub Vec<PackageImportDeclaration>,
    pub Option<ParameterPortList>,
    pub Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclaration(
    pub CheckerIdentifier,
    pub Option<CheckerPortList>,
    pub Option<CheckerDeclarationCheckerOrGenerateItem>,
    pub Option<CheckerIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclarationCheckerOrGenerateItem(
    pub Option<AttributeInstance>,
    pub CheckerOrGenerateItem,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDeclaration(
    pub Option<Virtual>,
    pub Option<FinalSpecifier>,
    pub ClassIdentifier,
    pub Option<ParameterPortList>,
    pub Option<(ClassType, Option<ListOfArguments>)>,
    pub Option<Vec<InterfaceClassType>>,
    pub Vec<ClassItem>,
    pub Option<ClassIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassDeclaration(
    pub ClassIdentifier,
    pub Option<ParameterPortList>,
    pub Option<Vec<InterfaceClassType>>,
    pub Vec<InterfaceClassItem>,
    pub Option<ClassIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageDeclaration(
    pub Vec<AttributeInstance>,
    pub Option<Lifetime>,
    pub PackageIdentifier,
    pub Option<TimeunitsDeclaration>,
    pub Vec<(Vec<AttributeInstance>, PackageItem)>,
    pub Option<PackageIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TimeunitsDeclaration {
    Timeunit(TimeLiteral, Option<TimeLiteral>),
    Timeprecision(TimeLiteral),
    Timeunitprecision(TimeLiteral, TimeLiteral),
    Timeprecisionunit(TimeLiteral, TimeLiteral),
}
