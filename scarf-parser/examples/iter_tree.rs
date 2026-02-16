// =======================================================================
// iter_tree.rs
// =======================================================================
// Iterate across the AST of SystemVerilog source code

use clap::Parser;
use scarf_parser::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    include: Vec<PathBuf>,

    #[arg(short, long)]
    defines: Vec<String>,

    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;
    let src = std::fs::read_to_string(&path).unwrap();
    let string_cache = PreprocessorCache::default();
    let mut configs = PreprocessConfigs::new(&string_cache);
    let (_, src) = configs
        .retain_file(path.clone().into_os_string().into_string().unwrap(), src);
    let lexed_src = lex(src, path.to_str().unwrap(), None).collect::<Vec<_>>();
    match dump_lex(&lexed_src.clone().into_iter(), "./scarf_debug/lex.txt") {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
    let lex_errors = report_lex_errors(&lexed_src.clone().into_iter());
    if !lex_errors.is_empty() {
        for report in lex_errors {
            report
                .print((path.to_str().unwrap(), Source::from(src)))
                .unwrap()
        }
        return;
    }
    let token_stream = lex_to_parse_stream(lexed_src.into_iter());
    let mut preprocessed_stream: Vec<SpannedToken<'_>> = vec![];
    let preprocess_result = preprocess(
        &mut TokenIterator::new(token_stream.into_iter()),
        &mut preprocessed_stream,
        &mut configs,
    );
    let mut error_sources = sources(configs.included_files());
    match preprocess_result {
        Err(err) => {
            let report: Report<'_, (String, std::ops::Range<usize>)> =
                err.into();
            report.print(&mut error_sources).unwrap();
        }
        _ => (),
    }
    let parsed_src = parse(&preprocessed_stream);
    if let Err(err) = parsed_src {
        let report: Report<'_, (String, std::ops::Range<usize>)> = err.into();
        report.print(&mut error_sources).unwrap();
        return;
    }
    let source_text = parsed_src.unwrap();
    for node in source_text.iter() {
        println!("{}", node.name());
    }
}
