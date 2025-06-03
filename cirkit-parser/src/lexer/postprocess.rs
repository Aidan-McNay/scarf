// =======================================================================
// postprocess.rs
// =======================================================================
// Postprocessing to perform on a sequence of lexed tokens

use crate::*;
use logos::Span;

// -----------------------------------------------------------------------
// block_comment_merge_postprocess
// -----------------------------------------------------------------------
// Merge content into a single BlockComment based on the start and end
// delimiters

pub fn block_comment_merge_postprocess<'a>(
    stream: Vec<(Result<Token<'a>, String>, Span)>,
    src: &'a str,
) -> Vec<(Result<Token<'a>, String>, Span)> {
    let mut block_comment_started = false;
    let mut block_comment_start_span = Span::default();
    let mut new_vec: Vec<(Result<Token<'a>, String>, Span)> = Vec::new();
    for chunk in stream {
        match chunk {
            (Ok(Token::BlockCommentStart), start_span) => {
                if !block_comment_started {
                    block_comment_started = true;
                    block_comment_start_span = start_span.clone();
                }
            }
            (Ok(Token::BlockCommentEnd), end_span) => {
                if !block_comment_started {
                    new_vec.push((
                        Err("Ending block comment without beginning".to_owned()),
                        end_span,
                    ))
                } else {
                    let comment_span = Span {
                        start: block_comment_start_span.start,
                        end: end_span.end,
                    };
                    let text_span = Span {
                        start: block_comment_start_span.end,
                        end: end_span.start,
                    };
                    let comment_text = &src[text_span.start..text_span.end];
                    new_vec.push((Ok(Token::BlockComment(comment_text)), comment_span));
                    block_comment_started = false;
                }
            }
            _ => {
                if !block_comment_started {
                    new_vec.push(chunk)
                }
            }
        }
    }
    if block_comment_started {
        new_vec.push((
            Err("Block comment with no ending".to_owned()),
            block_comment_start_span,
        ))
    }
    new_vec
}

// -----------------------------------------------------------------------
// keyword_postprocess
// -----------------------------------------------------------------------
// Turns keywords into identifiers based on which keywords are reserved
// based on the current standard

pub fn keyword_postprocess<'a>(stream: &mut Vec<(Result<Token<'a>, String>, Span)>, _: &'a str) {
    let mut curr_standard = vec![(StandardVersion::IEEE1800_2023, Span::default())];
    let mut begin_keywords_started = false;
    for chunk in stream.iter_mut() {
        match chunk {
            (Ok(Token::DirBeginKeywords), _) => {
                begin_keywords_started = true;
            }
            (Ok(Token::StringLiteral(specifier)), span) => {
                if begin_keywords_started {
                    match *specifier {
                        "1800-2023" => {
                            curr_standard.push((StandardVersion::IEEE1800_2023, span.clone()))
                        }
                        "1800-2017" => {
                            curr_standard.push((StandardVersion::IEEE1800_2017, span.clone()))
                        }
                        "1800-2012" => {
                            curr_standard.push((StandardVersion::IEEE1800_2012, span.clone()))
                        }
                        "1800-2009" => {
                            curr_standard.push((StandardVersion::IEEE1800_2009, span.clone()))
                        }
                        "1800-2005" => {
                            curr_standard.push((StandardVersion::IEEE1800_2005, span.clone()))
                        }
                        "1364-2005" => {
                            curr_standard.push((StandardVersion::IEEE1364_2005, span.clone()))
                        }
                        "1364-2001" => {
                            curr_standard.push((StandardVersion::IEEE1364_2001, span.clone()))
                        }
                        "1364-2001-noconfig" => curr_standard
                            .push((StandardVersion::IEEE1364_2001Noconfig, span.clone())),
                        "1364-1995" => {
                            curr_standard.push((StandardVersion::IEEE1364_1995, span.clone()))
                        }
                        _ => {
                            *chunk = (
                                Err(format!("Invalid version specifier '{}'", specifier)),
                                span.clone(),
                            )
                        }
                    }
                    begin_keywords_started = false;
                }
            }
            (Ok(Token::DirEndKeywords), span) => {
                if curr_standard.len() > 1 {
                    curr_standard.pop();
                } else {
                    *chunk = (
                        Err("end_keywords directive with no begin_keywords".to_owned()),
                        span.clone(),
                    );
                }
            }
            (result, span) => {
                if begin_keywords_started {
                    *chunk = (Err("Expected version specifier".to_owned()), span.clone());
                    begin_keywords_started = false;
                } else {
                    if let Ok(token) = result {
                        if token.keyword_replace(curr_standard.last().unwrap().clone().0) {
                            *chunk = (Ok(Token::SimpleIdentifier("test")), span.clone());
                        }
                    }
                }
            }
        }
    }
}

