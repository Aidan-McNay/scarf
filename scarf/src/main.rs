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
        let string_cache = PreprocessorCache::new();
        let mut state = PreprocessorState::new(vec![], vec![]);
        let (_, src) = state.retain_file(path.clone(), src, &string_cache);
        let lexed_src = lex(src, path.as_str()).process();
        match lexed_src.dump("./scarf_debug/lex.txt") {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
        let lex_errors = lexed_src.report_errors().collect::<Vec<_>>();
        if !lex_errors.is_empty() {
            for report in lex_errors {
                report.print((path.as_str(), Source::from(src))).unwrap()
            }
            return;
        }
        let token_stream = lexed_src.tokens();
        let preprocess_result = preprocess(
            &mut TokenIterator::new(token_stream.into_iter()),
            &mut state,
            &string_cache,
        );
        let mut error_sources = sources(state.included_files());
        let preprocessed_stream = match preprocess_result {
            Err(err) => {
                let report: Report<'_, (String, std::ops::Range<usize>)> =
                    err.into();
                report.print(&mut error_sources).unwrap();
                return;
            }
            Ok(preprocessed_stream) => preprocessed_stream,
        };
        let parsed_src = parse(&preprocessed_stream);
        if let Err(err) = parsed_src {
            let report: Report<'_, (String, std::ops::Range<usize>)> =
                err.into();
            report.print(&mut error_sources).unwrap()
        }
        // println!("{:#?}", parsed_src);
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
