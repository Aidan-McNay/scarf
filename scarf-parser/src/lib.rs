// =======================================================================
// lib.rs
// =======================================================================
//! A SystemVerilog preprocessor and parser
//!
//! `scarf-parser` provides capabilities for transformting a SystemVerilog
//! source file into a CST compliant with IEEE 1800-2023, with an
//! emphasis on informative error messages. It can be used as the
//! front-end for other tools looking to interpret SystemVerilog designs.
//!
//! ## Features
//!
//!  - `lossless`: Equivalent to the `lossless` feature for [`scarf_syntax`].
//!    Produces a CST with room for non-trivia nodes, but does not actually
//!    parse any from provided sources
//!  - `parse_lossless`: Extends `lossless` to parse non-trivia tokens.
//!    Due to their arbitrary position in source files, this adds a
//!    measurable performance decrease, and should only be used if
//!    newlines/comments are needed.

mod error;
pub mod lexer;
pub mod parser;
pub mod preprocessor;
use ariadne::ReportBuilder;
use ariadne::{Color, Label, ReportKind};
pub use ariadne::{Report, Source, sources};
pub use error::*;
use lexer::*;
pub use lexer::{LexedSource, Token, lex};
pub use parser::parse;
use parser::*;
pub use preprocessor::*;
use winnow::Parser;
use winnow::stream::TokenSlice;
#[cfg(test)]
pub mod test;
pub use scarf_syntax::Span;
#[cfg(test)]
pub use test::*;

/// A string and its associated [`Span`] in the source files
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpannedString<'a>(pub &'a str, pub Span<'a>);

/// A token and its location in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken<'s>(pub Token<'s>, pub Span<'s>);
impl<'s> PartialEq<Token<'s>> for SpannedToken<'s> {
    fn eq(&self, other: &Token) -> bool {
        self.0 == *other
    }
}
impl<'s> From<(Token<'s>, Span<'s>)> for SpannedToken<'s> {
    fn from(item: (Token<'s>, Span<'s>)) -> Self {
        (item.0, item.1).into()
    }
}

fn get_expansion_string(expansion_depth: usize, is_first: bool) -> String {
    if expansion_depth == 0 {
        "Original token".to_string()
    } else if (expansion_depth == 1) && is_first {
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

pub(crate) fn kind_color<'s>(kind: &ariadne::ReportKind<'s>) -> Color {
    match kind {
        ReportKind::Error => Color::Red,
        ReportKind::Warning => Color::Yellow,
        ReportKind::Advice => Color::Fixed(147),
        ReportKind::Custom(_, color) => color.clone(),
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
    let mut expansion_depth: usize = curr_span.expansion_depth();
    let mut expanded = false;
    loop {
        if let Some(expanded_span) = curr_span.expanded_from {
            report = report.with_label(
                Label::new((
                    curr_span.file.to_string(),
                    curr_span.bytes.clone(),
                ))
                .with_message(get_expansion_string(
                    expansion_depth,
                    expanded == false,
                ))
                .with_color(Color::BrightGreen),
            );
            curr_span = expanded_span;
            expansion_depth -= 1;
            expanded = true;
        } else {
            break;
        }
    }
    if expanded {
        // Also label expansion in original spot
        report = report.with_label(
            Label::new((curr_span.file.to_string(), curr_span.bytes.clone()))
                .with_message(get_expansion_string(expansion_depth, false))
                .with_color(Color::BrightGreen),
        );
    }
    curr_span = &span;
    report = report.with_label(
        Label::new((curr_span.file.to_string(), curr_span.bytes.clone()))
            .with_message(msg)
            .with_color(color)
            .with_priority(1),
    );
    let mut note = "".to_string();
    let mut note_pad = "".to_string();
    let total_inclusion_depth = curr_span.inclusion_depth();
    let mut curr_inclusion_depth = 0;
    // Only display a max of 7 includes
    loop {
        if (curr_inclusion_depth == 3) & (total_inclusion_depth > 7) {
            for _ in 7..=total_inclusion_depth {
                curr_span = curr_span.included_from.unwrap();
            }
            note = format!("{}\n{}╰- ...", note, note_pad);
            note_pad += "  ";
        }
        if let Some(included_span) = curr_span.included_from {
            curr_inclusion_depth += 1;
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
    debug_assert!(curr_span.included_from.is_none());
    if !note.is_empty() {
        report = report.with_note(note);
    }
    report
}
