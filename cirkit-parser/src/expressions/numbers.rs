// =======================================================================
// numbers.rs
// =======================================================================
// Parsing for 1800-2023 A.8.7

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn fixed_point_number_parser<'a>() -> impl Parser<'a, &'a str, FixedPointNumber, ParserError<'a>>
{
    unsigned_number_parser()
        .then_ignore(just('.'))
        .then(unsigned_number_parser())
        .map(|(a, b)| FixedPointNumber(a, b))
}

pub fn unsigned_number_parser<'a>() -> impl Parser<'a, &'a str, UnsignedNumber, ParserError<'a>> {
    let decimal_digit_parser = one_of("0123456789").map(String::from);
    decimal_digit_parser.clone().foldl(
        choice((decimal_digit_parser, just('_').map(String::from))).repeated(),
        |a, b| a + &b,
    )
}
