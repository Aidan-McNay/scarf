// =======================================================================
// system_verilog_source_text.rs
// =======================================================================
// Parsing for 1800-2023 A.1.2

use crate::*;
use chumsky::prelude::*;
use oxide_syntax::*;

pub fn source_text_parser<'a>() -> impl Parser<'a, &'a str, SourceText, ParserError<'a>> {
    just("test")
        .to(SourceText(None, Vec::new()))
        .recover_with(via_parser(end().to(SourceText(None, Vec::new()))))
}
