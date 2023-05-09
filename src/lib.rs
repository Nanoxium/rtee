use entities::Parameter;

mod fuzzy_logic;

pub trait TrustEvaluator {
    fn evaluate(&self, parameters: &TrustParameters) -> f64;
}

pub trait ReputationEvaluator {
    fn evaluate(&self, parameters: &ReputationParameters) -> f64;
}

pub struct TrustParameters {
    perplexity: f64,

}

pub struct ReputationParameters {
    // Define reputation-related parameters here
}

pub struct TrustReputationEngine<T: TrustEvaluator, R: ReputationEvaluator> {
    trust_evaluator: T,
    reputation_evaluator: R,
}

impl<T: TrustEvaluator, R: ReputationEvaluator> TrustReputationEngine<T, R> {
    pub fn new(trust_evaluator: T, reputation_evaluator: R) -> Self {
        TrustReputationEngine {
            trust_evaluator,
            reputation_evaluator,
        }
    }

    pub fn evaluate_trust(&self, parameters: &TrustParameters) -> f64 {
        self.trust_evaluator.evaluate(parameters)
    }

    pub fn evaluate_reputation(&self, parameters: &ReputationParameters) -> f64 {
        self.reputation_evaluator.evaluate(parameters)
    }
}

/// This crate is a library for trust evaluation. It gives one, many options to combine to be used
/// in a trust evaluation engine.
///
/// Currently the only functions or method to evaluate trust is by using fuzzy logic
use fuzzy_rs::FuzzyController;

pub struct FuzzyTrustEvaluator {
    fuzzy_controller: FuzzyController,
}

// #[pymodule]
// fn rtee(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function()
//     Ok(())
// }

