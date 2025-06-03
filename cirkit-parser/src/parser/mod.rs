// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

// pub mod declarations;
pub mod errors;
pub mod expressions;
// pub mod general;
// pub mod source_text;
// pub mod udp_declaration_and_instantiation;
pub mod utils;
use crate::*;
use ariadne::Report;
use ariadne::{Color, Label, ReportKind};
use chumsky::error::{RichPattern, RichReason};
use chumsky::input::ValueInput;
use chumsky::prelude::*;
pub use cirkit_syntax::SourceText;
// pub use declarations::*;
pub use errors::*;
pub use expressions::*;
// pub use general::*;
// pub use source_text::*;
// pub use udp_declaration_and_instantiation::*;
pub use utils::*;

pub type ParserSpan = SimpleSpan;

pub fn trivial_parser<'a, I>() -> impl Parser<'a, I, SourceText<'a>, ParserError<'a>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    select! {
        Token::Module => SourceText(None, Vec::new())
    }
}

pub fn parse<'a, I>(src: I) -> ParseResult<SourceText<'a>, Rich<'a, Token<'a>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = ParserSpan>,
{
    trivial_parser().parse(src)
}

pub fn format_pattern<'a>(pattern: &RichPattern<'a, Token<'a>>) -> String {
    match pattern {
        RichPattern::Token(tok) => tok.to_string(),
        RichPattern::Label(l) => l.to_string(),
        RichPattern::Identifier(i) => i.to_string(),
        RichPattern::Any => "any".to_owned(),
        RichPattern::SomethingElse => "something else".to_owned(),
        RichPattern::EndOfInput => "end of input".to_owned(),
    }
}

pub fn format_reason<'a>(reason: &RichReason<'a, Token<'a>>) -> String {
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
                    temp_expected_str.push_str(" or ");
                    temp_expected_str.push_str(format_pattern(expected.last().unwrap()).as_str());
                    temp_expected_str
                }
            };
            format!("found {}, expected {}", found_str, expected_str)
        }
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
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(format_reason(e.reason()))
            .with_label(
                Label::new((file_path, e.span().into_range()))
                    .with_message(format_reason(e.reason()))
                    .with_color(Color::Red),
            )
            .finish();
        reports.push(report);
    });
    reports
}
