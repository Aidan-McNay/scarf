// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

pub mod declarations;
pub mod errors;
pub mod expressions;
pub mod general;
pub mod source_text;
pub mod udp_declaration_and_instantiation;
pub mod utils;
use ariadne::Report;
use ariadne::{Color, Label, ReportKind};
pub use chumsky::prelude::*;
pub use cirkit_syntax::SourceText;
pub use declarations::*;
pub use errors::*;
pub use expressions::*;
pub use general::*;
pub use source_text::*;
pub use udp_declaration_and_instantiation::*;
pub use utils::*;

pub fn parse<'a>(src: &'a str) -> ParseResult<SourceText, Rich<'a, char>> {
    source_text_parser().parse(src)
}

pub fn report_parse_errors<'a, 'b>(
    result: ParseResult<SourceText, Rich<'a, char>>,
    file_path: &'b str,
) -> Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> {
    let mut reports: Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> = Vec::new();
    result.into_errors().into_iter().for_each(|e| {
        let report = Report::build(ReportKind::Error, (file_path, e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new((file_path, e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish();
        reports.push(report);
    });
    reports
}
