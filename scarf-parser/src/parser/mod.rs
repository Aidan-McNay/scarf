// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

use crate::*;
use lexer::Token;
use scarf_syntax::Metadata;
use scarf_syntax::ModuleKeyword;
use winnow::Parser;
use winnow::error::ModalResult;
use winnow::stream::TokenSlice;
use winnow::token::literal;

pub type ParserStream<'s> = TokenSlice<'s, Token<'s>>;

pub fn do_nothing_parser<'s>(input: &mut ParserStream<'s>) -> ModalResult<ModuleKeyword<'s>> {
    literal(Token::Module)
        .span()
        .map(|s| {
            ModuleKeyword::Module(Metadata {
                span: s,
                extra_nodes: vec![],
            })
        })
        .parse_next(input)
}
