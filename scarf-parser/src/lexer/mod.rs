// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the lexer

pub mod callbacks;
pub mod keywords;
pub mod merge;
pub mod tokens;
use ariadne::Report;
pub use ariadne::{Color, Label, ReportKind};
pub use callbacks::*;
pub use keywords::*;
pub use logos::Logos;
pub use logos::Span as ByteSpan;
use merge::TokenMerge;
use scarf_syntax::Span;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;
pub use tokens::Token;

pub fn token_span_mapper<'a>(
    file_name: &'a str,
) -> impl Fn(
    (Result<Token<'a>, String>, ByteSpan),
) -> (Result<Token<'a>, String>, Span<'a>) {
    move |(token_result, byte_span)| {
        (
            token_result,
            Span {
                file: file_name,
                bytes: byte_span,
                included_from: None,
            },
        )
    }
}

pub fn lex<'a>(
    src: &'a str,
    file_name: &'a str,
) -> Vec<(Result<Token<'a>, String>, Span<'a>)> {
    let span_mapper = token_span_mapper(file_name);
    TokenMerge::new(Token::lexer(src), src)
        .map(span_mapper)
        .collect()
}

pub fn report_lex_errors<'a>(
    result: &Vec<(Result<Token<'a>, String>, Span<'a>)>,
) -> Vec<Report<'a, (&'a str, ByteSpan)>> {
    let mut reports: Vec<Report<'a, (&'a str, ByteSpan)>> = Vec::new();
    for (result, span) in result {
        if let &Err(ref text) = result {
            let report = if text.len() == 0 {
                Report::build(
                    ReportKind::Error,
                    (span.file, span.bytes.clone()),
                )
                .with_code("L1")
                .with_config(
                    ariadne::Config::new()
                        .with_index_type(ariadne::IndexType::Byte),
                )
                .with_message("Unrecognized token")
                .with_label(
                    Label::new((span.file, span.bytes.clone()))
                        .with_message("Unrecognized token")
                        .with_color(Color::Red),
                )
                .finish()
            } else {
                Report::build(
                    ReportKind::Error,
                    (span.file, span.bytes.clone()),
                )
                .with_code("L1")
                .with_config(
                    ariadne::Config::new()
                        .with_index_type(ariadne::IndexType::Byte),
                )
                .with_message(text.clone())
                .with_label(
                    Label::new((span.file, span.bytes.clone()))
                        .with_message(text)
                        .with_color(Color::Red),
                )
                .finish()
            };
            reports.push(report);
        }
    }
    reports
}

pub fn dump_lex<'a>(
    lex_stream: &Vec<(Result<Token<'a>, String>, Span<'a>)>,
    file_path: &str,
) -> io::Result<()> {
    let file_path = Path::new(file_path);
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    for (result, span) in lex_stream {
        let dump_str = format!(
            "[{:>2}:{:>2}] {}\n",
            span.bytes.start,
            span.bytes.end,
            match result {
                Ok(token) => token,
                Err(_) => &Token::Error,
            }
        );
        writer.write_all(dump_str.as_bytes())?;
    }
    writer.flush()?;
    Ok(())
}
