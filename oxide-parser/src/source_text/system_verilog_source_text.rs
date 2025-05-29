// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.2

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;
use std::iter;

fn attribute_instance_vec_parser<'a>()
-> impl Parser<'a, &'a str, Vec<AttributeInstance>, ParserError<'a>> {
    attribute_instance_parser()
        .then_ignore(sep())
        .repeated()
        .collect::<Vec<AttributeInstance>>()
}

pub fn source_text_parser<'a>() -> impl Parser<'a, &'a str, SourceText, ParserError<'a>> {
    text::whitespace()
        .ignore_then(timeunits_declaration_parser().or_not())
        .then(
            sep()
                .ignore_then(description_parser())
                .repeated()
                .collect::<Vec<Description>>(),
        )
        .then_ignore(text::whitespace())
        .map(|(a, b)| SourceText(a, b))
}

pub fn description_parser<'a>() -> impl Parser<'a, &'a str, Description, ParserError<'a>> {
    let description_package_item_parser = attribute_instance_vec_parser()
        .then_ignore(sep())
        .then(package_item_parser())
        .map(|(a, b)| DescriptionPackageItem(a, b));
    let description_bind_directive_parser = attribute_instance_vec_parser()
        .then_ignore(sep())
        .then(bind_directive_parser())
        .map(|(a, b)| DescriptionBindDirective(a, b));
    choice((
        module_declaration_parser().map(|a| Description::ModuleDeclaration(Box::new(a))),
        udp_declaration_parser().map(|a| Description::UdpDeclaration(Box::new(a))),
        interface_declaration_parser().map(|a| Description::InterfaceDeclaration(Box::new(a))),
        program_declaration_parser().map(|a| Description::ProgramDeclaration(Box::new(a))),
        package_declaration_parser().map(|a| Description::PackageDeclaration(Box::new(a))),
        description_package_item_parser.map(|a| Description::DescriptionPackageItem(Box::new(a))),
        description_bind_directive_parser
            .map(|a| Description::DescriptionBindDirective(Box::new(a))),
        config_declaration_parser().map(|a| Description::ConfigDeclaration(Box::new(a))),
    ))
}

pub fn module_declaration_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclaration, ParserError<'a>> {
    choice((
        module_declaration_nonansi_parser()
            .map(|a| ModuleDeclaration::ModuleDeclarationNonansi(Box::new(a))),
        module_declaration_ansi_parser()
            .map(|a| ModuleDeclaration::ModuleDeclarationAnsi(Box::new(a))),
        module_declaration_wildcard_parser()
            .map(|a| ModuleDeclaration::ModuleDeclarationWildcard(Box::new(a))),
        module_declaration_extern_nonansi_parser()
            .map(|a| ModuleDeclaration::ModuleDeclarationExternNonansi(Box::new(a))),
        module_declaration_extern_ansi_parser()
            .map(|a| ModuleDeclaration::ModuleDeclarationExternAnsi(Box::new(a))),
    ))
}

pub fn module_declaration_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclarationNonansi, ParserError<'a>> {
    module_nonansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().then_ignore(sep()).or_not())
        .then(
            module_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endmodule"))
        .then(
            just(':')
                .padded()
                .ignore_then(module_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| ModuleDeclarationNonansi(a, b, c, d))
}

pub fn module_declaration_ansi_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclarationAnsi, ParserError<'a>> {
    module_ansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().then_ignore(sep()).or_not())
        .then(
            non_port_module_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endmodule"))
        .then(
            just(':')
                .padded()
                .ignore_then(module_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| ModuleDeclarationAnsi(a, b, c, d))
}

pub fn module_declaration_wildcard_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclarationWildcard, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(module_identifier_parser())
        .then_ignore(
            just('(')
                .ignore_then(sep())
                .ignore_then(just(".*"))
                .ignore_then(sep())
                .ignore_then(just(')'))
                .ignore_then(sep())
                .ignore_then(just(';')),
        )
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            module_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then_ignore(just("endmodule"))
        .then(
            just(':')
                .padded()
                .ignore_then(module_identifier_parser())
                .or_not(),
        )
        .map(|((((((a, b), c), d), e), f), g)| ModuleDeclarationWildcard(a, b, c, d, e, f, g))
}

pub fn module_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclarationExternNonansi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(module_nonansi_header_parser())
        .map(|a| ModuleDeclarationExternNonansi(a))
}

pub fn module_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, &'a str, ModuleDeclarationExternAnsi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(module_ansi_header_parser())
        .map(|a| ModuleDeclarationExternAnsi(a))
}

pub fn module_nonansi_header_parser<'a>()
-> impl Parser<'a, &'a str, ModuleNonansiHeader, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(module_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(list_of_ports_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|((((((a, b), c), d), e), f), g)| ModuleNonansiHeader(a, b, c, d, e, f, g))
}

