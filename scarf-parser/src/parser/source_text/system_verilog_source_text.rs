// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub(crate) fn attribute_instance_vec_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, Vec<AttributeInstance<'a>>, ParserError<'a>> + Clone {
    attribute_instance_parser(constant_expression_parser(expression_parser()))
        .repeated()
        .collect::<Vec<AttributeInstance>>()
}

pub fn source_text_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, SourceText<'a>, ParserError<'a>> + Clone {
    extra_node_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            description_parser()
                .repeated()
                .collect::<Vec<Description<'a>>>(),
        )
        .map(|((a, b), c)| SourceText(a, b, c))
        .boxed()
}

pub fn description_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, Description<'a>, ParserError<'a>> + Clone {
    let description_package_item_parser = attribute_instance_vec_parser()
        .then(package_item_parser())
        .map(|(a, b)| DescriptionPackageItem(a, b));
    let description_bind_directive_parser = attribute_instance_vec_parser()
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
    .boxed()
}

pub fn module_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclaration<'a>, ParserError<'a>> + Clone {
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
    .boxed()
}

pub fn module_declaration_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclarationNonansi<'a>, ParserError<'a>> + Clone {
    module_nonansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            module_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endmodule))
        .then(
            token(Token::Colon)
                .then(module_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| ModuleDeclarationNonansi(a, b, c, d, e))
        .boxed()
}

pub fn module_declaration_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclarationAnsi<'a>, ParserError<'a>> + Clone {
    module_ansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            non_port_module_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endmodule))
        .then(
            token(Token::Colon)
                .then(module_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| ModuleDeclarationAnsi(a, b, c, d, e))
        .boxed()
}

pub fn module_nonansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleNonansiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then(lifetime_parser().or_not())
        .then(module_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_ports_parser())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| ModuleNonansiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn module_ansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleAnsiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then(lifetime_parser().or_not())
        .then(module_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_port_declarations_parser().or_not())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| ModuleAnsiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn module_declaration_wildcard_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclarationWildcard<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(module_keyword_parser())
        .then(lifetime_parser().or_not())
        .then(module_identifier_parser())
        .then(token(Token::Paren))
        .then(token(Token::Period))
        .then(token(Token::StarEparen))
        .then(token(Token::SColon))
        .then(timeunits_declaration_parser().or_not())
        .then(
            module_item_parser()
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then(token(Token::Endmodule))
        .then(
            token(Token::Colon)
                .then(module_identifier_parser())
                .or_not(),
        )
        .map(|(((((((((((a, b), c), d), e), f), g), h), i), j), k), l)| {
            ModuleDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k, l)
        })
        .boxed()
}

pub fn module_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclarationExternNonansi<'a>, ParserError<'a>> + Clone {
    token(Token::Extern)
        .then(module_nonansi_header_parser())
        .map(|(a, b)| ModuleDeclarationExternNonansi(a, b))
        .boxed()
}

pub fn module_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleDeclarationExternAnsi<'a>, ParserError<'a>> + Clone {
    token(Token::Extern)
        .then(module_ansi_header_parser())
        .map(|(a, b)| ModuleDeclarationExternAnsi(a, b))
        .boxed()
}

pub fn module_keyword_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ModuleKeyword<'a>, ParserError<'a>> + Clone {
    choice((
        token(Token::Module).map(|a| ModuleKeyword::Module(a)),
        token(Token::Macromodule).map(|a| ModuleKeyword::Macromodule(a)),
    ))
    .boxed()
}

pub fn interface_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclaration<'a>, ParserError<'a>> + Clone {
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
    .boxed()
}

pub fn interface_declaration_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclarationNonansi<'a>, ParserError<'a>> + Clone {
    interface_nonansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            interface_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endinterface))
        .then(
            token(Token::Colon)
                .then(interface_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| InterfaceDeclarationNonansi(a, b, c, d, e))
        .boxed()
}

pub fn interface_declaration_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclarationAnsi<'a>, ParserError<'a>> + Clone {
    interface_ansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            non_port_interface_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endinterface))
        .then(
            token(Token::Colon)
                .then(interface_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| InterfaceDeclarationAnsi(a, b, c, d, e))
        .boxed()
}

pub fn interface_declaration_wildcard_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclarationWildcard<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Interface))
        .then(interface_identifier_parser())
        .then(token(Token::Paren))
        .then(token(Token::Period))
        .then(token(Token::StarEparen))
        .then(token(Token::SColon))
        .then(timeunits_declaration_parser().or_not())
        .then(
            interface_item_parser()
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then(token(Token::Endinterface))
        .then(
            token(Token::Colon)
                .then(interface_identifier_parser())
                .or_not(),
        )
        .map(|((((((((((a, b), c), d), e), f), g), h), i), j), k)| {
            InterfaceDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k)
        })
        .boxed()
}

