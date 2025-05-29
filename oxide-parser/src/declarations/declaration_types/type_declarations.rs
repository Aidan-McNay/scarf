// =======================================================================
// type_declarations.rs
// =======================================================================
// Parsing 1800-2023 A.2.1.3

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn package_import_declaration_parser<'a>()
-> impl Parser<'a, &'a str, PackageImportDeclaration, ParserError<'a>> {
    todo()
}

pub fn lifetime_parser<'a>() -> impl Parser<'a, &'a str, Lifetime, ParserError<'a>> {
    choice((
        just("static").to(Lifetime::Static),
        just("automatic").to(Lifetime::Automatic),
    ))
}
