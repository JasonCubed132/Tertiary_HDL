use std::fs;
use clap::Parser as cParser;
use pest::Parser as pParser;
use pest::iterators::Pair;
use pest_derive::Parser as pParser;

#[derive(cParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String
}

#[derive(pParser)]
#[grammar = "tertiary.pest"]
pub struct TertiaryParser;

fn print_parse<'a>(parsed: Pair<'a, Rule>, level: i32) {
    for _ in 0..level {
        print!("-");
    }
    print!(" ");

    let sub_rule_count = parsed.clone().into_inner().len();

    if sub_rule_count == 0 {
        println!("{:?} {:?}", parsed.as_rule(), parsed.as_span().as_str());
    } else {
        println!("{:?}", parsed.as_rule());

        for item in parsed.into_inner() {
            print_parse(item, level + 1);
        }
    }
}

fn main() {
    let args = Args::parse();


    let unparsed_file = fs::read_to_string(args.input_file)
        .expect("Couldn't find file");

    let file = TertiaryParser::parse(Rule::file, unparsed_file.as_str())
        .expect("bad parse").next().unwrap();

    print_parse(file, 0);
}
