// =======================================================================
// subroutine_call_statements.rs
// =======================================================================
// Parsing for 1800-2023 A.6.9

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;

pub fn subroutine_call_statement_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SubroutineCallStatement<'s>, VerboseError<'s>> {
    let _subroutine_parser = (subroutine_call_parser, token(Token::SColon))
        .map(|(a, b)| SubroutineCallStatement::Subroutine(Box::new((a, b))));
    let _void_parser = (
        token(Token::Void),
        token(Token::Apost),
        token(Token::Paren),
        function_subroutine_call_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c, d, e)| {
            SubroutineCallStatement::Void(Box::new((a, b, c, d, e)))
        });
    alt((_subroutine_parser, _void_parser)).parse_next(input)
}
