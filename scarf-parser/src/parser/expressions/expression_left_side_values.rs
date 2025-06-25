// =======================================================================
// expression_left_side_values.rs
// =======================================================================
// Parsing for 1800-2023 A.8.5

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn net_lvalue_parser<'a, I>() -> impl Parser<'a, I, NetLvalue<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let mut parser = Recursive::declare();
    let _selection_net_lvalue_parser = ps_or_hierarchical_net_identifier_parser()
        .then(constant_select_parser(constant_expression_parser()))
        .map(|(a, b)| NetLvalue::Selection(Box::new(SelectionNetLvalue(a, b))));
    let _nested_net_lvalue_parser = token(Token::Brace)
        .then(parser.clone())
        .then(
            token(Token::Comma)
                .then(parser.clone())
                .repeated()
                .collect::<Vec<(Metadata<'a>, NetLvalue<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| NetLvalue::Nested(Box::new(NestedNetLvalue(a, b, c, d))));
    let _assignment_net_lvalue_parser = assignment_pattern_expression_type_parser()
        .or_not()
        .then(assignment_pattern_net_lvalue_parser(parser.clone()))
        .map(|(a, b)| NetLvalue::Assignment(Box::new(AssignmentNetLvalue(a, b))));
    parser.define(choice((
        _selection_net_lvalue_parser,
        _nested_net_lvalue_parser,
        _assignment_net_lvalue_parser,
    )));
    parser.boxed()
}

fn implicit_class_handle_or_package_scope_parser<'a, I>()
-> impl Parser<'a, I, ImplicitClassHandleOrPackageScope<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        implicit_class_handle_parser()
            .then(token(Token::Period))
            .map(|(a, b)| ImplicitClassHandleOrPackageScope::ImplicitClassHandle(Box::new((a, b)))),
        package_scope_parser()
            .map(|a| ImplicitClassHandleOrPackageScope::PackageScope(Box::new(a))),
    ))
    .boxed()
}

pub fn variable_lvalue_parser<'a, I>(
    expression_parser: impl Parser<'a, I, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, I, VariableLvalue<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    let mut parser = Recursive::declare();
    let _selection_variable_lvalue_parser = implicit_class_handle_or_package_scope_parser()
        .then(hierarchical_variable_identifier_parser())
        .then(select_parser())
        .map(|((a, b), c)| VariableLvalue::Selection(Box::new(SelectionVariableLvalue(a, b, c))));
    let _nested_variable_lvalue_parser = token(Token::Brace)
        .then(parser.clone())
        .then(
            token(Token::Comma)
                .then(parser.clone())
                .repeated()
                .collect::<Vec<(Metadata<'a>, VariableLvalue<'a>)>>(),
        )
        .then(token(Token::EBrace))
        .map(|(((a, b), c), d)| VariableLvalue::Nested(Box::new(NestedVariableLvalue(a, b, c, d))));
    let _assignment_variable_lvalue_parser = assignment_pattern_expression_type_parser()
        .or_not()
        .then(assignment_pattern_variable_lvalue_parser(parser.clone()))
        .map(|(a, b)| VariableLvalue::Assignment(Box::new(AssignmentVariableLvalue(a, b))));
    let _streaming_variable_lvalue_parser = streaming_concatenation_parser(expression_parser)
        .map(|a| VariableLvalue::Streaming(Box::new(a)));
    parser.define(choice((
        _selection_variable_lvalue_parser,
        _nested_variable_lvalue_parser,
        _assignment_variable_lvalue_parser,
        _streaming_variable_lvalue_parser,
    )));
    parser.boxed()
}

pub fn nonrange_variable_lvalue_parser<'a, I>()
-> impl Parser<'a, I, NonrangeVariableLvalue<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    implicit_class_handle_or_package_scope_parser()
        .then(hierarchical_variable_identifier_parser())
        .then(nonrange_select_parser())
        .map(|((a, b), c)| NonrangeVariableLvalue(a, b, c))
        .boxed()
}
