// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use lexer::Span;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt, repeat};
use winnow::token::any;

pub fn constant_primary_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantPrimary<'s>, VerboseError<'s>> {
    let _primary_literal_parser = primary_literal_parser
        .map(|a| ConstantPrimary::PrimaryLiteral(Box::new(a)));
    let _ps_parameter_parser =
        (ps_parameter_identifier_parser, constant_select_parser)
            .map(|(a, b)| ConstantPrimary::PsParameter(Box::new((a, b))));
    let _specparam_parser = (
        specparam_identifier_parser,
        opt((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| ConstantPrimary::Specparam(Box::new((a, b))));
    let _genvar_parser =
        genvar_identifier_parser.map(|a| ConstantPrimary::Genvar(Box::new(a)));
    let _enum_parser =
        (opt(package_or_class_scope_parser), enum_identifier_parser)
            .map(|(a, b)| ConstantPrimary::Enum(Box::new((a, b))));
    let _empty_unpacked_array_concatenation_parser =
        empty_unpacked_array_concatenation_parser.map(|a| {
            ConstantPrimary::EmptyUnpackedArrayConcatenation(Box::new(a))
        });
    let _concatenation_parser = (
        constant_concatenation_parser,
        opt((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| ConstantPrimary::Concatenation(Box::new((a, b))));
    let _multiple_concatenation_parser = (
        constant_multiple_concatenation_parser,
        opt((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| ConstantPrimary::MultipleConcatenation(Box::new((a, b))));
    let _function_call_parser = (
        constant_function_call_parser,
        opt((
            token(Token::Bracket),
            constant_range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| ConstantPrimary::FunctionCall(Box::new((a, b))));
    let _let_expression_parser = constant_let_expression_parser
        .map(|a| ConstantPrimary::LetExpression(Box::new(a)));
    let _mintypmax_parser = (
        token(Token::Paren),
        constant_mintypmax_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| {
            ConstantPrimary::MintypmaxExpression(Box::new((a, b, c)))
        });
    let _cast_parser =
        constant_cast_parser.map(|a| ConstantPrimary::Cast(Box::new(a)));
    let _assignment_pattern_expression_parser =
        constant_assignment_pattern_expression_parser
            .map(|a| ConstantPrimary::AssignmentPatternExpression(Box::new(a)));
    let _type_reference_parser = type_reference_parser
        .map(|a| ConstantPrimary::TypeReference(Box::new(a)));
    let _null_parser =
        token(Token::Null).map(|a| ConstantPrimary::Null(Box::new(a)));
    alt((
        _primary_literal_parser,
        _ps_parameter_parser,
        _specparam_parser,
        _genvar_parser,
        _enum_parser,
        _empty_unpacked_array_concatenation_parser,
        _concatenation_parser,
        _multiple_concatenation_parser,
        _function_call_parser,
        _let_expression_parser,
        _mintypmax_parser,
        _cast_parser,
        _assignment_pattern_expression_parser,
        _type_reference_parser,
        _null_parser,
    ))
    .parse_next(input)
}

pub fn module_path_primary_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ModulePathPrimary<'s>, VerboseError<'s>> {
    alt((
        number_parser.map(|a| ModulePathPrimary::Number(Box::new(a))),
        identifier_parser.map(|a| ModulePathPrimary::Identifier(Box::new(a))),
        module_path_concatenation_parser
            .map(|a| ModulePathPrimary::Concatenation(Box::new(a))),
        module_path_multiple_concatenation_parser
            .map(|a| ModulePathPrimary::MultipleConcatenation(Box::new(a))),
        function_subroutine_call_parser
            .map(|a| ModulePathPrimary::FunctionSubroutineCall(Box::new(a))),
        (
            token(Token::Paren),
            module_path_mintypmax_expression_parser,
            token(Token::EParen),
        )
            .map(|(a, b, c)| {
                ModulePathPrimary::MintypmaxExpression(Box::new((a, b, c)))
            }),
    ))
    .parse_next(input)
}

pub fn primary_parser<'s>(
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
    let _function_subroutine_call_parser = (
        function_subroutine_call_parser,
        opt((
            token(Token::Bracket),
            range_expression_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| Primary::FunctionSubroutineCall(Box::new((a, b))));
    let _let_expression_parser =
        let_expression_parser.map(|a| Primary::LetExpression(Box::new(a)));
    let _mintypmax_parser = (
        token(Token::Paren),
        mintypmax_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| Primary::MintypmaxExpression(Box::new((a, b, c))));
    let _cast_parser = cast_parser.map(|a| Primary::Cast(Box::new(a)));
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
        _function_subroutine_call_parser,
        _let_expression_parser,
        _mintypmax_parser,
        _cast_parser,
        _assignment_pattern_expression_parser,
        _streaming_concatenation_parser,
        _sequence_method_call_parser,
        _this_parser,
        _dollar_parser,
        _null_parser,
    ))
    .parse_next(input)
}

fn class_qualifier_or_package_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassQualifierOrPackageScope<'s>, VerboseError<'s>> {
    alt((
        class_qualifier_parser
            .map(|a| ClassQualifierOrPackageScope::ClassQualifier(Box::new(a))),
        package_scope_parser
            .map(|a| ClassQualifierOrPackageScope::PackageScope(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn implicit_class_handle_or_class_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImplicitClassHandleOrClassScope<'s>, VerboseError<'s>> {
    alt((
        (implicit_class_handle_parser, token(Token::Period)).map(|(a, b)| {
            ImplicitClassHandleOrClassScope::ImplicitClassHandle(Box::new((
                a, b,
            )))
        }),
        class_scope_parser
            .map(|a| ImplicitClassHandleOrClassScope::ClassScope(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn class_qualifier_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassQualifier<'s>, VerboseError<'s>> {
    (
        opt((token(Token::Local), token(Token::ColonColon))),
        opt(implicit_class_handle_or_class_scope_parser),
    )
        .map(|(a, b)| ClassQualifier(a, b))
        .parse_next(input)
}

pub fn range_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<RangeExpression<'s>, VerboseError<'s>> {
    alt((
        expression_parser.map(|a| RangeExpression::Expression(Box::new(a))),
        part_select_range_parser
            .map(|a| RangeExpression::PartSelectRange(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn primary_literal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<PrimaryLiteral<'s>, VerboseError<'s>> {
    alt((
        number_parser.map(|a| PrimaryLiteral::Number(Box::new(a))),
        time_literal_parser.map(|a| PrimaryLiteral::TimeLiteral(Box::new(a))),
        unbased_unsized_literal_parser
            .map(|a| PrimaryLiteral::UnbasedUnsizedLiteral(Box::new(a))),
        string_literal_parser
            .map(|a| PrimaryLiteral::StringLiteral(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn time_literal_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimeLiteral<'s>, VerboseError<'s>> {
    (
        alt((
            fixed_point_number_parser.map(|a| {
                TimeLiteral::TimeLiteralFixedPoint(Box::new((
                    a,
                    TimeUnit::S(Metadata {
                        span: Span::default(),
                        extra_nodes: vec![],
                    }),
                )))
            }),
            unsigned_number_parser.map(|a| {
                TimeLiteral::TimeLiteralUnsigned(Box::new((
                    a,
                    TimeUnit::S(Metadata {
                        span: Span::default(),
                        extra_nodes: vec![],
                    }),
                )))
            }),
        )),
        time_unit_parser,
    )
        .map(|(a, b)| match a {
            TimeLiteral::TimeLiteralFixedPoint(box_value) => {
                TimeLiteral::TimeLiteralFixedPoint(Box::new((box_value.0, b)))
            }
            TimeLiteral::TimeLiteralUnsigned(box_value) => {
                TimeLiteral::TimeLiteralUnsigned(Box::new((box_value.0, b)))
            }
        })
        .parse_next(input)
}

fn time_unit_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TimeUnit<'s>, VerboseError<'s>> {
    (
        any.verify_map(|s: &'s SpannedToken<'s>| match s.0 {
            Token::TimeUnit("s") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            Token::TimeUnit("ms") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            Token::TimeUnit("us") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            Token::TimeUnit("ns") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            Token::TimeUnit("ps") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            Token::TimeUnit("fs") => Some(TimeUnit::S(Metadata {
                span: s.1.clone(),
                extra_nodes: Vec::new(),
            })),
            _ => None,
        }),
        extra_node_parser,
    )
        .map(|(time_unit, extra_nodes)| match time_unit {
            TimeUnit::S(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
            TimeUnit::MS(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
            TimeUnit::US(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
            TimeUnit::NS(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
            TimeUnit::PS(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
            TimeUnit::FS(metadata) => {
                TimeUnit::S(replace_nodes(metadata, extra_nodes))
            }
        })
        .context("a time unit")
        .parse_next(input)
}

pub fn implicit_class_handle_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImplicitClassHandle<'s>, VerboseError<'s>> {
    let _this_parser = token(Token::This).map(|a| ImplicitClassHandle::This(a));
    let _super_parser =
        token(Token::Super).map(|a| ImplicitClassHandle::Super(a));
    let _this_super_parser = (
        token(Token::This),
        token(Token::Period),
        token(Token::Super),
    )
        .map(|(a, b, c)| ImplicitClassHandle::ThisSuper(a, b, c));
    alt((_this_parser, _super_parser, _this_super_parser)).parse_next(input)
}

pub fn bit_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BitSelect<'s>, VerboseError<'s>> {
    repeat(
        0..,
        (
            token(Token::Bracket),
            expression_parser,
            token(Token::EBracket),
        ),
    )
    .map(|a| BitSelect(a))
    .parse_next(input)
}

fn member_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<
    (
        (
            Vec<(Metadata<'s>, MemberIdentifier<'s>, BitSelect<'s>)>,
            Metadata<'s>,
            MemberIdentifier<'s>,
        ),
        BitSelect<'s>,
    ),
    VerboseError<'s>,
> {
    let initial_result = (
        token(Token::Period),
        member_identifier_parser,
        bit_select_parser,
    )
        .parse_next(input)?;
    match member_select_parser(input) {
        Ok(mut sub_expr) => {
            sub_expr.0.0.push(initial_result);
            Ok(sub_expr)
        }
        Err(_) => Ok((
            (vec![], initial_result.0, initial_result.1),
            initial_result.2,
        )),
    }
}

pub fn select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Select<'s>, VerboseError<'s>> {
    let _member_select_parser = (
        member_select_parser,
        opt((
            token(Token::Bracket),
            part_select_range_parser,
            token(Token::EBracket),
        )),
    )
        .map(|((a, b), c)| Select(Some(a), b, c));
    let _no_member_select_parser = (
        bit_select_parser,
        opt((
            token(Token::Bracket),
            part_select_range_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| Select(None, a, b));
    alt((_member_select_parser, _no_member_select_parser)).parse_next(input)
}

pub fn nonrange_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonrangeSelect<'s>, VerboseError<'s>> {
    let _member_select_parser =
        member_select_parser.map(|(a, b)| NonrangeSelect(Some(a), b));
    let _no_member_select_parser =
        bit_select_parser.map(|a| NonrangeSelect(None, a));
    alt((_member_select_parser, _no_member_select_parser)).parse_next(input)
}

pub fn constant_bit_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantBitSelect<'s>, VerboseError<'s>> {
    repeat(
        0..,
        (
            token(Token::Bracket),
            constant_expression_parser,
            token(Token::EBracket),
        ),
    )
    .map(|a| ConstantBitSelect(a))
    .parse_next(input)
}

fn constant_member_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<
    (
        (
            Vec<(Metadata<'s>, MemberIdentifier<'s>, ConstantBitSelect<'s>)>,
            Metadata<'s>,
            MemberIdentifier<'s>,
        ),
        ConstantBitSelect<'s>,
    ),
    VerboseError<'s>,
> {
    let initial_result = (
        token(Token::Period),
        member_identifier_parser,
        constant_bit_select_parser,
    )
        .parse_next(input)?;
    match constant_member_select_parser(input) {
        Ok(mut sub_expr) => {
            sub_expr.0.0.push(initial_result);
            Ok(sub_expr)
        }
        Err(_) => Ok((
            (vec![], initial_result.0, initial_result.1),
            initial_result.2,
        )),
    }
}

pub fn constant_select_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantSelect<'s>, VerboseError<'s>> {
    let _member_select_parser = (
        constant_member_select_parser,
        opt((
            token(Token::Bracket),
            constant_part_select_range_parser,
            token(Token::EBracket),
        )),
    )
        .map(|((a, b), c)| ConstantSelect(Some(a), b, c));
    let _no_member_select_parser = (
        constant_bit_select_parser,
        opt((
            token(Token::Bracket),
            constant_part_select_range_parser,
            token(Token::EBracket),
        )),
    )
        .map(|(a, b)| ConstantSelect(None, a, b));
    alt((_member_select_parser, _no_member_select_parser)).parse_next(input)
}

pub fn cast_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Cast<'s>, VerboseError<'s>> {
    (
        casting_type_parser,
        token(Token::Apost),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| Cast(a, b, c, d, e))
        .parse_next(input)
}

pub fn constant_cast_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantCast<'s>, VerboseError<'s>> {
    (
        casting_type_parser,
        token(Token::Apost),
        token(Token::Paren),
        constant_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| ConstantCast(a, b, c, d, e))
        .parse_next(input)
}

pub fn constant_let_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ConstantLetExpression<'s>, VerboseError<'s>> {
    let_expression_parser
        .map(|a| ConstantLetExpression(a))
        .parse_next(input)
}
