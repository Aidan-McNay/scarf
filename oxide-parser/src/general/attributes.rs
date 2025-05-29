// =======================================================================
// attributes.rs
// =======================================================================
// Parsing for 1800-2023 A.9.1

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;
use std::iter;

pub fn attribute_instance_parser<'a>()
-> impl Parser<'a, &'a str, AttributeInstance, ParserError<'a>> {
    attr_spec_parser()
        .map(|a| iter::once(a).collect())
        .foldl(
            sep()
                .ignore_then(just(',').ignore_then(attr_spec_parser()))
                .repeated(),
            foldl_vector,
        )
        .map(|a| AttributeInstance(a))
}

pub fn attr_spec_parser<'a>() -> impl Parser<'a, &'a str, AttrSpec, ParserError<'a>> {
    let assignment_parser = just('=')
        .ignore_then(text::whitespace())
        .ignore_then(constant_expression_parser());
    attr_name_parser()
        .then_ignore(sep())
        .then(assignment_parser.or_not())
        .map(|(a, b)| AttrSpec(a, b))
}

pub fn attr_name_parser<'a>() -> impl Parser<'a, &'a str, AttrName, ParserError<'a>> {
    identifier_parser().map(|a| AttrName(a))
}
