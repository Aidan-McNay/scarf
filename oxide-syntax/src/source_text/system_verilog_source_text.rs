// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// AST Nodes from 1800-2023 A.1.2

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceText(Option<TimeunitsDeclaration>, Vec<Description>);

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
pub struct DescriptionPackageItem(Vec<AttributeInstance>, PackageItem);

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptionBindDirective(Vec<AttributeInstance>, BindDirective);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNonansiHeader(
    Vec<AttributeInstance>,
    ModuleKeyword,
    Option<Lifetime>,
    ModuleIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleAnsiHeader(
    Vec<AttributeInstance>,
    ModuleKeyword,
    Option<Lifetime>,
    ModuleIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    Option<ListOfPortDeclarations>,
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
    ModuleNonansiHeader,
    Option<TimeunitsDeclaration>,
    Vec<ModuleItem>,
    Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationAnsi(
    ModuleAnsiHeader,
    Option<TimeunitsDeclaration>,
    Vec<NonPortModuleItem>,
    Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationWildcard(
    Vec<AttributeInstance>,
    ModuleKeyword,
    Option<Lifetime>,
    ModuleIdentifier,
    Option<TimeunitsDeclaration>,
    Vec<ModuleItem>,
    Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternNonansi(ModuleNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternAnsi(ModuleAnsiHeader);

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
    InterfaceNonansiHeader,
    Option<TimeunitsDeclaration>,
    Vec<InterfaceItem>,
    Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationAnsi(
    InterfaceAnsiHeader,
    Option<TimeunitsDeclaration>,
    Vec<NonPortInterfaceItem>,
    Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationWildcard(
    Vec<AttributeInstance>,
    InterfaceIdentifier,
    Option<TimeunitsDeclaration>,
    Vec<InterfaceItem>,
    Option<InterfaceIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternNonansi(InterfaceNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceDeclarationExternAnsi(InterfaceAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceNonansiHeader(
    Vec<AttributeInstance>,
    Option<Lifetime>,
    InterfaceIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceAnsiHeader(
    Vec<AttributeInstance>,
    Option<Lifetime>,
    InterfaceIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    Option<ListOfPortDeclarations>,
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
    ProgramNonansiHeader,
    Option<TimeunitsDeclaration>,
    Vec<ProgramItem>,
    Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationAnsi(
    ProgramAnsiHeader,
    Option<TimeunitsDeclaration>,
    Vec<NonPortProgramItem>,
    Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationWildcard(
    Vec<AttributeInstance>,
    ProgramIdentifier,
    Option<TimeunitsDeclaration>,
    Vec<ProgramItem>,
    Option<ProgramIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternNonansi(ProgramNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramDeclarationExternAnsi(ProgramAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramNonansiHeader(
    Vec<AttributeInstance>,
    Option<Lifetime>,
    ProgramIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    ListOfPorts,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ProgramAnsiHeader(
    Vec<AttributeInstance>,
    Option<Lifetime>,
    ProgramIdentifier,
    Vec<PackageImportDeclaration>,
    Option<ParameterPortList>,
    Option<ListOfPortDeclarations>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclaration(
    CheckerIdentifier,
    Option<CheckerPortList>,
    Option<CheckerDeclarationCheckerOrGenerateItem>,
    Option<CheckerIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerDeclarationCheckerOrGenerateItem(
    Option<AttributeInstance>,
    CheckerOrGenerateItem,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDeclaration(
    Option<Virtual>,
    Option<FinalSpecifier>,
    ClassIdentifier,
    Option<ParameterPortList>,
    Option<(ClassType, Option<ListOfArguments>)>,
    Option<Vec<InterfaceClassType>>,
    Vec<ClassItem>,
    Option<ClassIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceClassDeclaration(
    ClassIdentifier,
    Option<ParameterPortList>,
    Option<Vec<InterfaceClassType>>,
    Vec<InterfaceClassItem>,
    Option<ClassIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct PackageDeclaration(
    Vec<AttributeInstance>,
    Option<Lifetime>,
    PackageIdentifier,
    Option<TimeunitsDeclaration>,
    Vec<(Vec<AttributeInstance>, PackageItem)>,
    Option<PackageIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub enum TimeunitsDeclaration {
    Timeunit(TimeLiteral, Option<TimeLiteral>),
    Timeprecision(TimeLiteral),
    Timeunitprecision(TimeLiteral, TimeLiteral),
    Timeprecisionunit(TimeLiteral, TimeLiteral),
}
