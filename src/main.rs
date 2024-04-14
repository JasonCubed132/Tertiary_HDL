use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tertiary.pest"]
pub struct TertiaryParser;

fn main() {
    let unparsed_file = fs::read_to_string("test/simple.t").expect("Couldn't find file");

    let file = TertiaryParser::parse(Rule::file, &unparsed_file)
        .expect("bad parse")
        .next().unwrap();
    
    println!("{:?}", file);
}
