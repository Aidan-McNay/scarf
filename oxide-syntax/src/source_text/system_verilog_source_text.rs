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
    ModuleDeclarationKeyword(Box<ModuleDeclarationKeyword>),
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
pub struct ModuleDeclarationKeyword(
    Vec<AttributeInstance>,
    ModuleKeyword,
    Option<Lifetime>,
    ModuleIdentifier,
    Option<TimeunitsDeclaration>,
    Vec<ModuleItem>,
    Option<ModuleIdentifier>,
);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternNonnsi(ModuleNonansiHeader);

#[derive(Clone, Debug, PartialEq)]
pub struct ModuleDeclarationExternAnsi(ModuleAnsiHeader);

#[derive(Clone, Debug, PartialEq)]
pub enum ModuleKeyword {
    Module,
    Macromodule,
}
