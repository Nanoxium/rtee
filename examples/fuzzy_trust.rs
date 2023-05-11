use rtee::prelude::*;

fn main() {
    let low_trust = FuzzySet::new("low", Box::new(triangular(0.0, 0.0, 5.0)));
    let medium_trust = FuzzySet::new("medium", Box::new(triangular(0.0, 5.0, 10.0)));
    let high_trust = FuzzySet::new("high", Box::new(triangular(5.0, 10.0, 10.0)));

    let reputation = FuzzyVariable::new("reputation", vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()], Box::new(|_| 0.0));
    let transaction_history = FuzzyVariable::new("transaction_history", vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()], Box::new(|_| 0.0));
    let trust_level = FuzzyVariable::new("trust_level", vec![low_trust.clone(), medium_trust.clone(), high_trust.clone()], Box::new(|total_weight| total_weight / 3.0));

    let rule1 = Rule::new(vec![low_trust.clone(), low_trust.clone()], low_trust.clone());
    let rule2 = Rule::new(vec![low_trust.clone(), medium_trust.clone()], low_trust.clone());
    let rule3 = Rule::new(vec![medium_trust.clone(), low_trust.clone()], low_trust.clone());
    let rule4 = Rule::new(vec![medium_trust.clone(), medium_trust.clone()], medium_trust.clone());
    let rule5 = Rule::new(vec![high_trust.clone(), low_trust.clone()], medium_trust.clone());
    let rule6 = Rule::new(vec![low_trust.clone(), high_trust.clone()], medium_trust.clone());
    let rule7 = Rule::new(vec![high_trust.clone(), medium_trust.clone()], high_trust.clone());
    let rule8 = Rule::new(vec![medium_trust.clone(), high_trust.clone()], high_trust.clone());
    let rule9 = Rule::new(vec![high_trust.clone(), high_trust.clone()], high_trust.clone());
    
    let mamdani = Mamdani::new(vec![reputation, transaction_history], trust_level, vec![rule1, rule2, rule3, rule4, rule5, rule6, rule7, rule8, rule9]);
    let trust_controller = FuzzyController::new(mamdani);

    let reputation_value = 7.5;
    let transaction_history_value = 4.0;

    let trust_value = trust_controller.evaluate(&[reputation_value, transaction_history_value]);
    println!("Trust value: {:?}", trust_value);
}
