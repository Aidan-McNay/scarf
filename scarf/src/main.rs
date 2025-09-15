// =======================================================================
// main.rs
// =======================================================================
// The top-level code for scarf

use clap::{Args, Parser, Subcommand};
use scarf_parser::*;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format source files in-place
    Format(FormatArgs),
}

// -----------------------------------------------------------------------
// format
// -----------------------------------------------------------------------

#[derive(Args)]
struct FormatArgs {
    /// The file(s) to format
    paths: Vec<String>,
}

fn format(args: &FormatArgs) {
    for path in &args.paths {
        let src = std::fs::read_to_string(&path).unwrap();
        let lexed_src = lex(&src);
        let lex_errors = report_lex_errors(&lexed_src, path);
        if !lex_errors.is_empty() {
            for report in lex_errors {
                report
                    .print((path.as_str(), Source::from(src.as_str())))
                    .unwrap()
            }
            return;
        }
        let parser_stream = lex_to_parse_stream(lexed_src);
        let parsed_src = parse(&parser_stream[..]);
        // let parse_errors = report_parse_errors(parsed_src.clone(), path);
        // if !parse_errors.is_empty() {
        //     for report in parse_errors {
        //         report
        //             .print((path.as_str(), Source::from(src.as_str())))
        //             .unwrap()
        //     }
        //     return;
        // }
        println!("{:?}", parsed_src);
    }
}

// fn format(args: &FormatArgs) {
//     for path in &args.paths {
//         let src = std::fs::read_to_string(&path).unwrap();
//         let result = lex(&src);
//         for (token, span) in &result {
//             println!("{:?} ({:?})", token, span);
//         }
//         for report in report_lex_errors(result, &path) {
//             report
//                 .eprint((path.as_str(), Source::from(src.as_str())))
//                 .unwrap()
//         }
//     }
// }

// -----------------------------------------------------------------------
// main
// -----------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Format(args) => format(&args),
    }
}
