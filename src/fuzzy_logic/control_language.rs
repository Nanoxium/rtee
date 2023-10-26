use pest::Parser;
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "fuzzy_logic/grammar/fcl.pest"]
pub struct FCLParser;

pub fn fcl_parser(input: &str) -> Result<(), pest::error::Error<Rule>> {
    // let ident = FCLParser::parse(Rule::rule, input)?;
    // println!("Ident: {:?}", ident);

    let pairs = FCLParser::parse(Rule::fcl, input)?;
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

// #[derive(Parser)]
// #[grammar = "fuzzy_logic/grammar/fll.pest"]
// pub struct FLLParser;

// pub fn fll_parser(input: &str) -> Result<(), pest::error::Error<Rule>> {
//     todo!("Fill this function")
// }

