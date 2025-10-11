// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the parser

pub mod behavioral_statements;
pub mod declarations;
pub mod errors;
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
use core::ops::Range;
pub use declarations::*;
pub use errors::*;
pub use expressions::*;
pub use general::*;
pub use instantiations::*;
pub(crate) use pratt::*;
pub use primitive_instances::*;
use scarf_syntax::SourceText;
pub use source_text::*;
pub use spanned_token::*;
pub use specify_section::*;
use std::fs;
pub use udp_declaration_and_instantiation::*;
pub use utils::*;
use winnow::error::ErrMode;

pub fn parse<'s>(
    input: &'s [SpannedToken<'s>],
) -> Result<SourceText<'s>, VerboseError<'s>> {
    match source_text_parser.parse_next(&mut TokenSlice::new(input)) {
        Ok(source_text) => Ok(source_text),
        Err(ErrMode::Backtrack(err)) => Err(err),
        Err(ErrMode::Cut(err)) => Err(err),
        Err(ErrMode::Incomplete(_)) => {
            panic!("Produced 'incomplete', an unsupported error")
        }
    }
}

fn format_expectation<'s>(pattern: &Expectation<'s>) -> String {
    match pattern {
        Expectation::Token(token) => token.to_string(),
        Expectation::Label(label) => label.to_string(),
        Expectation::EOI => "end of input".to_string(),
    }
}

fn format_reason<'s>(error: &VerboseError<'s>) -> String {
    let found_str = match error.found {
        Some(tok) => tok.to_string(),
        None => "end of input".to_owned(),
    };
    let mut dedup_expected: Vec<Expectation<'s>> = vec![];
    for expected in error.expected.iter() {
        if !dedup_expected.contains(expected) {
            dedup_expected.push(expected.clone());
        }
    }
    let expected_str = match &dedup_expected[..] {
        [] => "something else".to_owned(),
        [expected] => format_expectation(expected),
        _ => {
            let mut temp_expected_str = String::new();
            for expected in &dedup_expected[..dedup_expected.len() - 1] {
                temp_expected_str
                    .push_str(format_expectation(expected).as_str());
                temp_expected_str.push_str(", ");
            }
            temp_expected_str.push_str("or ");
            temp_expected_str.push_str(
                format_expectation(dedup_expected.last().unwrap()).as_str(),
            );
            temp_expected_str
        }
    };
    format!("found {}, expected {}", found_str, expected_str)
}

fn format_reason_short<'s>(error: &VerboseError<'s>) -> String {
    match error.found {
        Some(tok) => format!("Didn't expect {}", tok.to_string()),
        None => "Didn't expect end of input".to_owned(),
    }
}

pub fn report_parse_errors<'s, 'b>(
    result: &Result<SourceText<'s>, VerboseError<'s>>,
    file_path: &'b str,
) -> Vec<Report<'s, (&'b str, std::ops::Range<usize>)>> {
    let mut reports: Vec<Report<'s, (&'b str, std::ops::Range<usize>)>> =
        Vec::new();
    if let &Err(ref verbose_error) = result {
        let error_span = if verbose_error.is_eoi() {
            let file_len = fs::metadata(file_path).expect("REASON").len();
            Range {
                start: file_len as usize,
                end: file_len as usize,
            }
        } else {
            verbose_error.span.clone()
        };
        let report =
            Report::build(ReportKind::Error, (file_path, error_span.clone()))
                .with_code("P1")
                .with_config(
                    ariadne::Config::new()
                        .with_index_type(ariadne::IndexType::Byte),
                )
                .with_message(format_reason(verbose_error))
                .with_label(
                    Label::new((file_path, error_span))
                        .with_message(format_reason_short(verbose_error))
                        .with_color(Color::Red),
                )
                .finish();
        reports.push(report);
    }
    reports
}
