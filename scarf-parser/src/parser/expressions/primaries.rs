// =======================================================================
// primaries.rs
// =======================================================================
// Parsing for 1800-2023 A.8.4

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn constant_primary_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantPrimary<'a>, ParserError<'a>> + Clone {
    let mut parser = Recursive::declare();
    let _range_slice_parser = token(Token::Bracket)
        .then(constant_range_expression_parser(
            constant_expression_parser.clone(),
        ))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| (a, b, c));
    let _primary_literal_parser =
        primary_literal_parser().map(|a| ConstantPrimary::PrimaryLiteral(Box::new(a)));
    let _ps_parameter_parser = ps_parameter_identifier_parser(constant_expression_parser.clone())
        .then(constant_select_parser(constant_expression_parser.clone()))
        .map(|(a, b)| ConstantPrimary::PsParameter(Box::new((a, b))));
    let _specparam_parser = specparam_identifier_parser()
        .then(_range_slice_parser.clone().or_not())
        .map(|(a, b)| ConstantPrimary::Specparam(Box::new((a, b))));
    let _genvar_parser = genvar_identifier_parser().map(|a| ConstantPrimary::Genvar(Box::new(a)));
    let _enum_parser = package_or_class_scope_parser()
        .or_not()
        .then(enum_identifier_parser())
        .map(|(a, b)| ConstantPrimary::Enum(Box::new((a, b))));
    let _empty_unpacked_array_concatenation_parser = empty_unpacked_array_concatenation_parser()
        .map(|a| ConstantPrimary::EmptyUnpackedArrayConcatenation(Box::new(a)));
    let _concatenation_parser = constant_concatenation_parser(constant_expression_parser.clone())
        .then(_range_slice_parser.clone().or_not())
        .map(|(a, b)| ConstantPrimary::Concatenation(Box::new((a, b))));
    let _multiple_concatenation_parser =
        constant_multiple_concatenation_parser(constant_expression_parser.clone())
            .then(_range_slice_parser.clone().or_not())
            .map(|(a, b)| ConstantPrimary::MultipleConcatenation(Box::new((a, b))));
    let _function_call_parser = constant_function_call_parser(constant_expression_parser.clone())
        .then(_range_slice_parser.clone().or_not())
        .map(|(a, b)| ConstantPrimary::FunctionCall(Box::new((a, b))));
    let _let_expression_parser = constant_let_expression_parser(constant_expression_parser.clone())
        .map(|a| ConstantPrimary::LetExpression(Box::new(a)));
    let _mintypmax_parser = token(Token::Paren)
        .then(constant_mintypmax_expression_parser(
            constant_expression_parser.clone(),
        ))
        .then(token(Token::EParen))
        .map(|((a, b), c)| ConstantPrimary::MintypmaxExpression(Box::new((a, b, c))));
    let _cast_parser = constant_cast_parser(constant_expression_parser.clone(), parser.clone())
        .map(|a| ConstantPrimary::Cast(Box::new(a)));
    let _assignment_pattern_expression_parser =
        constant_assignment_pattern_expression_parser(constant_expression_parser)
            .map(|a| ConstantPrimary::AssignmentPatternExpression(Box::new(a)));
    let _type_reference_parser = type_reference_parser(expression_parser)
        .map(|a| ConstantPrimary::TypeReference(Box::new(a)));
    let _null_parser = token(Token::Null).map(|a| ConstantPrimary::Null(Box::new(a)));
    parser.define(choice((
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
    )));
    parser
}

pub fn module_path_primary_parser<'a>(
    module_path_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ModulePathExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ModulePathPrimary<'a>, ParserError<'a>> + Clone {
    choice((
        number_parser().map(|a| ModulePathPrimary::Number(Box::new(a))),
        identifier_parser().map(|a| ModulePathPrimary::Identifier(Box::new(a))),
        module_path_concatenation_parser(module_path_expression_parser.clone())
            .map(|a| ModulePathPrimary::Concatenation(Box::new(a))),
        module_path_multiple_concatenation_parser(module_path_expression_parser.clone())
            .map(|a| ModulePathPrimary::MultipleConcatenation(Box::new(a))),
        function_subroutine_call_parser()
            .map(|a| ModulePathPrimary::FunctionSubroutineCall(Box::new(a))),
        token(Token::Paren)
            .then(module_path_mintypmax_expression_parser(
                module_path_expression_parser,
            ))
            .then(token(Token::EParen))
            .map(|((a, b), c)| ModulePathPrimary::MintypmaxExpression(Box::new((a, b, c)))),
    ))
}

