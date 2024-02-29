use crate::fuzzy_logic::*;
use super::FuzzySystem;

pub struct Mamdani {
    inputs: Vec<FuzzyVariable>,
    outputs: Vec<FuzzyVariable>,
    pub rules: Vec<Rule>,
}

impl Mamdani {
    pub fn new(inputs: Vec<FuzzyVariable>, outputs: Vec<FuzzyVariable>, rules: Vec<Rule>) -> Self {
        Self {
            inputs,
            outputs,
            rules,
        }
    }
}

impl FuzzySystem for Mamdani {
    // Fuzzify the inputs
    fn fuzzify(&self, input_values: &[f64]) -> Vec<Vec<f64>> {
        self.inputs
            .iter()
            .zip(input_values)
            .map(|(input_var, &input_value)| {
                input_var
                    .fuzzy_sets
                    .iter()
                    .map(|fuzzy_set| fuzzy_set.membership(input_value))
                    .collect()
            })
            .collect()
    }

    /// Perform the inference process
    fn infer(&self, input_values: &[f64]) -> f64 {
        // Fuzzify the input values 
        let fuzzy_inputs = self.fuzzify(input_values);

        // Apply the rules to get the rule outputs
        let rule_outputs: Vec<f64> = self
            .rules
            .iter()
            .map(|rule| {
                let min_value = rule
                    .antecedents
                    .iter()
                    .enumerate()
                    .map(|(i, apriori)| {
                        let set_index = self.inputs[i]
                            .fuzzy_sets
                            .iter()
                            .position(|fs| fs.name == apriori.name)
                            .unwrap();

                        fuzzy_inputs[i][set_index]
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

                rule.consequent.membership(min_value)
            })
            .collect();

        // Aggregate the rule outputs
        let aggregated_output = self
            .outputs
            .fuzzy_sets
            .iter()
            .map(|output_set| {
                let max_membership = rule_outputs
                    .iter()
                    .map(|&rule_output| output_set.membership(rule_output))
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

                (output_set.name.clone(), max_membership)
            })
            .collect::<Vec<(String, f64)>>();

        // Defuzzify the aggregated output
        let defuzzified_output = (self.outputs.defuzz_fn)(
            aggregated_output
                .iter()
                .map(|(_, membership)| membership)
                .sum::<f64>(),
        );

        defuzzified_output
    }
}
