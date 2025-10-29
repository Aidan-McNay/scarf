// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt};

pub fn casting_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CastingType<'s>, VerboseError<'s>> {
    alt((
        simple_type_parser.map(|a| CastingType::SimpleType(Box::new(a))),
        constant_primary_parser
            .map(|a| CastingType::ConstantPrimary(Box::new(a))),
        signing_parser.map(|a| CastingType::Signing(Box::new(a))),
        token(Token::String).map(|a| CastingType::String(Box::new(a))),
        token(Token::Const).map(|a| CastingType::Const(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn class_or_package_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassOrPackageScope<'s>, VerboseError<'s>> {
    alt((
        class_scope_parser.map(|a| ClassOrPackageScope::Class(Box::new(a))),
        package_scope_parser.map(|a| ClassOrPackageScope::Package(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn data_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataType<'s>, VerboseError<'s>> {
    let _vector_parser = (
        integer_vector_type_parser,
        opt(signing_parser),
        repeat_strict(packed_dimension_parser),
    )
        .map(|(a, b, c)| DataType::Vector(Box::new((a, b, c))));
    let _atom_parser = (integer_atom_type_parser, opt(signing_parser))
        .map(|(a, b)| DataType::Atom(Box::new((a, b))));
    let _non_integer_parser =
        non_integer_type_parser.map(|a| DataType::NonInteger(Box::new(a)));
    let _struct_union_parser = (
        struct_union_parser,
        opt((token(Token::Packed), opt(signing_parser))),
        token(Token::Brace),
        struct_union_member_parser,
        repeat_strict(struct_union_member_parser),
        token(Token::EBrace),
        repeat_strict(packed_dimension_parser),
    )
        .map(|(a, b, c, d, e, f, g)| {
            DataType::StructUnion(Box::new((a, b, c, d, e, f, g)))
        });
    let _enum_parser = (
        token(Token::Enum),
        opt(enum_base_type_parser),
        token(Token::Brace),
        enum_name_declaration_parser,
        repeat_strict((token(Token::Comma), enum_name_declaration_parser)),
        token(Token::EBrace),
        repeat_strict(packed_dimension_parser),
    )
        .map(|(a, b, c, d, e, f, g)| {
            DataType::Enum(Box::new((a, b, c, d, e, f, g)))
        });
    let _string_parser =
        token(Token::String).map(|a| DataType::String(Box::new(a)));
    let _chandle_parser =
        token(Token::Chandle).map(|a| DataType::Chandle(Box::new(a)));
    let _virtual_parser = (
        token(Token::Virtual),
        opt(token(Token::Interface)),
        interface_identifier_parser,
        opt(parameter_value_assignment_parser),
        opt((token(Token::Period), modport_identifier_parser)),
    )
        .map(|(a, b, c, d, e)| DataType::Virtual(Box::new((a, b, c, d, e))));
    let _type_parser = (
        class_or_package_scope_parser,
        type_identifier_parser,
        repeat_strict(packed_dimension_parser),
    )
        .map(|(a, b, c)| DataType::Type(Box::new((a, b, c))));
    let _class_type_parser =
        class_type_parser.map(|a| DataType::ClassType(Box::new(a)));
    let _event_parser =
        token(Token::Event).map(|a| DataType::Event(Box::new(a)));
    let _ps_covergroup_parser = ps_covergroup_identifier_parser
        .map(|a| DataType::PsCovergroup(Box::new(a)));
    let _type_ref_parser =
        type_reference_parser.map(|a| DataType::TypeRef(Box::new(a)));
    alt((
        _vector_parser,
        _atom_parser,
        _non_integer_parser,
        _struct_union_parser,
        _enum_parser,
        _string_parser,
        _chandle_parser,
        _virtual_parser,
        _type_parser,
        _class_type_parser,
        _event_parser,
        _ps_covergroup_parser,
        _type_ref_parser,
    ))
    .parse_next(input)
}

pub fn data_type_or_implicit_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrImplicit<'s>, VerboseError<'s>> {
    alt((
        data_type_parser.map(|a| DataTypeOrImplicit::DataType(a)),
        implicit_data_type_parser
            .map(|a| DataTypeOrImplicit::ImplicitDataType(a)),
    ))
    .parse_next(input)
}

pub fn implicit_data_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ImplicitDataType<'s>, VerboseError<'s>> {
    (opt(signing_parser), repeat_strict(packed_dimension_parser))
        .map(|(a, b)| ImplicitDataType(a, b))
        .parse_next(input)
}

pub fn enum_base_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnumBaseType<'s>, VerboseError<'s>> {
    let _atom_parser = (integer_atom_type_parser, opt(signing_parser))
        .map(|(a, b)| EnumBaseType::Atom(Box::new((a, b))));
    let _vector_parser = (
        integer_vector_type_parser,
        opt(signing_parser),
        opt(packed_dimension_parser),
    )
        .map(|(a, b, c)| EnumBaseType::Vector(Box::new((a, b, c))));
    let _type_parser = (type_identifier_parser, opt(packed_dimension_parser))
        .map(|(a, b)| EnumBaseType::Type(Box::new((a, b))));
    alt((_atom_parser, _vector_parser, _type_parser)).parse_next(input)
}

pub fn enum_name_declaration_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<EnumNameDeclaration<'s>, VerboseError<'s>> {
    (
        enum_identifier_parser,
        opt((
            token(Token::Bracket),
            integral_number_parser,
            opt((token(Token::Colon), integral_number_parser)),
            token(Token::EBracket),
        )),
        opt((token(Token::Eq), constant_expression_parser)),
    )
        .map(|(a, b, c)| EnumNameDeclaration(a, b, c))
        .parse_next(input)
}

pub fn class_scope_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassScope<'s>, VerboseError<'s>> {
    (class_type_parser, token(Token::ColonColon))
        .map(|(a, b)| ClassScope(a, b))
        .parse_next(input)
}

pub fn class_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<ClassType<'s>, VerboseError<'s>> {
    (
        ps_class_identifier_parser,
        opt(parameter_value_assignment_parser),
        repeat_strict((
            token(Token::ColonColon),
            class_identifier_parser,
            opt(parameter_value_assignment_parser),
        )),
    )
        .map(|(a, b, c)| ClassType(a, b, c))
        .parse_next(input)
}

pub fn interface_class_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassType<'s>, VerboseError<'s>> {
    (
        ps_class_identifier_parser,
        opt(parameter_value_assignment_parser),
    )
        .map(|(a, b)| InterfaceClassType(a, b))
        .parse_next(input)
}

pub fn integer_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegerType<'s>, VerboseError<'s>> {
    alt((
        integer_atom_type_parser.map(|a| IntegerType::Atom(Box::new(a))),
        integer_vector_type_parser.map(|a| IntegerType::Vector(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn integer_atom_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegerAtomType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Byte).map(|a| IntegerAtomType::Byte(a)),
        token(Token::Shortint).map(|a| IntegerAtomType::Shortint(a)),
        token(Token::Int).map(|a| IntegerAtomType::Int(a)),
        token(Token::Longint).map(|a| IntegerAtomType::Longint(a)),
        token(Token::Integer).map(|a| IntegerAtomType::Integer(a)),
        token(Token::Time).map(|a| IntegerAtomType::Time(a)),
    ))
    .parse_next(input)
}

pub fn integer_vector_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegerVectorType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Bit).map(|a| IntegerVectorType::Bit(a)),
        token(Token::Logic).map(|a| IntegerVectorType::Logic(a)),
        token(Token::Reg).map(|a| IntegerVectorType::Reg(a)),
    ))
    .parse_next(input)
}

pub fn non_integer_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NonIntegerType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Shortreal).map(|a| NonIntegerType::Shortreal(a)),
        token(Token::Real).map(|a| NonIntegerType::Real(a)),
        token(Token::Realtime).map(|a| NonIntegerType::Realtime(a)),
    ))
    .parse_next(input)
}

pub fn net_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetType<'s>, VerboseError<'s>> {
    alt((
        token(Token::Supply0).map(|a| NetType::Supply0(a)),
        token(Token::Supply1).map(|a| NetType::Supply1(a)),
        token(Token::Tri).map(|a| NetType::Tri(a)),
        token(Token::Triand).map(|a| NetType::Triand(a)),
        token(Token::Trior).map(|a| NetType::Trior(a)),
        token(Token::Trireg).map(|a| NetType::Trireg(a)),
        token(Token::Tri0).map(|a| NetType::Tri0(a)),
        token(Token::Tri1).map(|a| NetType::Tri1(a)),
        token(Token::Uwire).map(|a| NetType::Uwire(a)),
        token(Token::Wire).map(|a| NetType::Wire(a)),
        token(Token::Wand).map(|a| NetType::Wand(a)),
        token(Token::Wor).map(|a| NetType::Wor(a)),
    ))
    .parse_next(input)
}

pub fn net_port_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NetPortType<'s>, VerboseError<'s>> {
    let _implicit_parser = (opt(net_type_parser), data_type_or_implicit_parser)
        .map(|(a, b)| NetPortType::Implicit(Box::new((a, b))));
    let _nettype_parser =
        nettype_identifier_parser.map(|a| NetPortType::Nettype(Box::new(a)));
    let _interconnect_parser =
        (token(Token::Interconnect), implicit_data_type_parser)
            .map(|(a, b)| NetPortType::Interconnect(Box::new((a, b))));
    alt((_implicit_parser, _nettype_parser, _interconnect_parser))
        .parse_next(input)
}

pub fn variable_port_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariablePortType<'s>, VerboseError<'s>> {
    var_data_type_parser
        .map(|a| VariablePortType(a))
        .parse_next(input)
}

pub fn var_data_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VarDataType<'s>, VerboseError<'s>> {
    alt((
        data_type_parser.map(|a| VarDataType::Data(Box::new(a))),
        (token(Token::Var), data_type_or_implicit_parser)
            .map(|(a, b)| VarDataType::Var(Box::new((a, b)))),
    ))
    .parse_next(input)
}

pub fn signing_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<Signing<'s>, VerboseError<'s>> {
    alt((
        token(Token::Signed).map(|a| Signing::Signed(a)),
        token(Token::Unsigned).map(|a| Signing::Unsigned(a)),
    ))
    .parse_next(input)
}

pub fn simple_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SimpleType<'s>, VerboseError<'s>> {
    alt((
        integer_type_parser.map(|a| SimpleType::Integer(Box::new(a))),
        non_integer_type_parser.map(|a| SimpleType::NonInteger(Box::new(a))),
        ps_type_identifier_parser.map(|a| SimpleType::PsType(Box::new(a))),
        ps_parameter_identifier_parser
            .map(|a| SimpleType::PsParameter(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn struct_union_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StructUnion<'s>, VerboseError<'s>> {
    let soft_or_tagged_parser = alt((
        token(Token::Soft).map(|a| SoftOrTagged::Soft(a)),
        token(Token::Tagged).map(|a| SoftOrTagged::Tagged(a)),
    ));
    alt((
        token(Token::Struct).map(|a| StructUnion::Struct(a)),
        (token(Token::Union), opt(soft_or_tagged_parser))
            .map(|(a, b)| StructUnion::Union(a, b)),
    ))
    .parse_next(input)
}

pub fn struct_union_member_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<StructUnionMember<'s>, VerboseError<'s>> {
    (
        attribute_instance_vec_parser,
        opt(random_qualifier_parser),
        data_type_or_void_parser,
        list_of_variable_decl_assignments_parser,
        token(Token::SColon),
    )
        .map(|(a, b, c, d, e)| StructUnionMember(a, b, c, d, e))
        .parse_next(input)
}

pub fn data_type_or_void_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrVoid<'s>, VerboseError<'s>> {
    alt((
        data_type_parser.map(|a| DataTypeOrVoid::DataType(Box::new(a))),
        token(Token::Void).map(|a| DataTypeOrVoid::Void(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn type_reference_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeReference<'s>, VerboseError<'s>> {
    let _expression_parser = (
        token(Token::Type),
        token(Token::Paren),
        expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| TypeReference::Expression(Box::new((a, b, c, d))));
    let _data_type_or_incomplete_class_scoped_type_parser = (
        token(Token::Type),
        token(Token::Paren),
        data_type_or_incomplete_class_scoped_type_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d)| {
            TypeReference::DataTypeOrIncompleteClassScopedType(Box::new((
                a, b, c, d,
            )))
        });
    alt((
        _expression_parser,
        _data_type_or_incomplete_class_scoped_type_parser,
    ))
    .parse_next(input)
}

pub fn data_type_or_incomplete_class_scoped_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataTypeOrIncompleteClassScopedType<'s>, VerboseError<'s>> {
    alt((
        data_type_parser
            .map(|a| DataTypeOrIncompleteClassScopedType::Data(Box::new(a))),
        incomplete_class_scoped_type_parser.map(|a| {
            DataTypeOrIncompleteClassScopedType::IncompleteClassScoped(
                Box::new(a),
            )
        }),
    ))
    .parse_next(input)
}

pub fn incomplete_class_scoped_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IncompleteClassScopedType<'s>, VerboseError<'s>> {
    let mut base = (
        type_identifier_parser,
        token(Token::ColonColon),
        type_identifier_or_class_type_parser,
    )
        .map(|(a, b, c)| IncompleteClassScopedType::Base(Box::new((a, b, c))))
        .parse_next(input)?;
    loop {
        let Ok(metadata) = token(Token::ColonColon).parse_next(input) else {
            return Ok(base);
        };
        let type_identifier_or_class_type =
            type_identifier_or_class_type_parser.parse_next(input)?;
        base = IncompleteClassScopedType::Recursive(Box::new((
            base,
            metadata,
            type_identifier_or_class_type,
        )));
    }
}

pub fn type_identifier_or_class_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<TypeIdentifierOrClassType<'s>, VerboseError<'s>> {
    alt((
        type_identifier_parser
            .map(|a| TypeIdentifierOrClassType::Type(Box::new(a))),
        class_type_parser
            .map(|a| TypeIdentifierOrClassType::Class(Box::new(a))),
    ))
    .parse_next(input)
}
