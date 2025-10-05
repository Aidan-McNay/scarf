// =======================================================================
// mod.rs
// =======================================================================
// The top-level interface for the lexer

pub mod callbacks;
pub mod keywords;
pub mod postprocess;
pub mod tokens;
use ariadne::Report;
pub use ariadne::{Color, Label, ReportKind};
pub use callbacks::*;
pub use keywords::*;
pub use logos::{Logos, Span};
pub use postprocess::postprocess;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;
pub use tokens::Token;

pub fn lex<'a>(src: &'a str) -> Vec<(Result<Token<'a>, String>, Span)> {
    let stream = postprocess(Token::lexer(src).spanned().collect(), src);
    stream
}

pub fn report_lex_errors<'a, 'b>(
    result: &Vec<(Result<Token<'a>, String>, Span)>,
    file_path: &'b str,
) -> Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> {
    let mut reports: Vec<Report<'a, (&'b str, std::ops::Range<usize>)>> =
        Vec::new();
    for (result, span) in result {
        if let &Err(ref text) = result {
            let report = if text.len() == 0 {
                Report::build(ReportKind::Error, (file_path, span.clone()))
                    .with_code("L1")
                    .with_config(
                        ariadne::Config::new()
                            .with_index_type(ariadne::IndexType::Byte),
                    )
                    .with_message("Unrecognized token")
                    .with_label(
                        Label::new((file_path, span.clone()))
                            .with_message("Unrecognized token")
                            .with_color(Color::Red),
                    )
                    .finish()
            } else {
                Report::build(ReportKind::Error, (file_path, span.clone()))
                    .with_code("L1")
                    .with_config(
                        ariadne::Config::new()
                            .with_index_type(ariadne::IndexType::Byte),
                    )
                    .with_message(text.clone())
                    .with_label(
                        Label::new((file_path, span.clone()))
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
    lex_stream: &Vec<(Result<Token<'a>, String>, Span)>,
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
            span.start,
            span.end,
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
