// =======================================================================
// test.rs
// =======================================================================
// Utilities for testing parsers

use crate::*;
use winnow::{ModalResult, Parser, error::ErrMode};

pub fn check_parser<'s, T: std::cmp::PartialEq + std::fmt::Debug>(
    mut parser: impl FnMut(
        &mut Tokens<'_>,
    ) -> ModalResult<T, ErrMode<VerboseError<'s>>>,
    input: &'s str,
    expected: T,
) {
    let parser_stream = lex_to_parse_stream(lex(input, "<test>"));
    let mut stateful_parser_input = Tokens {
        input: TokenSlice::new(&parser_stream[..]),
        state: VerboseError::default(),
    };
    let result = parser.parse_next(&mut stateful_parser_input);
    match result {
        Ok(parsed) => assert_eq!(parsed, expected),
        Err(error) => panic!("{}", error),
    }
}

pub fn apply_parser<'s, T>(
    mut parser: impl FnMut(
        &mut Tokens<'_>,
    ) -> ModalResult<T, ErrMode<VerboseError<'s>>>,
    input: &'s str,
) -> T {
    let parser_stream = lex_to_parse_stream(lex(input, "<test>"));
    let mut stateful_parser_input = Tokens {
        input: TokenSlice::new(&parser_stream[..]),
        state: VerboseError::default(),
    };
    parser.parse_next(&mut stateful_parser_input).unwrap()
}
