use cirkit_parser::{Source, parse, report_errors};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let result = parse(&src);
    println!("{:?}", parse(&src));
    for report in report_errors(result, &path) {
        report
            .print((path.as_str(), Source::from(src.as_str())))
            .unwrap()
    }
}
