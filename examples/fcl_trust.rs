use rtee::prelude::*;

fn main() {
    let rules = "IF reputation IS low THEN trust IS medium";

    let values = fcl_parser(rules).unwrap();


    // Define FuzzySet
    println!("Trust value: {:?}", values);
}
