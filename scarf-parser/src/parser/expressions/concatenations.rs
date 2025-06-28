// =======================================================================
// concatenations.rs
// =======================================================================
// Parsing for 1800-2023 A.8.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn concatenation_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, Concatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(expression_parser.clone())
        .then(
            token(Token::Comma)
                .then(expression_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, Expression<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| Concatenation(a, b, c, d))
        .boxed()
}

pub fn constant_concatenation_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantConcatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(constant_expression_parser.clone())
        .then(
            token(Token::Comma)
                .then(constant_expression_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, ConstantExpression<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ConstantConcatenation(a, b, c, d))
        .boxed()
}

pub fn constant_multiple_concatenation_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantMultipleConcatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(constant_expression_parser.clone())
        .then(constant_concatenation_parser(constant_expression_parser))
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ConstantMultipleConcatenation(a, b, c, d))
        .boxed()
}

pub fn module_path_concatenation_parser<'a>(
    module_path_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ModulePathExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ModulePathConcatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(module_path_expression_parser.clone())
        .then(
            token(Token::Comma)
                .then(module_path_expression_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, ModulePathExpression<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ModulePathConcatenation(a, b, c, d))
        .boxed()
}

pub fn module_path_multiple_concatenation_parser<'a>(
    module_path_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ModulePathExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ModulePathMultipleConcatenation<'a>, ParserError<'a>> + Clone
{
    token(Token::Brace)
        .then(constant_expression_parser(expression_parser()))
        .then(module_path_concatenation_parser(
            module_path_expression_parser,
        ))
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| ModulePathMultipleConcatenation(a, b, c, d))
        .boxed()
}

pub fn multiple_concatenation_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, MultipleConcatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(expression_parser.clone())
        .then(concatenation_parser(expression_parser))
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| MultipleConcatenation(a, b, c, d))
        .boxed()
}

pub fn streaming_concatenation_parser<'a>(
    _expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, StreamingConcatenation<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn stream_operator_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, StreamOperator<'a>, ParserError<'a>> + Clone {
    choice((
        token(Token::GtGt).map(|a| StreamOperator::Right(a)),
        token(Token::LtLt).map(|a| StreamOperator::Left(a)),
    ))
}

pub fn empty_unpacked_array_concatenation_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, EmptyUnpackedArrayConcatenation<'a>, ParserError<'a>> + Clone {
    token(Token::Brace)
        .then(token(Token::EBrace))
        .map(|(a, b)| EmptyUnpackedArrayConcatenation(a, b))
}
