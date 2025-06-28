// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

pub mod behavioral_statements;
pub mod declarations;
pub mod errors;
pub mod expressions;
pub mod general;
pub mod primitive_instances;
pub mod source_text;
pub mod udp_declaration_and_instantiation;
pub mod utils;
use crate::*;
use ariadne::Report;
use ariadne::{Color, Label, ReportKind};
pub use behavioral_statements::*;
use chumsky::error::{RichPattern, RichReason};
use chumsky::input::{BoxedStream, MappedInput};
use chumsky::prelude::*;
pub use declarations::*;
pub use errors::*;
pub use expressions::*;
pub use general::*;
pub use primitive_instances::*;
pub use scarf_syntax::SourceText;
pub use source_text::*;
pub use udp_declaration_and_instantiation::*;
pub use utils::*;

pub type ParserSpan = SimpleSpan;
// Hard-code input type to be able to cache parsers
pub type ParserInput<'a> = MappedInput<
    Token<'a>,
    ParserSpan,
    BoxedStream<'a, (Token<'a>, SimpleSpan)>,
    fn((Token<'a>, ParserSpan)) -> (Token<'a>, ParserSpan),
>;

pub fn parse<'a>(src: ParserInput<'a>) -> ParseResult<SourceText<'a>, Rich<'a, Token<'a>>> {
    source_text_parser().parse(src)
}

fn format_pattern<'a>(pattern: &RichPattern<'a, Token<'a>>) -> String {
    match pattern {
        RichPattern::Token(tok) => tok.to_string(),
        RichPattern::Label(l) => l.to_string(),
        RichPattern::Identifier(i) => i.to_string(),
        RichPattern::Any => "any".to_owned(),
        RichPattern::SomethingElse => "something else".to_owned(),
        RichPattern::EndOfInput => "end of input".to_owned(),
    }
}

fn format_reason<'a>(reason: &RichReason<'a, Token<'a>>) -> String {
    match reason {
        RichReason::ExpectedFound { expected, found } => {
            let found_str = match found.as_deref() {
                Some(tok) => tok.to_string(),
                None => "end of input".to_owned(),
            };
            let expected_str = match &expected[..] {
                [] => "something else".to_owned(),
                [expected] => format_pattern(expected),
                _ => {
                    let mut temp_expected_str = String::new();
                    for expected in &expected[..expected.len() - 1] {
                        temp_expected_str.push_str(format_pattern(expected).as_str());
                        temp_expected_str.push_str(", ");
                    }
                    temp_expected_str.push_str("or ");
                    temp_expected_str.push_str(format_pattern(expected.last().unwrap()).as_str());
                    temp_expected_str
                }
            };
            format!("found {}, expected {}", found_str, expected_str)
        }
        RichReason::Custom(str) => str.clone(),
    }
}

fn format_reason_short<'a>(reason: &RichReason<'a, Token<'a>>) -> String {
    match reason {
        RichReason::ExpectedFound { expected: _, found } => match found.as_deref() {
            Some(tok) => format!("Didn't expect {}", tok.to_string()),
            None => "Didn't expect end of input".to_owned(),
        },
        RichReason::Custom(str) => str.clone(),
    }
}

pub fn report_parse_errors<'a, 'b>(
    result: ParseResult<SourceText, Rich<'a, Token<'a>, ParserSpan>>,
    file_path: &'b str,
) -> Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> {
    let mut reports: Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> = Vec::new();
    result.into_errors().into_iter().for_each(|e| {
        let report = Report::build(ReportKind::Error, (file_path, e.span().into_range()))
            .with_code("P1")
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(format_reason(e.reason()))
            .with_label(
                Label::new((file_path, e.span().into_range()))
                    .with_message(format_reason_short(e.reason()))
                    .with_color(Color::Red),
            )
            .finish();
        reports.push(report);
    });
    reports
}
