use rtee::prelude::*;
use std::rc::Rc;

fn main() {
    // Define FuzzySet
    let low_trust = FuzzySet::new("low", Rc::new(triangular(0.0, 0.0, 5.0)));
    let medium_trust = FuzzySet::new("medium", Rc::new(triangular(0.0, 5.0, 10.0)));
    let high_trust = FuzzySet::new("high", Rc::new(triangular(5.0, 10.0, 10.0)));

    // Define fuzzy variable
    let reputation = FuzzyVariable::new(
        "reputation",
        vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()],
        Rc::new(|_| 0.0),
    );
    let transaction_history = FuzzyVariable::new(
        "transaction_history",
        vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()],
        Rc::new(|_| 0.0),
    );

    let trust_level = FuzzyVariable::new(
        "trust_level",
        vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()],
        Rc::new(|total_weight| total_weight / 3.0),
    );

    // Define rules to infer member ship
    let rule1 = Rule::new(
        vec![low_trust.clone(), low_trust.clone()],
        low_trust.clone(),
    );
    let rule2 = Rule::new(
        vec![low_trust.clone(), medium_trust.clone()],
        low_trust.clone(),
    );
    let rule3 = Rule::new(
        vec![medium_trust.clone(), low_trust.clone()],
        low_trust.clone(),
    );
    let rule4 = Rule::new(
        vec![medium_trust.clone(), medium_trust.clone()],
        medium_trust.clone(),
    );
    let rule5 = Rule::new(
        vec![high_trust.clone(), low_trust.clone()],
        medium_trust.clone(),
    );
    let rule6 = Rule::new(
        vec![low_trust.clone(), high_trust.clone()],
        medium_trust.clone(),
    );
    let rule7 = Rule::new(
        vec![high_trust.clone(), medium_trust.clone()],
        high_trust.clone(),
    );
    let rule8 = Rule::new(
        vec![medium_trust.clone(), high_trust.clone()],
        high_trust.clone(),
    );
    let rule9 = Rule::new(
        vec![high_trust.clone(), high_trust.clone()],
        high_trust.clone(),
    );

    // Using the mamdani fuzzy inference system
    let mamdani = Mamdani::new(
        vec![reputation, transaction_history],
        trust_level,
        vec![
            rule1, rule2, rule3, rule4, rule5, rule6, rule7, rule8, rule9,
        ],
    );

    // Create the controller for our trust engine
    let trust_controller = FuzzyController::new(mamdani);

    // Set the initial values
    let reputation_value = 7.5;
    let transaction_history_value = 4.0;

    // Evaluate the values and get the final result
    let trust_value = trust_controller.evaluate(&[reputation_value, transaction_history_value]);
    println!("Trust value: {:?}", trust_value);
}
