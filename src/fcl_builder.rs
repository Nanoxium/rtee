#[macro_use]
use pest_derive::Parser;

use crate::prelude::Rule;

#[derive(Parser)]
#[grammar = "fuzzy_logic/fcl.pest"]
struct FCLParser;

fn fcl_parser(input: &str) -> Result<(), pest::error::Error<Rule>> {
    let pairs = FCLParser::parse(Rule::rule, input)?;
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("Pairs:");
        for inner_pair in pair.into_inner() {
            println!("Rule:    {:?}", inner_pair.as_rule());
            println!("Span:    {:?}", inner_pair.as_span());
            println!("Text:    {}", inner_pair.as_str());
        }
    }
    Ok(())
}