pub fn primary_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, Primary<'a>, ParserError<'a>> + Clone {
    let _range_slice_parser = token(Token::Bracket)
        .then(range_expression_parser(expression_parser.clone()))
        .then(token(Token::EBracket))
        .map(|((a, b), c)| (a, b, c));
    let _primary_literal_parser =
        primary_literal_parser().map(|a| Primary::PrimaryLiteral(Box::new(a)));
    let _hierarchical_identifier_parser = class_qualifier_or_package_scope_parser()
        .or_not()
        .then(hierarchical_identifier_parser(expression_parser.clone()))
        .then(select_parser())
        .map(|((a, b), c)| Primary::HierarchicalIdentifier(Box::new((a, b, c))));
    let _empty_unpacked_array_concatenation_parser = empty_unpacked_array_concatenation_parser()
        .map(|a| Primary::EmptyUnpackedArrayConcatenation(Box::new(a)));
    let _concatenation_parser = concatenation_parser(expression_parser.clone())
        .then(_range_slice_parser.clone().or_not())
        .map(|(a, b)| Primary::Concatenation(Box::new((a, b))));
    let _multiple_concatenation_parser = multiple_concatenation_parser(expression_parser.clone())
        .then(_range_slice_parser.clone().or_not())
        .map(|(a, b)| Primary::MultipleConcatenation(Box::new((a, b))));
    let _function_subroutine_call_parser = function_subroutine_call_parser()
        .then(_range_slice_parser.or_not())
        .map(|(a, b)| Primary::FunctionSubroutineCall(Box::new((a, b))));
    let _let_expression_parser = let_expression_parser(expression_parser.clone())
        .map(|a| Primary::LetExpression(Box::new(a)));
    let _mintypmax_parser = token(Token::Paren)
        .then(mintypmax_expression_parser(expression_parser.clone()))
        .then(token(Token::EParen))
        .map(|((a, b), c)| Primary::MintypmaxExpression(Box::new((a, b, c))));
    let _cast_parser = cast_parser(expression_parser.clone()).map(|a| Primary::Cast(Box::new(a)));
    let _assignment_pattern_expression_parser =
        assignment_pattern_expression_parser(expression_parser.clone())
            .map(|a| Primary::AssignmentPatternExpression(Box::new(a)));
    let _streaming_concatenation_parser = streaming_concatenation_parser(expression_parser.clone())
        .map(|a| Primary::StreamingConcatenation(Box::new(a)));
    let _sequence_method_call_parser = sequence_method_call_parser(expression_parser.clone())
        .map(|a| Primary::SequenceMethodCall(Box::new(a)));
    let _this_parser = token(Token::This).map(|a| Primary::This(Box::new(a)));
    let _dollar_parser = token(Token::Dollar).map(|a| Primary::This(Box::new(a)));
    let _null_parser = token(Token::Null).map(|a| Primary::This(Box::new(a)));
    choice((
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
}

fn class_qualifier_or_package_scope_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassQualifierOrPackageScope<'a>, ParserError<'a>> + Clone {
    choice((
        class_qualifier_parser().map(|a| ClassQualifierOrPackageScope::ClassQualifier(Box::new(a))),
        package_scope_parser().map(|a| ClassQualifierOrPackageScope::PackageScope(Box::new(a))),
    ))
}

fn implicit_class_handle_or_class_scope_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ImplicitClassHandleOrClassScope<'a>, ParserError<'a>> + Clone {
    choice((
        implicit_class_handle_parser()
            .then(token(Token::Period))
            .map(|(a, b)| ImplicitClassHandleOrClassScope::ImplicitClassHandle(Box::new((a, b)))),
        class_scope_parser().map(|a| ImplicitClassHandleOrClassScope::ClassScope(Box::new(a))),
    ))
}

pub fn class_qualifier_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassQualifier<'a>, ParserError<'a>> + Clone {
    token(Token::Local)
        .then(token(Token::ColonColon))
        .or_not()
        .then(implicit_class_handle_or_class_scope_parser().or_not())
        .map(|(a, b)| ClassQualifier(a, b))
}

pub fn range_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, RangeExpression<'a>, ParserError<'a>> + Clone {
    choice((
        expression_parser
            .clone()
            .map(|a| RangeExpression::Expression(Box::new(a))),
        part_select_range_parser(expression_parser)
            .map(|a| RangeExpression::PartSelectRange(Box::new(a))),
    ))
}

pub fn primary_literal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, PrimaryLiteral<'a>, ParserError<'a>> + Clone {
    choice((
        number_parser().map(|a| PrimaryLiteral::Number(Box::new(a))),
        time_literal_parser().map(|a| PrimaryLiteral::TimeLiteral(Box::new(a))),
        unbased_unsized_literal_parser()
            .map(|a| PrimaryLiteral::UnbasedUnsizedLiteral(Box::new(a))),
        string_literal_parser().map(|a| PrimaryLiteral::StringLiteral(Box::new(a))),
    ))
}

pub fn time_literal_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, TimeLiteral<'a>, ParserError<'a>> + Clone {
    choice((
        fixed_point_number_parser().map(|a| {
            TimeLiteral::TimeLiteralFixedPoint(Box::new((a, TimeUnit::S(Metadata::default()))))
        }),
        unsigned_number_parser().map(|a| {
            TimeLiteral::TimeLiteralUnsigned(Box::new((a, TimeUnit::S(Metadata::default()))))
        }),
    ))
    .then(time_unit_parser())
    .map(|(a, b)| match a {
        TimeLiteral::TimeLiteralFixedPoint(box_value) => {
            TimeLiteral::TimeLiteralFixedPoint(Box::new((box_value.0, b)))
        }
        TimeLiteral::TimeLiteralUnsigned(box_value) => {
            TimeLiteral::TimeLiteralUnsigned(Box::new((box_value.0, b)))
        }
    })
}

fn time_unit_parser<'a>() -> impl Parser<'a, ParserInput<'a>, TimeUnit<'a>, ParserError<'a>> + Clone
{
    select! {
        Token::TimeUnit(unit) = e if unit == "s" => TimeUnit::S(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ms" => TimeUnit::MS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "us" => TimeUnit::US(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ns" => TimeUnit::NS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "ps" => TimeUnit::PS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
        Token::TimeUnit(unit) = e if unit == "fs" => TimeUnit::FS(Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        }),
    }
    .labelled("a time unit")
    .then(extra_node_parser())
    .map(|(timeunit, b)| match timeunit {
        TimeUnit::S(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::MS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::US(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::NS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::PS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
        TimeUnit::FS(metadata) => TimeUnit::S(replace_nodes(metadata, b)),
    })
    .boxed()
}

pub fn select_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Select<'a>, ParserError<'a>> + Clone
{
    todo_parser()
}

pub fn nonrange_select_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NonrangeSelect<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn implicit_class_handle_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ImplicitClassHandle<'a>, ParserError<'a>> + Clone {
    let _this_parser = token(Token::This).map(|a| ImplicitClassHandle::This(a));
    let _super_parser = token(Token::Super).map(|a| ImplicitClassHandle::Super(a));
    let _this_super_parser = token(Token::This)
        .then(token(Token::Period))
        .then(token(Token::Super))
        .map(|((a, b), c)| ImplicitClassHandle::ThisSuper(a, b, c));
    choice((_this_parser, _super_parser, _this_super_parser)).boxed()
}

pub fn constant_bit_select_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantBitSelect<'a>, ParserError<'a>> + Clone {
    token(Token::Bracket)
        .then(constant_expression_parser)
        .then(token(Token::EBracket))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Metadata<'a>, ConstantExpression<'a>, Metadata<'a>)>>()
        .map(|a| ConstantBitSelect(a))
}

pub fn constant_select_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantSelect<'a>, ParserError<'a>> + Clone {
    let _hierarchy_parser = token(Token::Period)
        .then(member_identifier_parser())
        .then(constant_bit_select_parser(
            constant_expression_parser.clone(),
        ))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Metadata<'a>, MemberIdentifier<'a>, ConstantBitSelect<'a>)>>()
        .then(token(Token::Period))
        .then(member_identifier_parser())
        .map(|((a, b), c)| (a, b, c));
    _hierarchy_parser
        .or_not()
        .then(constant_bit_select_parser(
            constant_expression_parser.clone(),
        ))
        .then(
            token(Token::Bracket)
                .then(constant_part_select_range_parser(
                    constant_expression_parser,
                ))
                .then(token(Token::EBracket))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .map(|((a, b), c)| ConstantSelect(a, b, c))
}

pub fn cast_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, Cast<'a>, ParserError<'a>> + Clone {
    casting_type_parser(
        constant_expression_parser(expression_parser.clone()),
        constant_primary_parser(
            constant_expression_parser(expression_parser.clone()),
            expression_parser.clone(),
        ),
    )
    .then(token(Token::Apost))
    .then(token(Token::Paren))
    .then(expression_parser)
    .then(token(Token::EParen))
    .map(|((((a, b), c), d), e)| Cast(a, b, c, d, e))
}

pub fn constant_cast_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
    constant_primary_parser: impl Parser<'a, ParserInput<'a>, ConstantPrimary<'a>, ParserError<'a>>
    + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantCast<'a>, ParserError<'a>> + Clone {
    casting_type_parser(constant_expression_parser.clone(), constant_primary_parser)
        .then(token(Token::Apost))
        .then(token(Token::Paren))
        .then(constant_expression_parser)
        .then(token(Token::EParen))
        .map(|((((a, b), c), d), e)| ConstantCast(a, b, c, d, e))
}

pub fn constant_let_expression_parser<'a>(
    _constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, ConstantLetExpression<'a>, ParserError<'a>> + Clone {
    todo_parser()
}
