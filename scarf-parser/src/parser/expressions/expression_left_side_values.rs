// =======================================================================
// expression_left_side_values.rs
// =======================================================================
// Parsing for 1800-2023 A.8.5

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

// pub fn net_lvalue_parser<'a, I>() -> impl Parser<'a, I, NetLvalue<'a>, ParserError<'a>>
// where
//     I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
// {
//     let mut parser = Recursive::declare();
//     let _selection_net_lvalue_parser = ps_or_hierarchical_net_identifier_parser()
//         .then(constant_select_parser())
//         .map(|(a, b)| NetLvalue::Selection(Box::new(SelectionNetLvalue(a, b))));
//     let _nested_net_lvalue_parser = token(Token::Brace)
//         .then(parser.clone())
//         .then(
//             token(Token::Comma)
//                 .then(parser)
//                 .repeated()
//                 .collect::<Vec<(Metadata<'a>, NetLvalue<'a>)>>(),
//         )
//         .then(token(Token::EBrace))
//         .map(|(((a, b), c), d)| NetLvalue::Nested(Box::new(NestedNetLvalue(a, b, c, d))));
//     parser.define(choice((
//         _selection_net_lvalue_parser,
//         _nested_net_lvalue_parser,
//     )));
//     parser
// }
