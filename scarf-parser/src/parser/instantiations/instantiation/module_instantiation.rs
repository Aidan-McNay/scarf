// =======================================================================
// module_instantiation.rs
// =======================================================================
// Parsing for 1800-2023 A.4.1.1

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::repeat;

pub fn name_of_instance_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<NameOfInstance<'s>, VerboseError<'s>> {
    (
        instance_identifier_parser,
        repeat(0.., unpacked_dimension_parser),
    )
        .map(|(a, b)| NameOfInstance(a, b))
        .parse_next(input)
}
