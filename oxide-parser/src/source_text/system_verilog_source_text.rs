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
        .then_ignore(text::whitespace())
        .repeated()
        .collect::<Vec<AttributeInstance>>()
}

pub fn source_text_parser<'a>() -> impl Parser<'a, &'a str, SourceText, ParserError<'a>> {
    timeunits_declaration_parser()
        .or_not()
        .map(|a| SourceText(a, Vec::new()))
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
