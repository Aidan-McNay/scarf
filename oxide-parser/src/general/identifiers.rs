// =======================================================================
// mod.rs
// =======================================================================
// Parsing for 1800-2023 A.9.3

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn checker_identifier_parser<'a>()
-> impl Parser<'a, &'a str, CheckerIdentifier, ParserError<'a>> {
    identifier_parser().map(|a| CheckerIdentifier(a))
}

pub fn class_identifier_parser<'a>() -> impl Parser<'a, &'a str, ClassIdentifier, ParserError<'a>> {
    identifier_parser().map(|a| ClassIdentifier(a))
}

pub fn escaped_identifier_parser<'a>() -> impl Parser<'a, &'a str, SimpleIdentifier, ParserError<'a>>
{
    let char_parser = one_of('!'..='~').map(String::from);
    just('\\')
        .ignore_then(
            char_parser
                .clone()
                .foldl(char_parser.repeated(), |a, b| a + &b),
        )
        .then_ignore(text::whitespace().at_least(1))
}

pub fn module_identifier_parser<'a>() -> impl Parser<'a, &'a str, ModuleIdentifier, ParserError<'a>>
{
    identifier_parser().map(|a| ModuleIdentifier(a))
}

pub fn identifier_parser<'a>() -> impl Parser<'a, &'a str, Identifier, ParserError<'a>> {
    choice((
        simple_identifier_parser().map(|a| Identifier::SimpleIdentifier(Box::new(a))),
        escaped_identifier_parser().map(|a| Identifier::EscapedIdentifier(Box::new(a))),
    ))
}

pub fn interface_identifier_parser<'a>()
-> impl Parser<'a, &'a str, InterfaceIdentifier, ParserError<'a>> {
    identifier_parser().map(|a| InterfaceIdentifier(a))
}

pub fn package_identifier_parser<'a>()
-> impl Parser<'a, &'a str, PackageIdentifier, ParserError<'a>> {
    identifier_parser().map(|a| PackageIdentifier(a))
}

pub fn program_identifier_parser<'a>()
-> impl Parser<'a, &'a str, ProgramIdentifier, ParserError<'a>> {
    identifier_parser().map(|a| ProgramIdentifier(a))
}

pub fn simple_identifier_parser<'a>() -> impl Parser<'a, &'a str, SimpleIdentifier, ParserError<'a>>
{
    let alpha_parser = choice((one_of('a'..='z'), one_of('A'..='Z'))).map(String::from);
    let char_parser = choice((
        one_of('a'..='z'),
        one_of('A'..='Z'),
        one_of('0'..='9'),
        just('_'),
        just('$'),
    ))
    .map(String::from);
    alpha_parser.foldl(char_parser.repeated(), |a, b| a + &b)
}
