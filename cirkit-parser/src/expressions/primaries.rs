// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn time_literal_parser<'a>() -> impl Parser<'a, &'a str, TimeLiteral, ParserError<'a>> {
    let time_literal_unsigned_parser = unsigned_number_parser()
        .then_ignore(text::whitespace())
        .then(time_unit_parser())
        .map(|(a, b)| TimeLiteral::TimeLiteralUnsigned(Box::new((a, b))));
    let time_literal_fixed_point_parser = fixed_point_number_parser()
        .then_ignore(text::whitespace())
        .then(time_unit_parser())
        .map(|(a, b)| TimeLiteral::TimeLiteralFixedPoint(Box::new((a, b))));
    choice((
        time_literal_unsigned_parser,
        time_literal_fixed_point_parser,
    ))
}

pub fn time_unit_parser<'a>() -> impl Parser<'a, &'a str, TimeUnit, ParserError<'a>> {
    choice((
        just('s').to(TimeUnit::S),
        just("ms").to(TimeUnit::MS),
        just("us").to(TimeUnit::US),
        just("ns").to(TimeUnit::NS),
        just("ps").to(TimeUnit::PS),
        just("fs").to(TimeUnit::FS),
    ))
}