pub fn module_ansi_header_parser<'a>() -> impl Parser<'a, &'a str, ModuleAnsiHeader, ParserError<'a>>
{
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(module_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(
            sep()
                .ignore_then(list_of_port_declarations_parser())
                .or_not(),
        )
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|((((((a, b), c), d), e), f), g)| ModuleAnsiHeader(a, b, c, d, e, f, g))
}

pub fn module_keyword_parser<'a>() -> impl Parser<'a, &'a str, ModuleKeyword, ParserError<'a>> {
    choice((
        just("module").to(ModuleKeyword::Module),
        just("macromodule").to(ModuleKeyword::Macromodule),
    ))
}

pub fn interface_declaration_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclaration, ParserError<'a>> {
    choice((
        interface_declaration_nonansi_parser()
            .map(|a| InterfaceDeclaration::InterfaceDeclarationNonansi(Box::new(a))),
        interface_declaration_ansi_parser()
            .map(|a| InterfaceDeclaration::InterfaceDeclarationAnsi(Box::new(a))),
        interface_declaration_wildcard_parser()
            .map(|a| InterfaceDeclaration::InterfaceDeclarationWildcard(Box::new(a))),
        interface_declaration_extern_nonansi_parser()
            .map(|a| InterfaceDeclaration::InterfaceDeclarationExternNonansi(Box::new(a))),
        interface_declaration_extern_ansi_parser()
            .map(|a| InterfaceDeclaration::InterfaceDeclarationExternAnsi(Box::new(a))),
    ))
}

pub fn interface_declaration_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclarationNonansi, ParserError<'a>> {
    interface_nonansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            interface_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endinterface"))
        .then(
            just(':')
                .padded()
                .ignore_then(interface_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| InterfaceDeclarationNonansi(a, b, c, d))
}

pub fn interface_declaration_ansi_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclarationAnsi, ParserError<'a>> {
    interface_ansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            non_port_interface_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endinterface"))
        .then(
            just(':')
                .padded()
                .ignore_then(interface_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| InterfaceDeclarationAnsi(a, b, c, d))
}

pub fn interface_declaration_wildcard_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclarationWildcard, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("interface"))
        .then_ignore(sep())
        .then(interface_identifier_parser())
        .then_ignore(
            just('(')
                .ignore_then(sep())
                .ignore_then(just(".*"))
                .ignore_then(sep())
                .ignore_then(just(')'))
                .ignore_then(sep())
                .ignore_then(just(';')),
        )
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            interface_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then_ignore(just("endinterface"))
        .then(
            just(':')
                .padded()
                .ignore_then(interface_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| InterfaceDeclarationWildcard(a, b, c, d, e))
}

pub fn interface_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclarationExternNonansi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(interface_nonansi_header_parser())
        .map(|a| InterfaceDeclarationExternNonansi(a))
}

pub fn interface_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceDeclarationExternAnsi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(interface_ansi_header_parser())
        .map(|a| InterfaceDeclarationExternAnsi(a))
}

pub fn interface_nonansi_header_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceNonansiHeader, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("interface"))
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(interface_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(list_of_ports_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|(((((a, b), c), d), e), f)| InterfaceNonansiHeader(a, b, c, d, e, f))
}

pub fn interface_ansi_header_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceAnsiHeader, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("interface"))
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(interface_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(
            sep()
                .ignore_then(list_of_port_declarations_parser())
                .or_not(),
        )
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|(((((a, b), c), d), e), f)| InterfaceAnsiHeader(a, b, c, d, e, f))
}

pub fn program_declaration_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclaration, ParserError<'a>> {
    choice((
        program_declaration_nonansi_parser()
            .map(|a| ProgramDeclaration::ProgramDeclarationNonansi(Box::new(a))),
        program_declaration_ansi_parser()
            .map(|a| ProgramDeclaration::ProgramDeclarationAnsi(Box::new(a))),
        program_declaration_wildcard_parser()
            .map(|a| ProgramDeclaration::ProgramDeclarationWildcard(Box::new(a))),
        program_declaration_extern_nonansi_parser()
            .map(|a| ProgramDeclaration::ProgramDeclarationExternNonansi(Box::new(a))),
        program_declaration_extern_ansi_parser()
            .map(|a| ProgramDeclaration::ProgramDeclarationExternAnsi(Box::new(a))),
    ))
}

pub fn program_declaration_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclarationNonansi, ParserError<'a>> {
    program_nonansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            program_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endprogram"))
        .then(
            just(':')
                .padded()
                .ignore_then(program_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| ProgramDeclarationNonansi(a, b, c, d))
}

pub fn program_declaration_ansi_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclarationAnsi, ParserError<'a>> {
    program_ansi_header_parser()
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            non_port_program_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then_ignore(just("endprogram"))
        .then(
            just(':')
                .padded()
                .ignore_then(program_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| ProgramDeclarationAnsi(a, b, c, d))
}

