// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use cirkit_syntax::*;

pub fn time_literal_parser<'a, I>() -> impl Parser<'a, I, TimeLiteral<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::TimeLiteral((val, unit)) => {let unit_enum = match unit {
            "s" => TimeUnit::S,
            "ms" => TimeUnit::MS,
            "us" => TimeUnit::US,
            "ns" => TimeUnit::NS,
            "ps" => TimeUnit::PS,
            "fs" => TimeUnit::FS,
            _ => panic!("Invalid time unit from lexing")
        };
        if val.contains(".") {
            let components: Vec<&'a str> = val.split(".").collect();
            TimeLiteral::TimeLiteralFixedPoint(Box::new((FixedPointNumber(components[0], components[1]), unit_enum)))
        } else {
            TimeLiteral::TimeLiteralUnsigned(Box::new((val, unit_enum)))
        }
    }
    }
}