pub fn interface_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclarationExternNonansi<'a>, ParserError<'a>> + Clone
{
    token(Token::Extern)
        .then(interface_nonansi_header_parser())
        .map(|(a, b)| InterfaceDeclarationExternNonansi(a, b))
        .boxed()
}

pub fn interface_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceDeclarationExternAnsi<'a>, ParserError<'a>> + Clone {
    token(Token::Extern)
        .then(interface_ansi_header_parser())
        .map(|(a, b)| InterfaceDeclarationExternAnsi(a, b))
        .boxed()
}

pub fn interface_nonansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceNonansiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Interface))
        .then(lifetime_parser().or_not())
        .then(interface_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_ports_parser())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| InterfaceNonansiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn interface_ansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceAnsiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Interface))
        .then(lifetime_parser().or_not())
        .then(interface_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_port_declarations_parser().or_not())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| InterfaceAnsiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn program_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclaration<'a>, ParserError<'a>> + Clone {
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
    .boxed()
}

pub fn program_declaration_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclarationNonansi<'a>, ParserError<'a>> + Clone {
    program_nonansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            program_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endprogram))
        .then(
            token(Token::Colon)
                .then(program_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| ProgramDeclarationNonansi(a, b, c, d, e))
        .boxed()
}

pub fn program_declaration_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclarationAnsi<'a>, ParserError<'a>> + Clone {
    program_ansi_header_parser()
        .then(timeunits_declaration_parser().or_not())
        .then(
            non_port_program_item_parser()
                .repeated()
                .collect::<Vec<NonPortProgramItem>>(),
        )
        .then(token(Token::Endprogram))
        .then(
            token(Token::Colon)
                .then(program_identifier_parser())
                .or_not(),
        )
        .map(|((((a, b), c), d), e)| ProgramDeclarationAnsi(a, b, c, d, e))
        .boxed()
}

pub fn program_declaration_wildcard_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclarationWildcard<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Program))
        .then(program_identifier_parser())
        .then(token(Token::Paren))
        .then(token(Token::Period))
        .then(token(Token::StarEparen))
        .then(token(Token::SColon))
        .then(timeunits_declaration_parser().or_not())
        .then(
            program_item_parser()
                .repeated()
                .collect::<Vec<ProgramItem>>(),
        )
        .then(token(Token::Endprogram))
        .then(
            token(Token::Colon)
                .then(program_identifier_parser())
                .or_not(),
        )
        .map(|((((((((((a, b), c), d), e), f), g), h), i), j), k)| {
            ProgramDeclarationWildcard(a, b, c, d, e, f, g, h, i, j, k)
        })
        .boxed()
}

pub fn program_declaration_extern_nonansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclarationExternNonansi<'a>, ParserError<'a>> + Clone {
    token(Token::Extern)
        .then(program_nonansi_header_parser())
        .map(|(a, b)| ProgramDeclarationExternNonansi(a, b))
        .boxed()
}

pub fn program_declaration_extern_ansi_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramDeclarationExternAnsi<'a>, ParserError<'a>> + Clone {
    token(Token::Extern)
        .then(program_ansi_header_parser())
        .map(|(a, b)| ProgramDeclarationExternAnsi(a, b))
        .boxed()
}

pub fn program_nonansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramNonansiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Program))
        .then(lifetime_parser().or_not())
        .then(program_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_ports_parser())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| ProgramNonansiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn program_ansi_header_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ProgramAnsiHeader<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(token(Token::Program))
        .then(lifetime_parser().or_not())
        .then(program_identifier_parser())
        .then(
            package_import_declaration_parser()
                .repeated()
                .collect::<Vec<PackageImportDeclaration>>(),
        )
        .then(parameter_port_list_parser().or_not())
        .then(list_of_port_declarations_parser().or_not())
        .then(token(Token::SColon))
        .map(|(((((((a, b), c), d), e), f), g), h)| ProgramAnsiHeader(a, b, c, d, e, f, g, h))
        .boxed()
}

pub fn checker_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, CheckerDeclaration<'a>, ParserError<'a>> + Clone {
    let checker_declaration_port_list_parser = token(Token::Paren)
        .then(checker_port_list_parser())
        .then(token(Token::EParen))
        .map(|((a, b), c)| (a, b, c));
    let checker_declaration_item_parser = attribute_instance_vec_parser()
        .then(checker_or_generate_item_parser())
        .repeated()
        .collect::<Vec<(Vec<AttributeInstance>, CheckerOrGenerateItem)>>();
    token(Token::Checker)
        .then(checker_identifier_parser())
        .then(checker_declaration_port_list_parser.or_not())
        .then(token(Token::SColon))
        .then(checker_declaration_item_parser)
        .then(token(Token::Endchecker))
        .then(
            token(Token::Colon)
                .then(checker_identifier_parser())
                .or_not(),
        )
        .map(|((((((a, b), c), d), e), f), g)| CheckerDeclaration(a, b, c, d, e, f, g))
        .boxed()
}

