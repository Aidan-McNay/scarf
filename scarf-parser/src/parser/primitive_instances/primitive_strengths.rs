// =======================================================================
// primitive_strengths.rs
// =======================================================================
// Parsing for 1800-2023 A.3.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn pulldown_strength_parser<'a, I>()
-> impl Parser<'a, I, PulldownStrength<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _s0s1_parser = token(Token::Paren)
        .then(strength0_parser())
        .then(token(Token::Comma))
        .then(strength1_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| PulldownStrength::S0S1(Box::new((a, b, c, d, e))));
    let _s1s0_parser = token(Token::Paren)
        .then(strength1_parser())
        .then(token(Token::Comma))
        .then(strength0_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| PulldownStrength::S1S0(Box::new((a, b, c, d, e))));
    let _s0_parser = token(Token::Paren)
        .then(strength0_parser())
        .then(token(Token::EParen))
        .map(|((a, b), c)| PulldownStrength::S0(Box::new((a, b, c))));
    choice((_s0s1_parser, _s1s0_parser, _s0_parser)).boxed()
}

pub fn pullup_strength_parser<'a, I>()
-> impl Parser<'a, I, PullupStrength<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let _s0s1_parser = token(Token::Paren)
        .then(strength0_parser())
        .then(token(Token::Comma))
        .then(strength1_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| PullupStrength::S0S1(Box::new((a, b, c, d, e))));
    let _s1s0_parser = token(Token::Paren)
        .then(strength1_parser())
        .then(token(Token::Comma))
        .then(strength0_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| PullupStrength::S1S0(Box::new((a, b, c, d, e))));
    let _s1_parser = token(Token::Paren)
        .then(strength1_parser())
        .then(token(Token::EParen))
        .map(|((a, b), c)| PullupStrength::S1(Box::new((a, b, c))));
    choice((_s0s1_parser, _s1s0_parser, _s1_parser)).boxed()
}
