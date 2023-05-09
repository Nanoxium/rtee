use crate::fuzzy_logic::inference::*;

pub struct FuzzyController<T>
where
    T: FuzzySystem,
{
    pub infer: T,
}

impl<T> FuzzyController<T>
where
    T: FuzzySystem,
{
    pub fn new(infer: T) -> Self {
        Self { infer }
    }

    pub fn evaluate(&self, input_values: &[f64]) -> f64 {
        self.infer.infer(input_values)
    }
}
