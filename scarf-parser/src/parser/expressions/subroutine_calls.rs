// =======================================================================
// subroutine_calls.rs
// =======================================================================
// Parsing for 1800-2023 A.8.2

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::combinator::{alt, opt};

pub fn constant_function_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantFunctionCall<'s>, VerboseError<'s>> {
    function_subroutine_call_parser
        .map(|a| ConstantFunctionCall(a))
        .parse_next(input)
}

pub fn tf_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TfCall<'s>, VerboseError<'s>> {
    (
        ps_or_hierarchical_tf_identifier_parser,
        attribute_instance_vec_parser,
        opt((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| TfCall(a, b, c))
        .parse_next(input)
}

pub fn system_tf_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SystemTfCall<'s>, VerboseError<'s>> {
    let _args_parser = (
        system_tf_identifier_parser,
        opt((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b)| SystemTfCall::Args(Box::new((a, b))));
    let _data_parser = (
        system_tf_identifier_parser,
        token(Token::Paren),
        data_type_parser,
        opt((token(Token::Comma), expression_parser)),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| SystemTfCall::Data(Box::new((a, b, c, d, e))));
    let _expressions_parser = (
        system_tf_identifier_parser,
        token(Token::Paren),
        expression_parser,
        repeat_strict((token(Token::Comma), opt(expression_parser))),
        opt((token(Token::Comma), opt(clocking_event_parser))),
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e, f)| {
            SystemTfCall::Expressions(Box::new((a, b, c, d, e, f)))
        });
    alt((_args_parser, _data_parser, _expressions_parser)).parse_next(input)
}

pub fn subroutine_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SubroutineCall<'s>, VerboseError<'s>> {
    alt((
        tf_call_parser.map(|a| SubroutineCall::Tf(Box::new(a))),
        system_tf_call_parser.map(|a| SubroutineCall::SystemTf(Box::new(a))),
        method_call_parser.map(|a| SubroutineCall::Method(Box::new(a))),
        (
            opt((token(Token::Std), token(Token::ColonColon))),
            randomize_call_parser,
        )
            .map(|(a, b)| SubroutineCall::Randomize(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn function_subroutine_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<FunctionSubroutineCall<'s>, VerboseError<'s>> {
    subroutine_call_parser
        .map(|a| FunctionSubroutineCall(a))
        .parse_next(input)
}

pub fn list_of_arguments_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ListOfArguments<'s>, VerboseError<'s>> {
    let _expressions_parser = (
        opt(expression_parser),
        repeat_strict((token(Token::Comma), opt(expression_parser))),
        repeat_strict((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt(expression_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| ListOfArguments::Expressions(Box::new((a, b, c))));
    let _no_expressions_parser = (
        token(Token::Period),
        identifier_parser,
        token(Token::Paren),
        opt(expression_parser),
        token(Token::EParen),
        repeat_strict((
            token(Token::Comma),
            token(Token::Period),
            identifier_parser,
            token(Token::Paren),
            opt(expression_parser),
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d, e, f)| {
            ListOfArguments::NoExpressions(Box::new((a, b, c, d, e, f)))
        });
    alt((_expressions_parser, _no_expressions_parser)).parse_next(input)
}

pub fn method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodCall<'s>, VerboseError<'s>> {
    (
        method_call_root_parser,
        token(Token::Period),
        method_call_body_parser,
    )
        .map(|(a, b, c)| MethodCall(a, b, c))
        .parse_next(input)
}

pub fn method_call_body_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodCallBody<'s>, VerboseError<'s>> {
    let _custom_parser = (
        method_identifier_parser,
        attribute_instance_vec_parser,
        opt((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c)| MethodCallBody::Custom(Box::new((a, b, c))));
    let _build_in_parser = built_in_method_call_parser
        .map(|a| MethodCallBody::BuiltIn(Box::new(a)));
    alt((_custom_parser, _build_in_parser)).parse_next(input)
}

pub fn built_in_method_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BuiltInMethodCall<'s>, VerboseError<'s>> {
    alt((
        array_manipulation_call_parser
            .map(|a| BuiltInMethodCall::ArrayManip(Box::new(a))),
        randomize_call_parser
            .map(|a| BuiltInMethodCall::Randomize(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn array_manipulation_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ArrayManipulationCall<'s>, VerboseError<'s>> {
    (
        array_method_name_parser,
        attribute_instance_vec_parser,
        opt((
            token(Token::Paren),
            list_of_arguments_parser,
            token(Token::EParen),
        )),
        opt((
            token(Token::With),
            token(Token::Paren),
            expression_parser,
            token(Token::EParen),
        )),
    )
        .map(|(a, b, c, d)| ArrayManipulationCall(a, b, c, d))
        .parse_next(input)
}

pub fn randomize_call_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RandomizeCall<'s>, VerboseError<'s>> {
    let _variable_identifier_list_or_null_parser = alt((
        variable_identifier_list_parser.map(|a| {
            VariableIdentifierListOrNull::VariableIdentifierList(Box::new(a))
        }),
        token(Token::Null)
            .map(|a| VariableIdentifierListOrNull::Null(Box::new(a))),
    ));
    (
        token(Token::Randomize),
        attribute_instance_vec_parser,
        opt((
            token(Token::Paren),
            opt(_variable_identifier_list_or_null_parser),
            token(Token::EParen),
        )),
        opt((
            token(Token::With),
            opt((
                token(Token::Paren),
                opt(identifier_list_parser),
                token(Token::EParen),
            )),
            constraint_block_parser,
        )),
    )
        .map(|(a, b, c, d)| RandomizeCall(a, b, c, d))
        .parse_next(input)
}

pub fn variable_identifier_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariableIdentifierList<'s>, VerboseError<'s>> {
    (
        variable_identifier_parser,
        repeat_strict((token(Token::Comma), variable_identifier_parser)),
    )
        .map(|(a, b)| VariableIdentifierList(a, b))
        .parse_next(input)
}

pub fn identifier_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IdentifierList<'s>, VerboseError<'s>> {
    (
        identifier_parser,
        repeat_strict((token(Token::Comma), identifier_parser)),
    )
        .map(|(a, b)| IdentifierList(a, b))
        .parse_next(input)
}

// Parse separate from other primaries to avoid subroutine call recursion
fn method_root_primary_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Primary<'s>, VerboseError<'s>> {
    let _primary_literal_parser =
        primary_literal_parser.map(|a| Primary::PrimaryLiteral(Box::new(a)));
    let _hierarchical_identifier_parser = (
        opt(class_qualifier_or_package_scope_parser),
        hierarchical_identifier_parser,
        select_parser,
    )
        .map(|(a, b, c)| Primary::HierarchicalIdentifier(Box::new((a, b, c))));
    let _empty_unpacked_array_concatenation_parser =
        empty_unpacked_array_concatenation_parser
            .map(|a| Primary::EmptyUnpackedArrayConcatenation(Box::new(a)));
    let _concatenation_parser = (
        concatenation_parser,
        opt((
            token(Token::Bracket),
            range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| Primary::Concatenation(Box::new((a, b))));
    let _multiple_concatenation_parser = (
        multiple_concatenation_parser,
        opt((
            token(Token::Bracket),
            range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| Primary::MultipleConcatenation(Box::new((a, b))));
    let _let_expression_parser =
        let_expression_parser.map(|a| Primary::LetExpression(Box::new(a)));
    let _mintypmax_parser = (
        token(Token::Paren),
        mintypmax_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| Primary::MintypmaxExpression(Box::new((a, b, c))));
    let _assignment_pattern_expression_parser =
        assignment_pattern_expression_parser
            .map(|a| Primary::AssignmentPatternExpression(Box::new(a)));
    let _streaming_concatenation_parser = streaming_concatenation_parser
        .map(|a| Primary::StreamingConcatenation(Box::new(a)));
    let _sequence_method_call_parser = sequence_method_call_parser
        .map(|a| Primary::SequenceMethodCall(Box::new(a)));
    let _this_parser = token(Token::This).map(|a| Primary::This(Box::new(a)));
    let _dollar_parser =
        token(Token::Dollar).map(|a| Primary::This(Box::new(a)));
    let _null_parser = token(Token::Null).map(|a| Primary::This(Box::new(a)));
    alt((
        _primary_literal_parser,
        _hierarchical_identifier_parser,
        _empty_unpacked_array_concatenation_parser,
        _concatenation_parser,
        _multiple_concatenation_parser,
        _let_expression_parser,
        _mintypmax_parser,
        _assignment_pattern_expression_parser,
        _streaming_concatenation_parser,
        _sequence_method_call_parser,
        _this_parser,
        _dollar_parser,
        _null_parser,
    ))
    .parse_next(input)
}

pub fn method_call_root_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<MethodCallRoot<'s>, VerboseError<'s>> {
    alt((
        method_root_primary_parser
            .map(|a| MethodCallRoot::Primary(Box::new(a))),
        implicit_class_handle_parser
            .map(|a| MethodCallRoot::ImplicitClassHandle(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn array_method_name_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ArrayMethodName<'s>, VerboseError<'s>> {
    alt((
        method_identifier_parser.map(|a| ArrayMethodName::Method(Box::new(a))),
        token(Token::Unique).map(|a| ArrayMethodName::Unique(Box::new(a))),
        token(Token::And).map(|a| ArrayMethodName::And(Box::new(a))),
        token(Token::Or).map(|a| ArrayMethodName::Or(Box::new(a))),
        token(Token::Xor).map(|a| ArrayMethodName::Xor(Box::new(a))),
    ))
    .parse_next(input)
}
