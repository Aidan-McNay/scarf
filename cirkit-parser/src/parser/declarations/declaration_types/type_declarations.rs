// =======================================================================
// type_declarations.rs
// =======================================================================
// Parsing 1800-2023 A.2.1.3

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn package_import_declaration_parser<'a, I>()
-> impl Parser<'a, I, PackageImportDeclaration, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn lifetime_parser<'a, I>() -> impl Parser<'a, I, Lifetime<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::Static = e => Lifetime::Static(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::Automatic = e => Lifetime::Automatic(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
    }
    .then(extra_node_parser())
    .map(|(lifetime, b)| match lifetime {
        Lifetime::Static(metadata) => Lifetime::Static(replace_nodes(metadata, b)),
        Lifetime::Automatic(metadata) => Lifetime::Automatic(replace_nodes(metadata, b)),
    })
}
