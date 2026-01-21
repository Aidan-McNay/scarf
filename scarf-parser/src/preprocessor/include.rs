// =======================================================================
// define.rs
// =======================================================================
// Preprocessing for preprocessor definitions

use crate::Span;
use crate::*;

pub enum IncludePath<'a> {
    ProjectRelative(&'a str),
    ToolRelative(&'a str),
}

fn get_include_path<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    define_span: Span<'s>,
) -> Result<(IncludePath<'s>, Span<'s>), PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, configs)? else {
        return Err(PreprocessorError::IncompleteDirective(define_span));
    };
    match spanned_token.0 {
        Token::StringLiteral(id_str) => {
            Ok((IncludePath::ProjectRelative(id_str), spanned_token.1))
        }
        Token::Lt => loop {
            let Some(next_token) = preprocess_single(src, configs)? else {
                break Err(PreprocessorError::Error(VerboseError {
                    valid: true,
                    span: spanned_token.1,
                    found: Some(spanned_token.0),
                    expected: vec![Expectation::Label("an include path")],
                }));
            };
            match next_token.0 {
                Token::Newline => {
                    break Err(PreprocessorError::Error(VerboseError {
                        valid: true,
                        span: spanned_token.1,
                        found: Some(spanned_token.0),
                        expected: vec![Expectation::Label("an include path")],
                    }));
                }
                Token::Gt => {
                    let mut path_span = spanned_token.1.clone();
                    path_span.bytes.start = spanned_token.1.bytes.end;
                    path_span.bytes.end = next_token.1.bytes.start;
                    let path = configs.get_slice(&path_span).unwrap();
                    let mut overall_span = next_token.1;
                    overall_span.bytes.start = spanned_token.1.bytes.start;
                    break Ok((IncludePath::ToolRelative(path), overall_span));
                }
                _ => (),
            }
        },
        _ => Err(PreprocessorError::Error(VerboseError {
            valid: true,
            span: spanned_token.1,
            found: Some(spanned_token.0),
            expected: vec![Expectation::Label("an include path")],
        })),
    }
}

pub fn preprocess_include<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    configs: &mut PreprocessConfigs<'s>,
    include_span: &'s Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let (include_path_text, _) =
        get_include_path(src, configs, include_span.clone())?;
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
    if let Some(size_hint) = included_file_contents.size_hint().1 {
        dest.reserve(size_hint);
    }
    preprocess(
        &mut TokenIterator::new(included_file_contents),
        dest,
        configs,
    )
}
