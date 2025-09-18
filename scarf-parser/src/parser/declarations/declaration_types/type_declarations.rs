// =======================================================================
// type_declarations.rs
// =======================================================================
// Parsing 1800-2023 A.2.1.3

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, todo};

pub fn package_import_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PackageImportDeclaration, VerboseError<'s>> {
    todo(input)
}

pub fn forward_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForwardType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Enum).map(|a| ForwardType::Enum(a)),
        token(Token::Struct).map(|a| ForwardType::Struct(a)),
        token(Token::Union).map(|a| ForwardType::Union(a)),
        token(Token::Class).map(|a| ForwardType::Class(a)),
        (token(Token::Interface), token(Token::Class))
            .map(|(a, b)| ForwardType::InterfaceClass(a, b)),
    ))
    .parse_next(input)
}

pub fn lifetime_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Lifetime<'s>, VerboseError<'s>> {
    alt((
        token(Token::Static).map(|a| Lifetime::Static(a)),
        token(Token::Automatic).map(|a| Lifetime::Automatic(a)),
    ))
    .parse_next(input)
}
