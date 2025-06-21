// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn assignment_pattern_expression_type_parser<'a, I>()
-> impl Parser<'a, I, AssignmentPatternExpressionType<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn assignment_pattern_net_lvalue_parser<'a, I>(
    net_lvalue_parser: impl Parser<'a, I, NetLvalue<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, AssignmentPatternNetLvalue<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Apost)
        .then(token(Token::Brace))
        .then(net_lvalue_parser.clone())
        .then(
            token(Token::Comma)
                .then(net_lvalue_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, NetLvalue<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|((((a, b), c), d), e)| AssignmentPatternNetLvalue(a, b, c, d, e))
        .boxed()
}

// Inlined for variable_lvalue_parser
pub fn assignment_pattern_variable_lvalue_parser<'a, I>(
    variable_lvalue_parser: impl Parser<'a, I, VariableLvalue<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, AssignmentPatternVariableLvalue<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    token(Token::Apost)
        .then(token(Token::Brace))
        .then(variable_lvalue_parser.clone())
        .then(
            token(Token::Comma)
                .then(variable_lvalue_parser)
                .repeated()
                .collect::<Vec<(Metadata<'a>, VariableLvalue<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|((((a, b), c), d), e)| AssignmentPatternVariableLvalue(a, b, c, d, e))
        .boxed()
}