pub fn program_declaration_wildcard_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclarationWildcard, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("program"))
        .then_ignore(sep())
        .then(program_identifier_parser())
        .then_ignore(
            just('(')
                .ignore_then(sep())
                .ignore_then(just(".*"))
                .ignore_then(sep())
                .ignore_then(just(')'))
                .ignore_then(sep())
                .ignore_then(just(';')),
        )
        .then_ignore(sep())
        .then(timeunits_declaration_parser().or_not())
        .then_ignore(sep())
        .then(
            program_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then_ignore(just("endprogram"))
        .then(
            just(':')
                .padded()
                .ignore_then(program_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| ProgramDeclarationWildcard(a, b, c, d, e))
}

pub fn program_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclarationExternNonansi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(program_nonansi_header_parser())
        .map(|a| ProgramDeclarationExternNonansi(a))
}

pub fn program_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, &'a str, ProgramDeclarationExternAnsi, ParserError<'a>> {
    just("extern")
        .ignore_then(sep())
        .ignore_then(program_ansi_header_parser())
        .map(|a| ProgramDeclarationExternAnsi(a))
}

pub fn program_nonansi_header_parser<'a>()
-> impl Parser<'a, &'a str, ProgramNonansiHeader, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("program"))
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(program_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(list_of_ports_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|(((((a, b), c), d), e), f)| ProgramNonansiHeader(a, b, c, d, e, f))
}

pub fn program_ansi_header_parser<'a>()
-> impl Parser<'a, &'a str, ProgramAnsiHeader, ParserError<'a>> {
    attribute_instance_vec_parser()
        .then_ignore(just("program"))
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(program_identifier_parser())
        .then(
            sep()
                .ignore_then(package_import_declaration_parser())
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(sep().ignore_then(parameter_port_list_parser()).or_not())
        .then(
            sep()
                .ignore_then(list_of_port_declarations_parser())
                .or_not(),
        )
        .then_ignore(text::whitespace())
        .then_ignore(just(';'))
        .map(|(((((a, b), c), d), e), f)| ProgramAnsiHeader(a, b, c, d, e, f))
}

pub fn checker_declaration_parser<'a>()
-> impl Parser<'a, &'a str, CheckerDeclaration, ParserError<'a>> {
    let checker_declaration_port_list_parser = checker_port_list_parser()
        .delimited_by(just('(').ignore_then(sep()), sep().ignore_then(just(')')));
    let checker_declaration_item_parser = attribute_instance_vec_parser()
        .then_ignore(sep())
        .then(checker_or_generate_item_parser())
        .repeated()
        .collect::<Vec<(Vec<AttributeInstance>, CheckerOrGenerateItem)>>();
    just("checker")
        .ignore_then(sep())
        .ignore_then(checker_identifier_parser())
        .then_ignore(text::whitespace())
        .then(checker_declaration_port_list_parser.or_not())
        .then_ignore(just(';').padded())
        .then(checker_declaration_item_parser)
        .then_ignore(just("endchecker"))
        .then(
            just(':')
                .padded()
                .ignore_then(checker_identifier_parser())
                .or_not(),
        )
        .map(|(((a, b), c), d)| CheckerDeclaration(a, b, c, d))
}

pub fn class_declaration_parser<'a>() -> impl Parser<'a, &'a str, ClassDeclaration, ParserError<'a>>
{
    let virtual_parser = just("virtual").to(()).or_not();
    let class_declaration_extension_parser = just("extends")
        .ignore_then(sep())
        .ignore_then(class_type_parser())
        .then_ignore(sep())
        .then(
            choice((
                list_of_arguments_parser()
                    .map(|a| ClassDeclarationExtensionArguments::ListOfArguments(Box::new(a))),
                just("default").to(ClassDeclarationExtensionArguments::Default),
            ))
            .delimited_by(just('(').ignore_then(sep()), sep().ignore_then(just(')')))
            .or_not(),
        );
    let class_declaration_implementation_parser =
        just("implements").ignore_then(sep()).ignore_then(
            interface_class_type_parser()
                .map(|a| iter::once(a).collect())
                .foldl(
                    just(',')
                        .ignore_then(text::whitespace())
                        .ignore_then(interface_class_type_parser())
                        .repeated(),
                    foldl_vector,
                ),
        );
    virtual_parser
        .then_ignore(sep())
        .then_ignore(just("class"))
        .then_ignore(sep())
        .then(final_specifier_parser().or_not())
        .then_ignore(sep())
        .then(class_identifier_parser())
        .then_ignore(sep())
        .then(parameter_port_list_parser().or_not())
        .then_ignore(sep())
        .then(class_declaration_extension_parser.or_not())
        .then_ignore(sep())
        .then(class_declaration_implementation_parser.or_not())
        .then_ignore(just(';').padded())
        .then(
            class_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<ClassItem>>(),
        )
        .then_ignore(just("endclass"))
        .then(
            just(':')
                .padded()
                .ignore_then(class_identifier_parser())
                .or_not(),
        )
        .map(|(((((((a, b), c), d), e), f), g), h)| ClassDeclaration(a, b, c, d, e, f, g, h))
}

