// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.6

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn cond_predicate_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, CondPredicate<'a>, ParserError<'a>> + Clone {
    expression_or_cond_pattern_parser(expression_parser.clone())
        .then(
            token(Token::AmpAmpAmp)
                .then(expression_or_cond_pattern_parser(expression_parser))
                .repeated()
                .collect::<Vec<(Metadata<'a>, ExpressionOrCondPattern<'a>)>>(),
        )
        .map(|(a, b)| CondPredicate(a, b))
        .boxed()
}

pub fn expression_or_cond_pattern_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, ExpressionOrCondPattern<'a>, ParserError<'a>> + Clone {
    choice((
        expression_parser
            .clone()
            .map(|a| ExpressionOrCondPattern::Expression(Box::new(a))),
        cond_pattern_parser(expression_parser)
            .map(|a| ExpressionOrCondPattern::CondPattern(Box::new(a))),
    ))
    .boxed()
}

pub fn cond_pattern_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, CondPattern<'a>, ParserError<'a>> + Clone {
    expression_parser
        .clone()
        .then(token(Token::Matches))
        .then(pattern_parser(expression_parser))
        .map(|((a, b), c)| CondPattern(a, b, c))
        .boxed()
}
