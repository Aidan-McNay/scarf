// =======================================================================
// concatenations.rs
// =======================================================================
// Parsing for 1800-2023 A.8.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, fail, repeat};

pub fn concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Concatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        expression_parser,
        repeat(0.., (token(Token::Comma), expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| Concatenation(a, b, c, d))
        .parse_next(input)
}

pub fn constant_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        repeat(0.., (token(Token::Comma), constant_expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ConstantConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn constant_multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantMultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        constant_concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ConstantMultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn module_path_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        module_path_expression_parser,
        repeat(0.., (token(Token::Comma), module_path_expression_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ModulePathConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn module_path_multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathMultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        constant_expression_parser,
        module_path_concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| ModulePathMultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn multiple_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MultipleConcatenation<'s>, VerboseError<'s>> {
    (
        token(Token::Brace),
        expression_parser,
        concatenation_parser,
        token(Token::EBrace),
    )
        .map(|(a, b, c, d)| MultipleConcatenation(a, b, c, d))
        .parse_next(input)
}

pub fn streaming_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamingConcatenation<'s>, VerboseError<'s>> {
    fail.parse_next(input)
}

pub fn stream_operator_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StreamOperator<'s>, VerboseError<'s>> {
    alt((
        token(Token::GtGt).map(|a| StreamOperator::Right(a)),
        token(Token::LtLt).map(|a| StreamOperator::Left(a)),
    ))
    .parse_next(input)
}

pub fn empty_unpacked_array_concatenation_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EmptyUnpackedArrayConcatenation<'s>, VerboseError<'s>> {
    (token(Token::Brace), token(Token::EBrace))
        .map(|(a, b)| EmptyUnpackedArrayConcatenation(a, b))
        .parse_next(input)
}
