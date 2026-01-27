// =======================================================================
// test.rs
// =======================================================================
// Utilities for testing parsers

use scarf_syntax::*;

#[macro_export]
macro_rules! apply_parser {
    ($input:literal, $parser:ident, $storage:expr) => {{
        *$storage = lex_to_parse_stream(lex($input, "<test>", None))
            .collect::<Vec<_>>();
        let mut tokens = Tokens {
            input: TokenSlice::new(&$storage[..]),
            state: VerboseError::default(),
        };
        $parser(&mut tokens).unwrap()
    }};
}

#[macro_export]
macro_rules! check_parser {
    ($input:literal, $parser:ident, $expected:expr) => {{
        let input = lex_to_parse_stream(lex($input, "<test>", None))
            .collect::<Vec<_>>();
        let mut tokens = Tokens {
            input: TokenSlice::new(&input[..]),
            state: VerboseError::default(),
        };
        assert_eq!($parser(&mut tokens).unwrap(), $expected)
    }};
}

pub fn test_metadata<'s>() -> Metadata<'s> {
    Metadata::new(Span::default(), vec![])
}
