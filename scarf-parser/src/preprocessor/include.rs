// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;
use std::iter::Peekable;

pub enum IncludePath<'a> {
    ProjectRelative(&'a str),
    ToolRelative(&'a str),
}

fn get_include_path<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    define_span: Span<'s>,
) -> Result<(IncludePath<'s>, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        Token::StringLiteral(id_str) => {
            Ok((IncludePath::ProjectRelative(id_str), spanned_token.1))
        }
        Token::DirIncludeToolPath(id_str) => {
            Ok((IncludePath::ToolRelative(id_str), spanned_token.1))
        }
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![Expectation::Label("an include path")],
        })),
    }
}

pub fn preprocess_include<'s>(
    src: &mut Peekable<impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    include_span: &'s Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let (include_path_text, _) = get_include_path(src, include_span.clone())?;
    // Treat both include types as the same
    let include_path_text = match include_path_text {
        IncludePath::ProjectRelative(text) => text,
        IncludePath::ToolRelative(text) => text,
    };
    let include_path = configs.get_file_path(include_path_text).unwrap();
    let included_file = std::fs::read_to_string(&include_path).unwrap();
    let (include_path, included_file) = configs
        .retain_file(include_path.to_str().unwrap().to_owned(), included_file);
    let included_file_contents = lex_to_parse_stream(lex(
        included_file,
        include_path,
        Some(include_span),
    ));
    dest.reserve(included_file_contents.len());
    preprocess(
        &mut included_file_contents.into_iter().peekable(),
        dest,
        configs,
    )
}
