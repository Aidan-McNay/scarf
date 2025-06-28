// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn casting_type_parser<'a>(
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
) -> impl Parser<'a, ParserInput<'a>, CastingType<'a>, ParserError<'a>> + Clone {
    choice((
        simple_type_parser(constant_expression_parser)
            .map(|a| CastingType::SimpleType(Box::new(a))),
        constant_primary_parser.map(|a| CastingType::ConstantPrimary(Box::new(a))),
        signing_parser().map(|a| CastingType::Signing(Box::new(a))),
        token(Token::String).map(|a| CastingType::String(Box::new(a))),
        token(Token::Const).map(|a| CastingType::Const(Box::new(a))),
    ))
    .boxed()
}

pub fn data_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, DataType<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn data_type_or_implicit_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, DataTypeOrImplicit<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn class_scope_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassScope<'a>, ParserError<'a>> + Clone {
    class_type_parser()
        .then(token(Token::ColonColon))
        .map(|(a, b)| ClassScope(a, b))
        .boxed()
}

pub fn class_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, ClassType<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn interface_class_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, InterfaceClassType, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn integer_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, IntegerType<'a>, ParserError<'a>> + Clone {
    choice((
        integer_atom_type_parser().map(|a| IntegerType::Atom(Box::new(a))),
        integer_vector_type_parser().map(|a| IntegerType::Vector(Box::new(a))),
    ))
    .boxed()
}

pub fn integer_atom_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, IntegerAtomType<'a>, ParserError<'a>> + Clone {
    choice((
        token(Token::Byte).map(|a| IntegerAtomType::Byte(a)),
        token(Token::Shortint).map(|a| IntegerAtomType::Shortint(a)),
        token(Token::Int).map(|a| IntegerAtomType::Int(a)),
        token(Token::Longint).map(|a| IntegerAtomType::Longint(a)),
        token(Token::Integer).map(|a| IntegerAtomType::Integer(a)),
        token(Token::Time).map(|a| IntegerAtomType::Time(a)),
    ))
    .boxed()
}

pub fn integer_vector_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, IntegerVectorType<'a>, ParserError<'a>> + Clone {
    choice((
        token(Token::Bit).map(|a| IntegerVectorType::Bit(a)),
        token(Token::Logic).map(|a| IntegerVectorType::Logic(a)),
        token(Token::Reg).map(|a| IntegerVectorType::Reg(a)),
    ))
    .boxed()
}

pub fn non_integer_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NonIntegerType<'a>, ParserError<'a>> + Clone {
    choice((
        token(Token::Shortreal).map(|a| NonIntegerType::Shortreal(a)),
        token(Token::Real).map(|a| NonIntegerType::Real(a)),
        token(Token::Realtime).map(|a| NonIntegerType::Realtime(a)),
    ))
    .boxed()
}

pub fn net_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NetType<'a>, ParserError<'a>> + Clone {
    choice((
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
    .boxed()
}

pub fn net_port_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, NetPortType<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn variable_port_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, VariablePortType<'a>, ParserError<'a>> + Clone {
    todo_parser()
}

pub fn signing_parser<'a>() -> impl Parser<'a, ParserInput<'a>, Signing<'a>, ParserError<'a>> + Clone
{
    choice((
        token(Token::Signed).map(|a| Signing::Signed(a)),
        token(Token::Unsigned).map(|a| Signing::Unsigned(a)),
    ))
    .boxed()
}

pub fn simple_type_parser<'a>(
    constant_expression_parser: impl Parser<
        'a,
        ParserInput<'a>,
        ConstantExpression<'a>,
        ParserError<'a>,
    > + Clone
    + 'a,
) -> impl Parser<'a, ParserInput<'a>, SimpleType<'a>, ParserError<'a>> + Clone {
    choice((
        integer_type_parser().map(|a| SimpleType::Integer(Box::new(a))),
        non_integer_type_parser().map(|a| SimpleType::NonInteger(Box::new(a))),
        ps_type_identifier_parser().map(|a| SimpleType::PsType(Box::new(a))),
        ps_parameter_identifier_parser(constant_expression_parser)
            .map(|a| SimpleType::PsParameter(Box::new(a))),
    ))
    .boxed()
}

pub fn struct_union_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, StructUnion<'a>, ParserError<'a>> + Clone {
    let soft_or_tagged_parser = choice((
        token(Token::Soft).map(|a| SoftOrTagged::Soft(a)),
        token(Token::Tagged).map(|a| SoftOrTagged::Tagged(a)),
    ));
    choice((
        token(Token::Struct).map(|a| StructUnion::Struct(a)),
        token(Token::Union)
            .then(soft_or_tagged_parser.or_not())
            .map(|(a, b)| StructUnion::Union(a, b)),
    ))
    .boxed()
}

pub fn type_reference_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, TypeReference<'a>, ParserError<'a>> + Clone {
    let _expression_parser = token(Token::Type)
        .then(token(Token::Paren))
        .then(expression_parser)
        .then(token(Token::EParen))
        .map(|(((a, b), c), d)| TypeReference::Expression(Box::new((a, b, c, d))));
    let _data_type_or_incomplete_class_scoped_type_parser = token(Token::Type)
        .then(token(Token::Paren))
        .then(data_type_or_incomplete_class_scoped_type_parser())
        .then(token(Token::EParen))
        .map(|(((a, b), c), d)| {
            TypeReference::DataTypeOrIncompleteClassScopedType(Box::new((a, b, c, d)))
        });
    choice((
        _expression_parser,
        _data_type_or_incomplete_class_scoped_type_parser,
    ))
    .boxed()
}

pub fn data_type_or_incomplete_class_scoped_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, DataTypeOrIncompleteClassScopedType<'a>, ParserError<'a>> + Clone
{
    todo_parser()
}
