// =======================================================================
// strengths.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.2

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn drive_strength_parser<'a, I>()
-> impl Parser<'a, I, DriveStrength<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let s0s1_parser = token(Token::Paren)
        .then(strength0_parser())
        .then(token(Token::Comma))
        .then(strength1_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::S0S1(a, b, c, d, e));
    let s1s0_parser = token(Token::Paren)
        .then(strength1_parser())
        .then(token(Token::Comma))
        .then(strength0_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::S1S0(a, b, c, d, e));
    let s0z1_parser = token(Token::Paren)
        .then(strength0_parser())
        .then(token(Token::Comma))
        .then(token(Token::Highz1))
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::S0Z1(a, b, c, d, e));
    let s1z0_parser = token(Token::Paren)
        .then(strength1_parser())
        .then(token(Token::Comma))
        .then(token(Token::Highz0))
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::S1Z0(a, b, c, d, e));
    let z0s1_parser = token(Token::Paren)
        .then(token(Token::Highz0))
        .then(token(Token::Comma))
        .then(strength1_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::Z0S1(a, b, c, d, e));
    let z1s0_parser = token(Token::Paren)
        .then(token(Token::Highz1))
        .then(token(Token::Comma))
        .then(strength0_parser())
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| DriveStrength::Z1S0(a, b, c, d, e));
    choice((
        s0s1_parser,
        s1s0_parser,
        s0z1_parser,
        s1z0_parser,
        z0s1_parser,
        z1s0_parser,
    ))
}

pub fn strength0_parser<'a, I>() -> impl Parser<'a, I, Strength0<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Supply0).map(|a| Strength0::Supply0(a)),
        token(Token::Strong0).map(|a| Strength0::Strong0(a)),
        token(Token::Pull0).map(|a| Strength0::Pull0(a)),
        token(Token::Weak0).map(|a| Strength0::Weak0(a)),
    ))
}

pub fn strength1_parser<'a, I>() -> impl Parser<'a, I, Strength1<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Supply1).map(|a| Strength1::Supply1(a)),
        token(Token::Strong1).map(|a| Strength1::Strong1(a)),
        token(Token::Pull1).map(|a| Strength1::Pull1(a)),
        token(Token::Weak1).map(|a| Strength1::Weak1(a)),
    ))
}

pub fn charge_strength_parser<'a, I>()
-> impl Parser<'a, I, ChargeStrength<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let charge_strength_size_parser = choice((
        token(Token::Small).map(|a| ChargeStrengthSize::Small(a)),
        token(Token::Medium).map(|a| ChargeStrengthSize::Medium(a)),
        token(Token::Large).map(|a| ChargeStrengthSize::Large(a)),
    ));
    token(Token::Paren)
        .then(charge_strength_size_parser)
        .then(token(Token::EParen))
        .map(|((a, b), c)| ChargeStrength(a, b, c))
}
