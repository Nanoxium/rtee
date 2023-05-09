use crate::fuzzy_logic::*;
use super::FuzzySystem;

#[derive(Debug, Clone)]
pub struct Mamdani<F: Fn(f64) -> f64> {
    inputs: Vec<FuzzyVariable<F>>,
    output: FuzzyVariable<F>,
    pub rules: Vec<Rule<F>>,
}

impl<F: Fn(f64) -> f64> Mamdani<F> {
    pub fn new(inputs: Vec<FuzzyVariable<F>>, output: FuzzyVariable<F>, rules: Vec<Rule<F>>) -> Self {
        Self {
            inputs,
            output,
            rules,
        }
    }
}

impl<F: Fn(f64) -> f64> FuzzySystem for Mamdani<F> {
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
                    .apriori
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

                rule.aposteriori.membership(min_value)
            })
            .collect();

        // Aggregate the rule outputs
        let aggregated_output = self
            .output
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
        let defuzzified_output = (self.output.defuzz_fn)(
            aggregated_output
                .iter()
                .map(|(_, membership)| membership)
                .sum::<f64>(),
        );

        defuzzified_output
    }
}
