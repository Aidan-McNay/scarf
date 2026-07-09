// =======================================================================
// iter_tree.rs
// =======================================================================
// Iterate across the AST of SystemVerilog source code

use clap::Parser;
use scarf_parser::report::Sources;
use scarf_parser::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    include: Vec<PathBuf>,

    #[arg(short, long)]
    defines: Vec<String>,

    /// A directory to dump debug output to
    #[arg(long)]
    debug: Option<PathBuf>,

    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;
    let src = std::fs::read_to_string(&path).unwrap();
    let string_cache = preprocessor::PreprocessorCache::new();
    let mut state = preprocessor::PreprocessorState::new(vec![], vec![]);
    let (_, src) = state.retain_file(
        path.clone().into_os_string().into_string().unwrap(),
        src,
        &string_cache,
    );
    let lexed_src = lex(src, path.to_str().unwrap()).process();
    if let Some(debug_path) = args.debug {
        match lexed_src.dump(&debug_path) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }
    let lex_errors = lexed_src.report_errors().collect::<Vec<_>>();
    if !lex_errors.is_empty() {
        let mut sources = state.included_files().sources();
        for report in lex_errors {
            report.print(&mut sources).unwrap();
        }
        return;
    }
    let token_stream = lexed_src.tokens();
    let preprocess_result = preprocess(token_stream, &mut state, &string_cache);
    let mut sources = state.included_files().sources();
    if !state.errors.is_empty() {
        for err in state.errors {
            let report: report::Report<'_> = (&err).into();
            report.print(&mut sources).unwrap();
        }
    }
    let preprocessed_stream = match preprocess_result {
        Err(_) => {
            return;
        }
        Ok(preprocessed_stream) => preprocessed_stream,
    };
    let parsed_src = parse(&preprocessed_stream);
    if let Err(err) = parsed_src {
        let report: report::Report<'_> = err.report("P1");
        report.print(&mut sources).unwrap();
        return;
    }
    let source_text = parsed_src.unwrap();
    for node in source_text.iter() {
        println!("{}", node.name());
    }
}
