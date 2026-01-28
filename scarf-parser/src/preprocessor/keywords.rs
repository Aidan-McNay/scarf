// =======================================================================
// keywords.rs
// =======================================================================
// Preprocessing for keywords based on different reserved standards

use crate::Span;
use crate::*;

fn get_keyword_standard<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    err_span: Span<'s>,
) -> Result<StandardVersion, PreprocessorError<'s>> {
    let Some(spanned_token) = preprocess_single(src, configs)? else {
        return Err(PreprocessorError::IncompleteDirective(err_span));
    };
    match spanned_token.0 {
        Token::StringLiteral(version_spec) => match version_spec {
            "1800-2023" => Ok(StandardVersion::IEEE1800_2023),
            "1800-2017" => Ok(StandardVersion::IEEE1800_2017),
            "1800-2012" => Ok(StandardVersion::IEEE1800_2012),
            "1800-2009" => Ok(StandardVersion::IEEE1800_2009),
            "1800-2005" => Ok(StandardVersion::IEEE1800_2005),
            "1364-2005" => Ok(StandardVersion::IEEE1364_2005),
            "1364-2001-noconfig" => Ok(StandardVersion::IEEE1364_2001Noconfig),
            "1364-2001" => Ok(StandardVersion::IEEE1364_2001),
            "1364-1995" => Ok(StandardVersion::IEEE1364_1995),
            _ => Err(PreprocessorError::InvalidVersionSpecifier((
                version_spec,
                spanned_token.1,
            ))),
        },
        _ => Err(PreprocessorError::IncompleteDirectiveWithToken(
            spanned_token,
        )),
    }
}

pub fn preprocess_keyword_standard<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    configs: &mut PreprocessConfigs<'s>,
    begin_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let new_standard = get_keyword_standard(src, configs, begin_span.clone())?;
    let old_standard = configs.curr_standard.clone();
    configs.curr_standard = new_standard;
    let result = preprocess(src, dest, configs);
    configs.curr_standard = old_standard;
    match result {
        Ok(()) => Err(PreprocessorError::NoEndKeywords(begin_span)),
        Err(PreprocessorError::EndKeywords(_)) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
const KEYWORDS: &str = "
an_identifier
integer
automatic
design
uwire
illegal_bins
eventually
implements
";

#[test]
fn standard_1364_1995() {
    let input = "`begin_keywords \"1364-1995\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::SimpleIdentifier("automatic"),
            Token::SimpleIdentifier("design"),
            Token::SimpleIdentifier("uwire"),
            Token::SimpleIdentifier("illegal_bins"),
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1364_2001() {
    let input = "`begin_keywords \"1364-2001\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::SimpleIdentifier("uwire"),
            Token::SimpleIdentifier("illegal_bins"),
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1364_2001_noconfig() {
    let input = "`begin_keywords \"1364-2001-noconfig\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::SimpleIdentifier("design"),
            Token::SimpleIdentifier("uwire"),
            Token::SimpleIdentifier("illegal_bins"),
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1364_2005() {
    let input = "`begin_keywords \"1364-2005\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::SimpleIdentifier("illegal_bins"),
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1800_2005() {
    let input = "`begin_keywords \"1800-2005\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1800_2009() {
    let input = "`begin_keywords \"1800-2009\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::Eventually,
            Token::SimpleIdentifier("implements")
        ]
    )
}

#[test]
fn standard_1800_2012() {
    let input = "`begin_keywords \"1800-2012\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::Eventually,
            Token::Implements
        ]
    )
}

#[test]
fn standard_1800_2017() {
    let input = "`begin_keywords \"1800-2017\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::Eventually,
            Token::Implements
        ]
    )
}

#[test]
fn standard_1800_2023() {
    let input = "`begin_keywords \"1800-2023\"".to_string()
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::Eventually,
            Token::Implements
        ]
    )
}

#[test]
#[should_panic]
fn missing_end_keywords() {
    check_preprocessor!(
        "`begin_keywords \"1800-2023\"",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic]
fn invalid_version_token() {
    check_preprocessor!(
        "`begin_keywords module
        `end_keywords",
        Vec::<Token<'_>>::new()
    )
}

#[test]
#[should_panic]
fn invalid_version_string() {
    check_preprocessor!(
        "`begin_keywords \"1985\"
        `end_keywords",
        Vec::<Token<'_>>::new()
    )
}

#[test]
fn nested() {
    let input = "`begin_keywords \"1800-2023\"".to_string()
        + "`begin_keywords \"1800-2005\""
        + KEYWORDS
        + "`begin_keywords \"1364-1995\""
        + KEYWORDS
        + "`end_keywords"
        + "`end_keywords"
        + KEYWORDS
        + "`end_keywords";
    check_preprocessor!(
        input.as_str(),
        vec![
            // 1800-2005
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements"),
            // 1364-1995
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::SimpleIdentifier("automatic"),
            Token::SimpleIdentifier("design"),
            Token::SimpleIdentifier("uwire"),
            Token::SimpleIdentifier("illegal_bins"),
            Token::SimpleIdentifier("eventually"),
            Token::SimpleIdentifier("implements"),
            // 1800-2023
            Token::SimpleIdentifier("an_identifier"),
            Token::Integer,
            Token::Automatic,
            Token::Design,
            Token::Uwire,
            Token::IllegalBins,
            Token::Eventually,
            Token::Implements
        ]
    )
}
