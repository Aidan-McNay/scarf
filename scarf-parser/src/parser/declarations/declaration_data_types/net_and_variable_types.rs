// =======================================================================
// net_and_variable_types.rs
// =======================================================================
// Parsing for 1800-2023 A.2.2.1

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn data_type_parser<'a, I>() -> impl Parser<'a, I, DataType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn data_type_or_implicit_parser<'a, I>()
-> impl Parser<'a, I, DataTypeOrImplicit<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn class_type_parser<'a, I>() -> impl Parser<'a, I, ClassType, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn interface_class_type_parser<'a, I>()
-> impl Parser<'a, I, InterfaceClassType, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn integer_atom_type_parser<'a, I>() -> impl Parser<'a, I, IntegerAtomType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Byte).map(|a| IntegerAtomType::Byte(a)),
        token(Token::Shortint).map(|a| IntegerAtomType::Shortint(a)),
        token(Token::Int).map(|a| IntegerAtomType::Int(a)),
        token(Token::Longint).map(|a| IntegerAtomType::Longint(a)),
        token(Token::Integer).map(|a| IntegerAtomType::Integer(a)),
        token(Token::Time).map(|a| IntegerAtomType::Time(a)),
    ))
}

pub fn integer_vector_type_parser<'a, I>()
-> impl Parser<'a, I, IntegerVectorType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Bit).map(|a| IntegerVectorType::Bit(a)),
        token(Token::Logic).map(|a| IntegerVectorType::Logic(a)),
        token(Token::Reg).map(|a| IntegerVectorType::Reg(a)),
    ))
}

pub fn non_integer_type_parser<'a, I>() -> impl Parser<'a, I, NonIntegerType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Shortreal).map(|a| NonIntegerType::Shortreal(a)),
        token(Token::Real).map(|a| NonIntegerType::Real(a)),
        token(Token::Realtime).map(|a| NonIntegerType::Realtime(a)),
    ))
}

pub fn net_type_parser<'a, I>() -> impl Parser<'a, I, NetType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
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
}

pub fn net_port_type_parser<'a, I>() -> impl Parser<'a, I, NetPortType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn variable_port_type_parser<'a, I>()
-> impl Parser<'a, I, VariablePortType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}

pub fn signing_parser<'a, I>() -> impl Parser<'a, I, Signing<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    choice((
        token(Token::Signed).map(|a| Signing::Signed(a)),
        token(Token::Unsigned).map(|a| Signing::Unsigned(a)),
    ))
}

pub fn struct_union_parser<'a, I>() -> impl Parser<'a, I, StructUnion<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
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
}

pub fn data_type_or_incomplete_class_scoped_type_parser<'a, I>()
-> impl Parser<'a, I, DataTypeOrIncompleteClassScopedType<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    todo_parser()
}
