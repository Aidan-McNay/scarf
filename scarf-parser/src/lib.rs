// =======================================================================
// lib.rs
// =======================================================================
// The top-level interface for parsing a SystemVerilog source file

pub mod lexer;
pub mod parser;
pub mod preprocessor;
use ariadne::ReportBuilder;
pub use ariadne::{Report, Source, sources};
use lexer::*;
pub use lexer::{Token, dump_lex, lex, report_lex_errors};
use parser::*;
pub use parser::{SpannedToken, VerboseError, parse, report_parse_errors};
pub use preprocessor::*;
use winnow::Parser;
use winnow::stream::TokenSlice;
#[cfg(test)]
pub mod test;
pub use scarf_syntax::Span;
#[cfg(test)]
pub use test::*;

pub fn lex_to_parse_stream<'s>(
    input: Vec<(Result<Token<'s>, String>, Span<'s>)>,
) -> Vec<SpannedToken<'s>> {
    let mapped_input = input.into_iter().map(|(tok, span)| match tok {
        Ok(tok) => SpannedToken(tok, span),
        Err(_) => SpannedToken(Token::Error, span),
    });
    mapped_input.collect::<Vec<SpannedToken<'s>>>()
}

fn get_expansion_string(expansion_depth: u32, is_last: bool) -> String {
    if expansion_depth == 0 {
        "Original token".to_string()
    } else if (expansion_depth == 1) && is_last {
        "Expanded here".to_string()
    } else {
        let suffix = match expansion_depth % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        format!("Expanded here {}{}", expansion_depth, suffix)
    }
}

pub(crate) fn attach_span_label<'s, M>(
    span: Span<'s>,
    color: ariadne::Color,
    msg: M,
    mut report: ReportBuilder<'s, (String, std::ops::Range<usize>)>,
) -> ReportBuilder<'s, (String, std::ops::Range<usize>)>
where
    M: ToString,
{
    let mut curr_span: &Span<'s> = &span;
    let mut expansion_depth: u32 = 0;
    loop {
        if let Some(expanded_span) = curr_span.expanded_from {
            report = report.with_label(
                Label::new((
                    curr_span.file.to_string(),
                    curr_span.bytes.clone(),
                ))
                .with_message(get_expansion_string(expansion_depth, false))
                .with_color(Color::BrightGreen),
            );
            curr_span = expanded_span;
            expansion_depth += 1;
        } else {
            break;
        }
    }
    report = report.with_label(
        Label::new((curr_span.file.to_string(), curr_span.bytes.clone()))
            .with_message(msg)
            .with_color(color)
            .with_priority(1),
    );
    if expansion_depth > 0 {
        // Also label expansion in original spot
        report = report.with_label(
            Label::new((curr_span.file.to_string(), curr_span.bytes.clone()))
                .with_message(get_expansion_string(expansion_depth, true))
                .with_color(Color::BrightGreen),
        );
    }
    let mut note = "".to_string();
    let mut note_pad = "".to_string();
    loop {
        if let Some(included_span) = curr_span.included_from {
            curr_span = included_span;
            if note.is_empty() {
                note = format!("Included from {}", curr_span.file);
            } else {
                note = format!(
                    "{}\n{}╰-Included from {}",
                    note, note_pad, curr_span.file
                );
                note_pad += "  ";
            }
        } else {
            break;
        }
    }
    if !note.is_empty() {
        report = report.with_note(note);
    }
    report
}
