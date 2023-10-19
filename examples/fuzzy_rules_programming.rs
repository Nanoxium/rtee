use rtee::prelude::*;

fn main() {
    // let rules = "IF reputation IS low AND trust IS low THEN trust IS medium";
    let fcl_program = "IF reputation IS low THEN trust IS high";

    let values = fcl_parser(fcl_program);
    if let Ok(rules) = values {
        println!("{rules:?}");
    } else {
        eprintln!("Error while parsing rules in FCL: {values:?}")
    }

//     let fll_program = r"#from examples/mamdani/ObstacleAvoidance.fll  
// Engine: ObstacleAvoidance
// description: An engine for obstacle avoidance
// InputVariable: obstacle
//   description: direction of the obstacle
//   enabled: true
//   range: 0.000 1.000
//   lock-range: false
//   term: left Ramp 1.000 0.000
//   term: right Ramp 0.000 1.000
// OutputVariable: mSteer
//   description: steering direction of the vehicle
//   enabled: true
//   range: 0.000 1.000
//   lock-range: false
//   aggregation: Maximum
//   defuzzifier: Centroid 100
//   default: nan
//   lock-previous: false
//   term: left Ramp 1.000 0.000
//   term: right Ramp 0.000 1.000
// RuleBlock: mamdani
//   description: Mamdani inference for steering
//   enabled: true
//   conjunction: none
//   disjunction: none
//   implication: AlgebraicProduct
//   activation: General
//   rule: if obstacle is left then mSteer is right
//   rule: if obstacle is right then mSteer is left";
//
//     let values = fll_parser(fll_program);
//     if let Ok(rules) = values {
//         println!("{rules:?}");
//     } else {
//         eprintln!("Error while parsing rules in FCL: {values:?}")
//     }

    // Define FuzzySet
    // println!("Trust value: {:?}", values);
}
