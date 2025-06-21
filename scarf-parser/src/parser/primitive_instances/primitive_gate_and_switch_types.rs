// =======================================================================
// primitive_gate_and_switch_types.rs
// =======================================================================
// Parsing for 1800-2023 A.3.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn cmos_switchtype_parser<'a, I>()
-> impl Parser<'a, I, CmosSwitchtype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Cmos).map(|a| CmosSwitchtype::Cmos(a)),
        token(Token::Rcmos).map(|a| CmosSwitchtype::Rcmos(a)),
    ))
    .boxed()
}

pub fn enable_gatetype_parser<'a, I>()
-> impl Parser<'a, I, EnableGatetype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Bufif0).map(|a| EnableGatetype::Bufif0(a)),
        token(Token::Bufif1).map(|a| EnableGatetype::Bufif1(a)),
        token(Token::Notif0).map(|a| EnableGatetype::Notif0(a)),
        token(Token::Notif1).map(|a| EnableGatetype::Notif1(a)),
    ))
    .boxed()
}

pub fn mos_switchtype_parser<'a, I>()
-> impl Parser<'a, I, MosSwitchtype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Nmos).map(|a| MosSwitchtype::Nmos(a)),
        token(Token::Pmos).map(|a| MosSwitchtype::Pmos(a)),
        token(Token::Rnmos).map(|a| MosSwitchtype::Rnmos(a)),
        token(Token::Rpmos).map(|a| MosSwitchtype::Rpmos(a)),
    ))
    .boxed()
}

pub fn n_input_gatetype_parser<'a, I>()
-> impl Parser<'a, I, NInputGatetype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::And).map(|a| NInputGatetype::And(a)),
        token(Token::Nand).map(|a| NInputGatetype::Nand(a)),
        token(Token::Or).map(|a| NInputGatetype::Or(a)),
        token(Token::Nor).map(|a| NInputGatetype::Nor(a)),
        token(Token::Xor).map(|a| NInputGatetype::Xor(a)),
        token(Token::Xnor).map(|a| NInputGatetype::Xnor(a)),
    ))
    .boxed()
}

pub fn n_output_gatetype_parser<'a, I>()
-> impl Parser<'a, I, NOutputGatetype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Buf).map(|a| NOutputGatetype::Buf(a)),
        token(Token::Not).map(|a| NOutputGatetype::Not(a)),
    ))
    .boxed()
}

pub fn pass_en_switchtype_parser<'a, I>()
-> impl Parser<'a, I, PassEnSwitchtype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Tranif0).map(|a| PassEnSwitchtype::Tranif0(a)),
        token(Token::Tranif1).map(|a| PassEnSwitchtype::Tranif1(a)),
        token(Token::Rtranif0).map(|a| PassEnSwitchtype::Rtranif0(a)),
        token(Token::Rtranif1).map(|a| PassEnSwitchtype::Rtranif1(a)),
    ))
    .boxed()
}

pub fn pass_switchtype_parser<'a, I>()
-> impl Parser<'a, I, PassSwitchtype<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Tran).map(|a| PassSwitchtype::Tran(a)),
        token(Token::Rtran).map(|a| PassSwitchtype::Rtran(a)),
    ))
    .boxed()
}
