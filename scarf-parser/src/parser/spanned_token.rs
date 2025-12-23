// =======================================================================
// spanned_token.rs
// =======================================================================
// A token with an associated span, to be used in parsing

use crate::*;
use lexer::Token;
use winnow::ModalResult;
use winnow::Parser;
use winnow::error::ErrMode;
use winnow::stream::{Stateful, TokenSlice};
use winnow::token::literal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken<'s>(pub Token<'s>, pub Span<'s>);
impl<'s> PartialEq<Token<'s>> for SpannedToken<'s> {
    fn eq(&self, other: &Token) -> bool {
        self.0 == *other
    }
}
impl<'s> From<(Token<'s>, Span<'s>)> for SpannedToken<'s> {
    fn from(item: (Token<'s>, Span<'s>)) -> Self {
        (item.0, item.1).into()
    }
}

// Keep track of the largest error we've seen in repeat/opt branches
pub type Tokens<'s> =
    Stateful<TokenSlice<'s, SpannedToken<'s>>, VerboseError<'s>>;
impl<'s> Parser<Tokens<'s>, &'s SpannedToken<'s>, ErrMode<VerboseError<'s>>>
    for Token<'s>
{
    fn parse_next(
        &mut self,
        input: &mut Tokens<'s>,
    ) -> ModalResult<&'s SpannedToken<'s>, VerboseError<'s>> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}
