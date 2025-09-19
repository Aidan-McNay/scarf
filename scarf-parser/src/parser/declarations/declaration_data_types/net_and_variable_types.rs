// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, fail, opt, repeat};

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

pub fn data_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<DataType<'s>, VerboseError<'s>> {
    fail.parse_next(input)
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
    (opt(signing_parser), repeat(0.., packed_dimension_parser))
        .map(|(a, b)| ImplicitDataType(a, b))
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
    fail.parse_next(input)
}

pub fn interface_class_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<InterfaceClassType, VerboseError<'s>> {
    fail.parse_next(input)
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
    fail.parse_next(input)
}

pub fn variable_port_type_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<VariablePortType<'s>, VerboseError<'s>> {
    fail.parse_next(input)
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
    fail.parse_next(input)
}
