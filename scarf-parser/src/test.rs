// =======================================================================
// test.rs
// =======================================================================
// Utilities for testing parsers

use scarf_syntax::*;

#[macro_export]
macro_rules! check_preprocessor {
    ($input:literal, $expected:expr) => {{
        let input = lex_to_parse_stream(lex($input, "<test>", None))
            .collect::<Vec<_>>();
        let string_cache = PreprocessorCache::default();
        let mut configs = PreprocessConfigs::new(&string_cache);
        let mut preprocessed_stream: Vec<SpannedToken<'_>> = vec![];
        let preprocess_result = preprocess(
            &mut TokenIterator::new(input.into_iter()),
            &mut preprocessed_stream,
            &mut configs,
        );
        match preprocess_result {
            Ok(()) => assert_eq!(preprocessed_stream, $expected),
            Err(err) => panic!("{:?}", err),
        }
    }};
}

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
