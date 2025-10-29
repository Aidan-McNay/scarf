// =======================================================================
// looping_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.8

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt};

pub fn loop_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LoopStatement<'s>, VerboseError<'s>> {
    let _forever_parser = (token(Token::Forever), statement_or_null_parser)
        .map(|(a, b)| LoopStatement::Forever(Box::new((a, b))));
    let _repeat_parser = (
        token(Token::Repeat),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e)| {
            LoopStatement::Repeat(Box::new((a, b, c, d, e)))
        });
    let _while_parser = (
        token(Token::While),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e)| LoopStatement::While(Box::new((a, b, c, d, e))));
    let _for_parser = (
        token(Token::For),
        token(Token::Paren),
        opt(for_initialization_parser),
        token(Token::SColon),
        opt(expression_parser),
        token(Token::SColon),
        opt(for_step_parser),
        token(Token::EParen),
        statement_or_null_parser,
    )
        .map(|(a, b, c, d, e, f, g, h, i)| {
            LoopStatement::For(Box::new((a, b, c, d, e, f, g, h, i)))
        });
    let _do_while_parser = (
        token(Token::Do),
        statement_or_null_parser,
        token(Token::While),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f, g)| {
            LoopStatement::DoWhile(Box::new((a, b, c, d, e, f, g)))
        });
    let _foreach_parser = (
        token(Token::Foreach),
        token(Token::Paren),
        ps_or_hierarchical_array_identifier_parser,
        token(Token::Bracket),
        loop_variables_parser,
        token(Token::EBracket),
        token(Token::EParen),
        statement_parser,
    )
        .map(|(a, b, c, d, e, f, g, h)| {
            LoopStatement::Foreach(Box::new((a, b, c, d, e, f, g, h)))
        });
    alt((
        _forever_parser,
        _repeat_parser,
        _while_parser,
        _for_parser,
        _do_while_parser,
        _foreach_parser,
    ))
    .parse_next(input)
}

pub fn for_initialization_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForInitialization<'s>, VerboseError<'s>> {
    alt((
        list_of_variable_assignments_parser
            .map(|a| ForInitialization::VariableAssignment(Box::new(a))),
        (
            for_variable_declaration_parser,
            repeat_strict((token(Token::Comma), for_variable_declaration_parser)),
        )
            .map(|(a, b)| {
                ForInitialization::VariableDeclarations(Box::new((a, b)))
            }),
    ))
    .parse_next(input)
}

pub fn for_variable_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForVariableDeclaration<'s>, VerboseError<'s>> {
    (
        opt(token(Token::Var)),
        data_type_parser,
        variable_identifier_parser,
        token(Token::Eq),
        expression_parser,
        repeat_strict((
            token(Token::Comma),
            variable_identifier_parser,
            token(Token::Eq),
            expression_parser,
        )),
    )
        .map(|(a, b, c, d, e, f)| ForVariableDeclaration(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn for_step_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForStep<'s>, VerboseError<'s>> {
    (
        for_step_assignment_parser,
        repeat_strict((token(Token::Comma), for_step_assignment_parser)),
    )
        .map(|(a, b)| ForStep(a, b))
        .parse_next(input)
}

pub fn for_step_assignment_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ForStepAssignment<'s>, VerboseError<'s>> {
    alt((
        operator_assignment_parser
            .map(|a| ForStepAssignment::Operator(Box::new(a))),
        inc_or_dec_expression_parser
            .map(|a| ForStepAssignment::IncOrDec(Box::new(a))),
        function_subroutine_call_parser
            .map(|a| ForStepAssignment::Function(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn loop_variables_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<LoopVariables<'s>, VerboseError<'s>> {
    (
        opt(index_variable_identifier_parser),
        repeat_strict((
            token(Token::Comma),
            opt(index_variable_identifier_parser),
        )),
    )
        .map(|(a, b)| LoopVariables(a, b))
        .parse_next(input)
}
