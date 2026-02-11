// =======================================================================
// iter_tree.rs
// =======================================================================
// Iterate across the AST of SystemVerilog source code

use scarf_parser::*;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        panic!("Usage: iter_tree source.sv")
    }
    let path = args.pop().unwrap();
    let src = std::fs::read_to_string(&path).unwrap();
    let string_cache = PreprocessorCache::default();
    let mut configs = PreprocessConfigs::new(&string_cache);
    let (_, src) = configs.retain_file(path.clone(), src);
    let lexed_src = lex(src, path.as_str(), None).collect::<Vec<_>>();
    match dump_lex(&lexed_src.clone().into_iter(), "./scarf_debug/lex.txt") {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
    let lex_errors = report_lex_errors(&lexed_src.clone().into_iter());
    if !lex_errors.is_empty() {
        for report in lex_errors {
            report.print((path.as_str(), Source::from(src))).unwrap()
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
        report.print(&mut error_sources).unwrap()
    }
}
