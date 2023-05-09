pub trait TrustEvaluator {
    fn evaluate(&self, parameters: &TrustParameters) -> f64;
}

pub struct TrustEngine<T: TrustEvaluator, > {
    trust_evaluator: T,
}

impl<T: TrustEvaluator> TrustReputationEngine<T> {
    pub fn new(trust_evaluator: T) -> Self {
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