pub fn class_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassDeclaration<'a>, ParserError<'a>> + Clone {
    let class_declaration_extension_parser = token(Token::Extends)
        .then(class_type_parser())
        .then(
            token(Token::Paren)
                .then(choice((
                    list_of_arguments_parser()
                        .map(|a| ClassDeclarationExtensionArguments::ListOfArguments(Box::new(a))),
                    token(Token::Default)
                        .map(|metadata| ClassDeclarationExtensionArguments::Default(metadata)),
                )))
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .map(|((a, b), c)| (a, b, c));
    let class_declaration_implementation_parser = token(Token::Implements)
        .then(interface_class_type_parser())
        .then(
            token(Token::Comma)
                .then(interface_class_type_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, InterfaceClassType)>>(),
        )
        .map(|((a, b), c)| (a, b, c));
    token(Token::Virtual)
        .or_not()
        .then(token(Token::Class))
        .then(final_specifier_parser().or_not())
        .then(class_identifier_parser())
        .then(parameter_port_list_parser().or_not())
        .then(class_declaration_extension_parser.or_not())
        .then(class_declaration_implementation_parser.or_not())
        .then(token(Token::SColon))
        .then(class_item_parser().repeated().collect::<Vec<ClassItem>>())
        .then(token(Token::Endclass))
        .then(token(Token::Colon).then(class_identifier_parser()).or_not())
        .map(|((((((((((a, b), c), d), e), f), g), h), i), j), k)| {
            ClassDeclaration(a, b, c, d, e, f, g, h, i, j, k)
        })
        .boxed()
}

pub fn interface_class_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceClassDeclaration<'a>, ParserError<'a>> + Clone {
    let interface_class_declaration_extension_parser = token(Token::Extends)
        .then(interface_class_type_parser())
        .then(
            token(Token::Comma)
                .then(interface_class_type_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, InterfaceClassType)>>(),
        )
        .map(|((a, b), c)| (a, b, c));
    token(Token::Interface)
        .then(token(Token::Class))
        .then(class_identifier_parser())
        .then(parameter_port_list_parser().or_not())
        .then(interface_class_declaration_extension_parser.or_not())
        .then(token(Token::SColon))
        .then(
            interface_class_item_parser()
                .repeated()
                .collect::<Vec<InterfaceClassItem>>(),
        )
        .then(token(Token::Endclass))
        .then(token(Token::Colon).then(class_identifier_parser()).or_not())
        .map(|((((((((a, b), c), d), e), f), g), h), i)| {
            InterfaceClassDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .boxed()
}

pub fn package_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, PackageDeclaration<'a>, ParserError<'a>> + Clone {
    let attribute_package_items_parser = attribute_instance_vec_parser()
        .then(package_item_parser())
        .repeated()
        .collect::<Vec<(Vec<AttributeInstance>, PackageItem)>>();
    attribute_instance_vec_parser()
        .then(token(Token::Package))
        .then(lifetime_parser().or_not())
        .then(package_identifier_parser())
        .then(token(Token::SColon))
        .then(timeunits_declaration_parser().or_not())
        .then(attribute_package_items_parser)
        .then(token(Token::Endpackage))
        .then(
            token(Token::Colon)
                .then(package_identifier_parser())
                .or_not(),
        )
        .map(|((((((((a, b), c), d), e), f), g), h), i)| {
            PackageDeclaration(a, b, c, d, e, f, g, h, i)
        })
        .boxed()
}

pub fn timeunits_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, TimeunitsDeclaration<'a>, ParserError<'a>> + Clone {
    let timeunit_parser = token(Token::Timeunit)
        .then(time_literal_parser())
        .then(token(Token::Slash).then(time_literal_parser()).or_not())
        .then(token(Token::SColon))
        .map(|(((a, b), c), d)| TimeunitsDeclaration::Timeunit(a, b, c, d));
    let timeprecision_parser = token(Token::Timeprecision)
        .then(time_literal_parser())
        .then(token(Token::SColon))
        .map(|((a, b), c)| TimeunitsDeclaration::Timeprecision(a, b, c));
    let timeunitprecision_parser = token(Token::Timeunit)
        .then(time_literal_parser())
        .then(token(Token::SColon))
        .then(token(Token::Timeprecision))
        .then(time_literal_parser())
        .then(token(Token::SColon))
        .map(|(((((a, b), c), d), e), f)| {
            TimeunitsDeclaration::Timeunitprecision(a, b, c, d, e, f)
        });
    let timeprecisionunit_parser = token(Token::Timeprecision)
        .then(time_literal_parser())
        .then(token(Token::SColon))
        .then(token(Token::Timeunit))
        .then(time_literal_parser())
        .then(token(Token::SColon))
        .map(|(((((a, b), c), d), e), f)| {
            TimeunitsDeclaration::Timeprecisionunit(a, b, c, d, e, f)
        });
    choice((
        timeunit_parser,
        timeprecision_parser,
        timeunitprecision_parser,
        timeprecisionunit_parser,
    ))
    .boxed()
}
