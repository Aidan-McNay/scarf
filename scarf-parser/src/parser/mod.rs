// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

pub mod behavioral_statements;
pub mod combinators;
pub mod declarations;
pub mod error;
pub mod expressions;
pub mod general;
pub mod instantiations;
pub mod pratt;
pub mod primitive_instances;
pub mod source_text;
pub mod spanned_token;
pub mod specify_section;
pub mod udp_declaration_and_instantiation;
pub mod utils;
use crate::*;
pub use behavioral_statements::*;
pub use combinators::*;
pub use declarations::*;
pub use error::*;
pub use expressions::*;
pub use general::*;
pub use instantiations::*;
pub(crate) use pratt::*;
pub use primitive_instances::*;
use scarf_syntax::*;
pub use source_text::*;
pub use spanned_token::*;
pub use specify_section::*;
pub use udp_declaration_and_instantiation::*;
pub use utils::*;
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
