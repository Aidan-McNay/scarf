// =======================================================================
// keywords.rs
// =======================================================================
// Preprocessing for keywords based on different reserved standards

use crate::Span;
use crate::*;

fn get_keyword_standard<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    err_span: Span<'s>,
) -> Result<StandardVersion, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
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
            "1364-2001" => Ok(StandardVersion::IEEE1364_2001),
            "1364-2001-noconfig" => Ok(StandardVersion::IEEE1364_2001Noconfig),
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
    dest: &mut Option<&mut Vec<SpannedToken<'s>>>,
    configs: &mut PreprocessConfigs<'s>,
    begin_span: Span<'s>,
) -> Result<(), PreprocessorError<'s>> {
    let new_standard = get_keyword_standard(src, begin_span.clone())?;
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