// -----------------------------------------------------------------------
// time_unit_postprocess
// -----------------------------------------------------------------------
// Convert identifiers into time units if they follow a number

pub fn time_unit_postprocess<'a>(stream: &mut Vec<(Result<Token<'a>, String>, Span)>) {
    let mut previous_number = false;
    for chunk in stream.iter_mut() {
        match chunk {
            (Ok(Token::UnsignedNumber(_)), _) => previous_number = true,
            (Ok(Token::FixedPointNumber(_)), _) => previous_number = true,
            (Ok(Token::SimpleIdentifier(text)), span) => {
                if previous_number {
                    match *text {
                        "s" | "ms" | "us" | "ns" | "ps" | "fs" => {
                            *chunk = (Ok(Token::TimeUnit(text)), span.clone())
                        }
                        _ => (),
                    }
                    previous_number = false;
                }
            }
            _ => previous_number = false,
        }
    }
}

// -----------------------------------------------------------------------
// triple_quote_string_postprocess
// -----------------------------------------------------------------------
// Form triple-quoted strings based on the start and end delimiters

pub fn triple_quote_string_postprocess<'a>(
    stream: Vec<(Result<Token<'a>, String>, Span)>,
    src: &'a str,
) -> Vec<(Result<Token<'a>, String>, Span)> {
    let mut triple_quote_string_started = false;
    let mut triple_quote_string_start_span = Span::default();
    let mut new_vec: Vec<(Result<Token<'a>, String>, Span)> = Vec::new();
    for chunk in stream {
        match chunk {
            (Ok(Token::QuoteQuoteQuote), span) => {
                if triple_quote_string_started {
                    let string_span = Span {
                        start: triple_quote_string_start_span.start,
                        end: span.end,
                    };
                    let text_span = Span {
                        start: triple_quote_string_start_span.end,
                        end: span.start,
                    };
                    let string_text = &src[text_span.start..text_span.end];
                    new_vec.push((
                        Ok(Token::TripleQuoteStringLiteral(string_text)),
                        string_span,
                    ));
                    triple_quote_string_started = false;
                } else {
                    triple_quote_string_started = true;
                    triple_quote_string_start_span = span;
                }
            }
            _ => {
                if !triple_quote_string_started {
                    new_vec.push(chunk)
                }
            }
        }
    }
    if triple_quote_string_started {
        new_vec.push((
            Err("Triple-quote string with no ending".to_owned()),
            triple_quote_string_start_span,
        ))
    }
    new_vec
}

// -----------------------------------------------------------------------
// postprocess
// -----------------------------------------------------------------------
// Apply all post-processing passes

pub fn postprocess<'a>(
    stream: Vec<(Result<Token<'a>, String>, Span)>,
    src: &'a str,
) -> Vec<(Result<Token<'a>, String>, Span)> {
    let stream = block_comment_merge_postprocess(stream, src);
    let mut stream = triple_quote_string_postprocess(stream, src);
    keyword_postprocess(&mut stream, src);
    time_unit_postprocess(&mut stream);
    stream
}
