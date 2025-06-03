// =======================================================================
// mod.rs
// =======================================================================
// Parsing for 1800-2023 A.9.3

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn checker_identifier_parser<'a, I>()
-> impl Parser<'a, I, CheckerIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CheckerIdentifier(a))
}

pub fn class_identifier_parser<'a, I>() -> impl Parser<'a, I, ClassIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ClassIdentifier(a))
}

pub fn module_identifier_parser<'a, I>() -> impl Parser<'a, I, ModuleIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ModuleIdentifier(a))
}

pub fn identifier_parser<'a, I>() -> impl Parser<'a, I, Identifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    select! {
        Token::SimpleIdentifier(text) => Identifier::SimpleIdentifier(text),
        Token::EscapedIdentifier(text) => Identifier::EscapedIdentifier(text),
    }
}

pub fn interface_identifier_parser<'a, I>()
-> impl Parser<'a, I, InterfaceIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InterfaceIdentifier(a))
}

pub fn package_identifier_parser<'a, I>()
-> impl Parser<'a, I, PackageIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PackageIdentifier(a))
}

pub fn program_identifier_parser<'a, I>()
-> impl Parser<'a, I, ProgramIdentifier<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ProgramIdentifier(a))
}