pub fn interface_class_declaration_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceClassDeclaration, ParserError<'a>> {
    let interface_class_declaration_extension_parser =
        just("extends").ignore_then(sep()).ignore_then(
            interface_class_type_parser()
                .map(|a| iter::once(a).collect())
                .foldl(
                    just(',')
                        .ignore_then(text::whitespace())
                        .ignore_then(interface_class_type_parser())
                        .repeated(),
                    foldl_vector,
                ),
        );
    just("interface")
        .ignore_then(sep())
        .ignore_then(just("class"))
        .ignore_then(sep())
        .ignore_then(class_identifier_parser())
        .then_ignore(sep())
        .then(parameter_port_list_parser().or_not())
        .then_ignore(sep())
        .then(interface_class_declaration_extension_parser.or_not())
        .then_ignore(just(';').padded())
        .then(
            interface_class_item_parser()
                .then_ignore(sep())
                .repeated()
                .collect::<Vec<InterfaceClassItem>>(),
        )
        .then_ignore(just("endclass"))
        .then(
            just(':')
                .padded()
                .ignore_then(class_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| InterfaceClassDeclaration(a, b, c, d, e))
}

pub fn package_declaration_parser<'a>()
-> impl Parser<'a, &'a str, PackageDeclaration, ParserError<'a>> {
    let attribute_package_items_parser = attribute_instance_vec_parser()
        .then(package_item_parser())
        .then_ignore(sep())
        .repeated()
        .collect::<Vec<(Vec<AttributeInstance>, PackageItem)>>();
    attribute_instance_vec_parser()
        .then_ignore(sep())
        .then_ignore(just("package"))
        .then_ignore(sep())
        .then(lifetime_parser().then_ignore(sep()).or_not())
        .then(package_identifier_parser())
        .then_ignore(just(';').padded())
        .then(timeunits_declaration_parser().then_ignore(sep()).or_not())
        .then(attribute_package_items_parser)
        .then_ignore(just("endpackage"))
        .then(
            just(':')
                .padded()
                .ignore_then(package_identifier_parser())
                .or_not(),
        )
        .map(|(((((a, b), c), d), e), f)| PackageDeclaration(a, b, c, d, e, f))
}

pub fn timeunits_declaration_parser<'a>()
-> impl Parser<'a, &'a str, TimeunitsDeclaration, ParserError<'a>> {
    let timeunit_parser = text::whitespace()
        .ignore_then(just("timeunit"))
        .ignore_then(sep())
        .ignore_then(time_literal_parser())
        .then(
            text::whitespace()
                .ignore_then(just('/'))
                .ignore_then(text::whitespace())
                .ignore_then(time_literal_parser())
                .or_not(),
        )
        .then_ignore(text::whitespace())
        .then_ignore(just(';'));
    let timeprecision_parser = text::whitespace()
        .ignore_then(just("timeprecision"))
        .ignore_then(sep())
        .ignore_then(time_literal_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';'));
    let timeunitprecision_parser = (text::whitespace()
        .ignore_then(just("timeunit"))
        .ignore_then(sep())
        .ignore_then(time_literal_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';')))
    .then_ignore(sep())
    .then(
        text::whitespace()
            .ignore_then(just("timeprecision"))
            .ignore_then(sep())
            .ignore_then(time_literal_parser())
            .then_ignore(text::whitespace())
            .then_ignore(just(';')),
    );
    let timeprecisionunit_parser = (text::whitespace()
        .ignore_then(just("timeprecision"))
        .ignore_then(sep())
        .ignore_then(time_literal_parser())
        .then_ignore(text::whitespace())
        .then_ignore(just(';')))
    .then_ignore(sep())
    .then(
        text::whitespace()
            .ignore_then(just("timeunit"))
            .ignore_then(sep())
            .ignore_then(time_literal_parser())
            .then_ignore(text::whitespace())
            .then_ignore(just(';')),
    );
    choice((
        timeunit_parser.map(|(a, b)| TimeunitsDeclaration::Timeunit(a, b)),
        timeprecision_parser.map(|a| TimeunitsDeclaration::Timeprecision(a)),
        timeunitprecision_parser.map(|(a, b)| TimeunitsDeclaration::Timeunitprecision(a, b)),
        timeprecisionunit_parser.map(|(a, b)| TimeunitsDeclaration::Timeprecisionunit(a, b)),
    ))
}
