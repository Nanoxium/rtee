//! This crate is a library for trust evaluation. It gives one, many options to combine to be used
//! in a trust evaluation engine.
//!
//! Currently the only functions or method to evaluate trust is by using fuzzy logic
//! 
//!
//!
pub mod prelude;
pub mod fuzzy_logic;
pub mod subjective_logic;

/// Trait on how a trust evaluator should interact
pub trait TrustEvaluator {
    type TrustContext;

    fn evaluate(&self, parameters: &Self::TrustContext) -> f64;
}

/// Trait on how a trust evaluator should interact
pub trait ReputationEvaluator {
    type ReputationContext;
    fn evaluate(&self, parameters: &Self::ReputationContext) -> f64;
}

/// This is the main trust reputation engine. It is the one that will be used to evaluate trust
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

    /// Evaluate the level of trust from given trust parameters
    pub fn evaluate_trust(&self, parameters: &T::TrustContext) -> f64 {
        self.trust_evaluator.evaluate(parameters)
    }

    /// Evaluate the reputation given a trust context
    pub fn evaluate_reputation(&self, parameters: &R::ReputationContext) -> f64 {
        self.reputation_evaluator.evaluate(parameters)
    }
}

