pub mod prelude;
pub mod fuzzy_logic;

use fuzzy_logic::FuzzyController;
use prelude::inference::FuzzySystem;

pub trait TrustEvaluator {
    type TrustParameters;

    fn evaluate(&self, parameters: &Self::TrustParameters) -> f64;
}

pub trait ReputationEvaluator {
    type ReputationParameters;
    fn evaluate(&self, parameters: &Self::ReputationParameters) -> f64;
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

    pub fn evaluate_trust(&self, parameters: &T::TrustParameters) -> f64 {
        self.trust_evaluator.evaluate(parameters)
    }

    pub fn evaluate_reputation(&self, parameters: &R::ReputationParameters) -> f64 {
        self.reputation_evaluator.evaluate(parameters)
    }
}

/// This crate is a library for trust evaluation. It gives one, many options to combine to be used
/// in a trust evaluation engine.
///
/// Currently the only functions or method to evaluate trust is by using fuzzy logic

pub struct FuzzyTrustEvaluator<T> where T: FuzzySystem {
    fuzzy_controller: FuzzyController<T>,
}

