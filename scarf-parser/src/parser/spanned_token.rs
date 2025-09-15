// =======================================================================
// spanned_token.rs
// =======================================================================
// A token with an associated span, to be used in parsing

use crate::*;
use lexer::{Span, Token};
use winnow::ModalResult;
use winnow::Parser;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::stream::TokenSlice;
use winnow::token::literal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken<'s>(pub Token<'s>, pub Span);
impl<'s> PartialEq<Token<'s>> for SpannedToken<'s> {
    fn eq(&self, other: &Token) -> bool {
        self.0 == *other
    }
}
impl<'s> From<(Token<'s>, Span)> for SpannedToken<'s> {
    fn from(item: (Token<'s>, Span)) -> Self {
        (item.0, item.1).into()
    }
}

pub type Tokens<'s> = TokenSlice<'s, SpannedToken<'s>>;
impl<'s> Parser<Tokens<'s>, &'s SpannedToken<'s>, ErrMode<ContextError>>
    for Token<'s>
{
    fn parse_next(
        &mut self,
        input: &mut Tokens<'s>,
    ) -> ModalResult<&'s SpannedToken<'s>> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}
