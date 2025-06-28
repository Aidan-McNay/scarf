// =======================================================================
// let_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.12

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn let_declaration_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LetDeclaration<'a>, ParserError<'a>> + Clone {
    token(Token::Let)
        .then(let_identifier_parser())
        .then(
            token(Token::Paren)
                .then(let_port_list_parser().or_not())
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::Eq))
        .then(expression_parser())
        .then(token(Token::SColon))
        .map(|(((((a, b), c), d), e), f)| LetDeclaration(a, b, c, d, e, f))
        .boxed()
}

pub fn let_identifier_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LetIdentifier<'a>, ParserError<'a>> + Clone {
    identifier_parser().map(|a| LetIdentifier(a))
}

pub fn let_port_list_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LetPortList<'a>, ParserError<'a>> + Clone {
    let_port_item_parser()
        .then(
            token(Token::Comma)
                .then(let_port_item_parser())
                .repeated()
                .collect::<Vec<(Metadata<'a>, LetPortItem<'a>)>>(),
        )
        .map(|(a, b)| LetPortList(a, b))
        .boxed()
}

pub fn let_port_item_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LetPortItem<'a>, ParserError<'a>> + Clone {
    attribute_instance_vec_parser()
        .then(let_formal_type_parser())
        .then(formal_port_identifier_parser())
        .then(
            variable_dimension_parser()
                .repeated()
                .collect::<Vec<VariableDimension<'a>>>(),
        )
        .then(token(Token::Eq).then(expression_parser()).or_not())
        .map(|((((a, b), c), d), e)| LetPortItem(a, b, c, d, e))
        .boxed()
}

pub fn let_formal_type_parser<'a>()
-> impl Parser<'a, ParserInput<'a>, LetFormalType<'a>, ParserError<'a>> + Clone {
    choice((
        data_type_or_implicit_parser().map(|a| LetFormalType::DataTypeOrImplicit(Box::new(a))),
        token(Token::Untyped).map(|a| LetFormalType::Untyped(Box::new(a))),
    ))
    .boxed()
}

pub fn let_expression_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, LetExpression<'a>, ParserError<'a>> + Clone {
    package_scope_parser()
        .or_not()
        .then(let_identifier_parser())
        .then(
            token(Token::Paren)
                .then(let_list_of_arguments_parser(expression_parser).or_not())
                .then(token(Token::EParen))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .map(|((a, b), c)| LetExpression(a, b, c))
}

pub fn let_list_of_arguments_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, LetListOfArguments<'a>, ParserError<'a>> + Clone {
    let _optional_let_actual_arg_parser = let_actual_arg_parser(expression_parser.clone()).or_not();
    let _identifier_vec_parser = token(Token::Comma)
        .then(token(Token::Period))
        .then(identifier_parser())
        .then(token(Token::Paren))
        .then(_optional_let_actual_arg_parser.clone())
        .then(token(Token::EParen))
        .map(|(((((a, b), c), d), e), f)| (a, b, c, d, e, f))
        .repeated()
        .collect::<Vec<(
            Metadata<'a>,
            Metadata<'a>,
            Identifier<'a>,
            Metadata<'a>,
            Option<LetActualArg<'a>>,
            Metadata<'a>,
        )>>();
    let _partial_identifier_parser = _optional_let_actual_arg_parser
        .clone()
        .then(
            token(Token::Comma)
                .then(_optional_let_actual_arg_parser.clone())
                .repeated()
                .collect::<Vec<(Metadata<'a>, Option<LetActualArg<'a>>)>>(),
        )
        .then(_identifier_vec_parser.clone())
        .map(|((a, b), c)| LetListOfPartialIdentifierArguments(a, b, c));
    let _identifier_parser = token(Token::Period)
        .then(identifier_parser())
        .then(token(Token::Paren))
        .then(_optional_let_actual_arg_parser)
        .then(token(Token::EParen))
        .then(_identifier_vec_parser)
        .map(|(((((a, b), c), d), e), f)| LetListOfIdentifierArguments(a, b, c, d, e, f));
    choice((
        _partial_identifier_parser.map(|a| LetListOfArguments::PartialIdentifier(Box::new(a))),
        _identifier_parser.map(|a| LetListOfArguments::Identifier(Box::new(a))),
    ))
    .boxed()
}

pub fn let_actual_arg_parser<'a>(
    expression_parser: impl Parser<'a, ParserInput<'a>, Expression<'a>, ParserError<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, LetActualArg<'a>, ParserError<'a>> + Clone {
    expression_parser.map(|a| LetActualArg(a))
}
