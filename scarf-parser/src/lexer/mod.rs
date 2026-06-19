// =======================================================================
// mod.rs
// =======================================================================
//! Lexing a source file into semantic tokens

pub(crate) mod callbacks;
pub(crate) mod keywords;
pub(crate) mod tokens;
use crate::SpannedToken;
use ariadne::Report;
use ariadne::{Color, Label, ReportKind};
pub use keywords::StandardVersion;
use logos::Logos;
use logos::Span as ByteSpan;
use scarf_syntax::Span;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;
pub use tokens::Token;

fn report_lex_result<'a>(
    lex_result: (Result<Token<'a>, String>, Span<'a>),
) -> Option<Report<'a, (&'a str, ByteSpan)>> {
    let (result, span) = lex_result;
    if let Err(ref text) = result {
        let report = if text.len() == 0 {
            Report::build(ReportKind::Error, (span.file, span.bytes.clone()))
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
            Report::build(ReportKind::Error, (span.file, span.bytes.clone()))
                .with_code("L2")
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
        Some(report)
    } else {
        None
    }
}

fn map_lex_result<'a>(
    lex_result: (Result<Token<'a>, String>, Span<'a>),
) -> SpannedToken<'a> {
    match lex_result.0 {
        Ok(tok) => SpannedToken(tok, lex_result.1),
        Err(_) => SpannedToken(Token::Error, lex_result.1),
    }
}

/// An iterator over syntactical tokens for a SystemVerilog source
pub trait LexedSource<'a>:
    Iterator<Item = (Result<Token<'a>, String>, Span<'a>)> + Clone
{
    /// Generate error reports for any errors encountered in lexing
    fn report_errors(
        &self,
    ) -> impl Iterator<Item = Report<'a, (&'a str, ByteSpan)>> {
        self.clone().filter_map(report_lex_result)
    }

    /// Dump a representation of the lexed source to a file, for debugging
    fn dump(&self, file_path: &Path) -> io::Result<()> {
        if let Some(parent_dir) = file_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);
        for (result, span) in self.clone() {
            let dump_str = format!(
                "[{:>2}:{:>2}] {}\n",
                span.bytes.start,
                span.bytes.end,
                match result {
                    Ok(token) => token,
                    Err(_) => Token::Error,
                }
            );
            writer.write_all(dump_str.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Process the lexing of the source, storing the result
    ///
    /// While this does incur memory overhead, it avoid processing
    /// the source multiple times if cloned.
    fn process(
        self,
    ) -> std::vec::IntoIter<(Result<Token<'a>, String>, Span<'a>)> {
        self.collect::<Vec<_>>().into_iter()
    }

    /// Convert lexer results into tokens, turning errors into [`Token::Error`]
    fn tokens(self) -> impl Iterator<Item = SpannedToken<'a>> {
        self.into_iter().map(map_lex_result)
    }
}
impl<'a, T> LexedSource<'a> for T where
    T: Iterator<Item = (Result<Token<'a>, String>, Span<'a>)> + Clone
{
}

fn token_span_mapper<'a>(
    file_name: &'a str,
    included_from: Option<&'a Span<'a>>,
) -> impl Fn(
    (Result<Token<'a>, String>, ByteSpan),
) -> (Result<Token<'a>, String>, Span<'a>)
+ Clone {
    move |(token_result, byte_span)| {
        (
            token_result,
            Span {
                file: file_name,
                bytes: byte_span,
                expanded_from: None,
                included_from,
            },
        )
    }
}

pub(crate) fn lex_helper<'a>(
    src: &'a str,
    file_name: &'a str,
    included_from: Option<&'a Span<'a>>,
) -> impl LexedSource<'a> {
    let span_mapper = token_span_mapper(file_name, included_from);
    Token::lexer(src).spanned().map(span_mapper)
}

/// Separate a source file into syntactic tokens
///
/// ```rust
/// # use scarf_parser::*;
/// let file_contents = "module test_module; endmodule";
/// let mut tokens = lex(file_contents, "test_file.v");
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::Module), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::SimpleIdentifier("test_module")), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::SColon), _)));
/// assert!(matches!(tokens.next().unwrap(), (Ok(Token::Endmodule), _)));
/// assert!(tokens.next().is_none());
/// ```
///
/// If the lexer encounters an error, the resulting `Err(string)` may contain
/// more information if possible to discern.
pub fn lex<'a>(src: &'a str, file_name: &'a str) -> impl LexedSource<'a> {
    lex_helper(src, file_name, None)
}
