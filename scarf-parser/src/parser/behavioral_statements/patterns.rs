// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt, repeat, todo};

pub fn pattern_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Pattern<'s>, VerboseError<'s>> {
    let _parentheses_parser =
        (token(Token::Paren), pattern_parser, token(Token::EParen))
            .map(|(a, b, c)| Pattern::Parentheses(Box::new((a, b, c))));
    let _variable_identifier_parser =
        (token(Token::Period), variable_identifier_parser)
            .map(|(a, b)| Pattern::VariableIdentifier(Box::new((a, b))));
    let _wildcard_parser = (token(Token::Period), token(Token::Star))
        .map(|(a, b)| Pattern::Wildcard(Box::new((a, b))));
    let _constant_expression_parser = constant_expression_parser
        .map(|a| Pattern::ConstantExpression(Box::new(a)));
    let _tagged_member_parser = (
        token(Token::Tagged),
        member_identifier_parser,
        opt(pattern_parser),
    )
        .map(|(a, b, c)| Pattern::TaggedMember(Box::new((a, b, c))));
    let _multi_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        pattern_parser,
        repeat(0.., (token(Token::Apost), pattern_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| {
            Pattern::MultiPattern(Box::new((a, b, c, d, e)))
        });
    let _multi_identifier_pattern_parser = (
        token(Token::Apost),
        token(Token::Brace),
        member_identifier_parser,
        token(Token::Colon),
        pattern_parser,
        repeat(
            0..,
            (
                token(Token::Apost),
                member_identifier_parser,
                token(Token::Colon),
                pattern_parser,
            ),
        ),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e, f, g)| {
            Pattern::MultiIdentifierPattern(Box::new((a, b, c, d, e, f, g)))
        });
    alt((
        _parentheses_parser,
        _variable_identifier_parser,
        _wildcard_parser,
        _constant_expression_parser,
        _tagged_member_parser,
        _multi_pattern_parser,
        _multi_identifier_pattern_parser,
    ))
    .parse_next(input)
}

pub fn assignment_pattern_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternExpression<'s>, VerboseError<'s>> {
    todo(input)
}

pub fn assignment_pattern_expression_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternExpressionType<'s>, VerboseError<'s>> {
    todo(input)
}

pub fn constant_assignment_pattern_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantAssignmentPatternExpression<'s>, VerboseError<'s>> {
    todo(input)
}

pub fn assignment_pattern_net_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternNetLvalue<'s>, VerboseError<'s>> {
    (
        token(Token::Apost),
        token(Token::Brace),
        net_lvalue_parser,
        repeat(0.., (token(Token::Comma), net_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| AssignmentPatternNetLvalue(a, b, c, d, e))
        .parse_next(input)
}

// Inlined for variable_lvalue_parser
pub fn assignment_pattern_variable_lvalue_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<AssignmentPatternVariableLvalue<'s>, VerboseError<'s>> {
    (
        token(Token::Apost),
        token(Token::Brace),
        variable_lvalue_parser,
        repeat(0.., (token(Token::Comma), variable_lvalue_parser)),
        token(Token::EBrace),
    )
        .map(|(a, b, c, d, e)| AssignmentPatternVariableLvalue(a, b, c, d, e))
        .parse_next(input)
}
