// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

pub(crate) mod behavioral_statements;
pub(crate) mod combinators;
pub(crate) mod declarations;
pub(crate) mod error;
pub(crate) mod expressions;
pub(crate) mod general;
pub(crate) mod instantiations;
pub(crate) mod pratt;
pub(crate) mod primitive_instances;
pub(crate) mod source_text;
pub(crate) mod spanned_token;
pub(crate) mod specify_section;
pub(crate) mod udp_declaration_and_instantiation;
pub(crate) mod utils;
use crate::*;
pub(crate) use behavioral_statements::*;
pub(crate) use combinators::*;
pub(crate) use declarations::*;
pub use error::*;
pub(crate) use expressions::*;
pub(crate) use general::*;
pub(crate) use instantiations::*;
pub(crate) use pratt::*;
pub(crate) use primitive_instances::*;
use scarf_syntax::*;
pub(crate) use source_text::*;
pub use spanned_token::*;
pub(crate) use specify_section::*;
pub(crate) use udp_declaration_and_instantiation::*;
pub(crate) use utils::*;
use winnow::error::{ErrMode, ParserError};

pub fn parse<'s>(
    input: &'s [SpannedToken<'s>],
) -> Result<SourceText<'s>, VerboseError<'s>> {
    let mut stateful_input = Tokens {
        input: TokenSlice::new(input),
        state: VerboseError::default(),
    };
    match source_text_parser.parse_next(&mut stateful_input) {
        Ok(source_text) => Ok(source_text),
        Err(ErrMode::Backtrack(err)) => Err(err.or(stateful_input.state)),
        Err(ErrMode::Cut(err)) => Err(err),
        Err(ErrMode::Incomplete(_)) => {
            panic!("Produced 'incomplete', an unsupported error")
        }
    }
}
