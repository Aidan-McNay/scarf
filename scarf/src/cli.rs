// =======================================================================
// cli.rs
// =======================================================================
// Helper functions for CLI parsing

use scarf_parser::SpannedString;
use scarf_parser::preprocessor::state::{Define, DefineBody};
use scarf_parser::{LexedSource, lex};
use scarf_syntax::Span;

fn _parse_cli_define<'a>(cli_define: &'a str) -> Define<'a> {
    if let Some((first, second)) = cli_define.split_once('=') {
        let name = SpannedString(first, Span::default());
        let body =
            DefineBody::Text(lex(second, "").tokens().collect::<Vec<_>>());
        Define { name, body }
    } else {
        todo!()
    }
}
