// =======================================================================
// patterns.rs
// =======================================================================
// Parsing for 1800-2023 A.6.7.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn pattern_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, Pattern<'a>, ParserError<'a>> + Clone {
    let mut parser = Recursive::declare();
    let _parentheses_parser = token(Token::Paren)
        .then(parser.clone())
        .then(token(Token::EParen))
        .map(|((a, b), c)| Pattern::Parentheses(Box::new((a, b, c))));
    let _variable_identifier_parser = token(Token::Period)
        .then(variable_identifier_parser())
        .map(|(a, b)| Pattern::VariableIdentifier(Box::new((a, b))));
    let _wildcard_parser = token(Token::Period)
        .then(token(Token::Star))
        .map(|(a, b)| Pattern::Wildcard(Box::new((a, b))));
    let _constant_expression_parser = constant_expression_parser(expression_parser)
        .map(|a| Pattern::ConstantExpression(Box::new(a)));
    let _tagged_member_parser = token(Token::Tagged)
        .then(member_identifier_parser())
        .then(parser.clone().or_not())
        .map(|((a, b), c)| Pattern::TaggedMember(Box::new((a, b, c))));
    let _multi_pattern_parser = token(Token::Apost)
        .then(token(Token::Brace))
        .then(parser.clone())
        .then(
            token(Token::Apost)
                .then(parser.clone())
                .repeated()
                .collect::<Vec<(Metadata<'a>, Pattern<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|((((a, b), c), d), e)| Pattern::MultiPattern(Box::new((a, b, c, d, e))));
    let _multi_identifier_pattern_parser = token(Token::Apost)
        .then(token(Token::Brace))
        .then(member_identifier_parser())
        .then(token(Token::Colon))
        .then(parser.clone())
        .then(
            token(Token::Apost)
                .then(member_identifier_parser())
                .then(token(Token::Colon))
                .then(parser.clone())
                .map(|(((a, b), c), d)| (a, b, c, d))
                .repeated()
                .collect::<Vec<(
                    Metadata<'a>,
                    MemberIdentifier<'a>,
                    Metadata<'a>,
                    Pattern<'a>,
                )>>(),
        )
        .then(token(Token::EBrace))
        .map(|((((((a, b), c), d), e), f), g)| {
            Pattern::MultiIdentifierPattern(Box::new((a, b, c, d, e, f, g)))
        });
    parser.define(choice((
        _parentheses_parser,
        _variable_identifier_parser,
        _wildcard_parser,
        _constant_expression_parser,
        _tagged_member_parser,
        _multi_pattern_parser,
        _multi_identifier_pattern_parser,
    )));
    parser.boxed()
}

pub fn assignment_pattern_expression_parser<'a>(
    _expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, AssignmentPatternExpression<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn assignment_pattern_expression_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, AssignmentPatternExpressionType<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn constant_assignment_pattern_expression_parser<'a>(
    _constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantAssignmentPatternExpression<'a>, ParserError<'a>> + Clone
{
    todo_parser()
}

pub fn assignment_pattern_net_lvalue_parser<'a>(
    net_lvalue_parser: impl Parser<'a, ParserInput<'a>, NetLvalue<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, AssignmentPatternNetLvalue<'a>, ParserError<'a>> + Clone {
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
pub fn assignment_pattern_variable_lvalue_parser<'a>(
    variable_lvalue_parser: impl Parser<'a, ParserInput<'a>, VariableLvalue<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, AssignmentPatternVariableLvalue<'a>, ParserError<'a>> + Clone
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
