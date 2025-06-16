// =======================================================================
// type_declarations.rs
// =======================================================================
// Parsing 1800-2023 A.2.1.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn package_import_declaration_parser<'a, I>()
-> impl Parser<'a, I, PackageImportDeclaration, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn forward_type_parser<'a, I>() -> impl Parser<'a, I, ForwardType<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Enum).map(|a| ForwardType::Enum(a)),
        token(Token::Struct).map(|a| ForwardType::Struct(a)),
        token(Token::Union).map(|a| ForwardType::Union(a)),
        token(Token::Class).map(|a| ForwardType::Class(a)),
        token(Token::Interface)
            .then(token(Token::Class))
            .map(|(a, b)| ForwardType::InterfaceClass(a, b)),
    ))
}

pub fn lifetime_parser<'a, I>() -> impl Parser<'a, I, Lifetime<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Static).map(|a| Lifetime::Static(a)),
        token(Token::Automatic).map(|a| Lifetime::Automatic(a)),
    ))
}
