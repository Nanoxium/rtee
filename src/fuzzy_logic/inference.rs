pub mod mamdani;
pub use mamdani::*;

pub trait FuzzySystem {
    fn fuzzify(&self, input_values: &[f64]) -> Vec<Vec<f64>>;
    fn infer(&self, input_values: &[f64]) -> f64;
}
