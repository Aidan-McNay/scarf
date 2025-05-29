// =======================================================================
// lib.rs
// =======================================================================
// The top-level parser

mod errors;
mod general;
mod source_text;
use ariadne::{Color, Label, ReportKind};
pub use ariadne::{Report, Source};
use chumsky::prelude::*;
pub use errors::*;
pub use general::*;
use oxide_syntax::SourceText;
use source_text::source_text_parser;

pub fn parse<'a>(src: &'a str) -> ParseResult<SourceText, Rich<'a, char>> {
    source_text_parser().parse(src)
}

pub fn report_errors<'a, 'b>(
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
