// =======================================================================
// task_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.7

use crate::*;
use scarf_syntax::*;
use winnow::Parser;
use winnow::combinator::{alt, opt, repeat};
use winnow::error::ModalResult;

pub fn task_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaskDeclaration<'s>, VerboseError<'s>> {
    (
        token(Token::Task),
        opt_dynamic_override_specifiers_parser,
        opt(lifetime_parser),
        task_body_declaration_parser,
    )
        .map(|(a, b, c, d)| TaskDeclaration(a, b, c, d))
        .parse_next(input)
}

pub fn interface_identifier_or_class_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceIdentifierOrClassScope<'s>, VerboseError<'s>> {
    alt((
        (interface_identifier_parser, token(Token::Period)).map(|(a, b)| {
            InterfaceIdentifierOrClassScope::InterfaceIdentifier(Box::new((
                a, b,
            )))
        }),
        class_scope_parser
            .map(|a| InterfaceIdentifierOrClassScope::ClassScope(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn task_body_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaskBodyDeclaration<'s>, VerboseError<'s>> {
    let _tf_parser = (
        interface_identifier_or_class_scope_parser,
        task_identifier_parser,
        token(Token::SColon),
        repeat(0.., tf_item_declaration_parser),
        repeat(0.., statement_or_null_parser),
        token(Token::Endtask),
        opt((token(Token::Colon), task_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g)| {
            TaskBodyDeclaration::Tf(Box::new((a, b, c, d, e, f, g)))
        });
    let _block_parser = (
        interface_identifier_or_class_scope_parser,
        task_identifier_parser,
        token(Token::Paren),
        opt(tf_port_list_parser),
        token(Token::EParen),
        token(Token::SColon),
        repeat(0.., block_item_declaration_parser),
        repeat(0.., statement_or_null_parser),
        token(Token::Endtask),
        opt((token(Token::Colon), task_identifier_parser)),
    )
        .map(|(a, b, c, d, e, f, g, h, i, j)| {
            TaskBodyDeclaration::Block(Box::new((a, b, c, d, e, f, g, h, i, j)))
        });
    alt((_tf_parser, _block_parser)).parse_next(input)
}

pub fn tf_item_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfItemDeclaration<'s>, VerboseError<'s>> {
    alt((
        block_item_declaration_parser
            .map(|a| TfItemDeclaration::Block(Box::new(a))),
        tf_port_declaration_parser.map(|a| TfItemDeclaration::Tf(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn tf_port_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfPortList<'s>, VerboseError<'s>> {
    (
        tf_port_item_parser,
        repeat(0.., (token(Token::Comma), tf_port_item_parser)),
    )
        .map(|(a, b)| TfPortList(a, b))
        .parse_next(input)
}

pub fn tf_port_item_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfPortItem<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt(tf_port_direction_parser),
        opt(token(Token::Var)),
        data_type_or_implicit_parser,
        opt((
            port_identifier_parser,
            repeat(0.., variable_dimension_parser),
            opt((token(Token::Eq), expression_parser)),
        )),
    )
        .map(|(a, b, c, d, e)| TfPortItem(a, b, c, d, e))
        .parse_next(input)
}

pub fn tf_port_direction_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfPortDirection<'s>, VerboseError<'s>> {
    alt((
        port_direction_parser.map(|a| TfPortDirection::Port(Box::new(a))),
        (
            opt(token(Token::Const)),
            token(Token::Ref),
            opt(token(Token::Static)),
        )
            .map(|(a, b, c)| TfPortDirection::Ref(Box::new((a, b, c)))),
    ))
    .parse_next(input)
}

pub fn tf_port_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfPortDeclaration<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        tf_port_direction_parser,
        opt(token(Token::Var)),
        data_type_or_implicit_parser,
        list_of_tf_variable_identifiers_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e, f)| TfPortDeclaration(a, b, c, d, e, f))
        .parse_next(input)
}

pub fn task_prototype_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TaskPrototype<'s>, VerboseError<'s>> {
    (
        token(Token::Task),
        opt_dynamic_override_specifiers_parser,
        task_identifier_parser,
        opt((
            token(Token::Paren),
            opt(tf_port_list_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d)| TaskPrototype(a, b, c, d))
        .parse_next(input)
}

pub fn opt_dynamic_override_specifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Option<DynamicOverrideSpecifiers<'s>>, VerboseError<'s>> {
    opt(dynamic_override_specifiers_parser)
        .map(|a| match a {
            Some(DynamicOverrideSpecifiers(None, None)) => None,
            other_expr => other_expr,
        })
        .parse_next(input)
}

pub fn dynamic_override_specifiers_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DynamicOverrideSpecifiers<'s>, VerboseError<'s>> {
    (
        opt(initial_or_extends_specifier_parser),
        opt(final_specifier_parser),
    )
        .map(|(a, b)| DynamicOverrideSpecifiers(a, b))
        .parse_next(input)
}

pub fn initial_or_extends_specifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InitialOrExtendsSpecifier<'s>, VerboseError<'s>> {
    alt((
        (token(Token::Colon), token(Token::Initial))
            .map(|(a, b)| InitialOrExtendsSpecifier::Initial((a, b))),
        (token(Token::Colon), token(Token::Extends))
            .map(|(a, b)| InitialOrExtendsSpecifier::Extends((a, b))),
    ))
    .parse_next(input)
}

pub fn final_specifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FinalSpecifier<'s>, VerboseError<'s>> {
    (token(Token::Colon), token(Token::Final))
        .map(|(a, b)| FinalSpecifier(a, b))
        .parse_next(input)
}
