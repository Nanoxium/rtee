pub mod controller;
pub mod functions;
pub mod inference;

pub use controller::*;

/// Structure to define a rule for membership of a variable.
#[derive(Debug, Clone)]
pub struct Rule<F: Fn(f64) -> f64> {
    apriori: Vec<FuzzySet<F>>,
    aposteriori: FuzzySet<F>,
}

/// Struct to represent fuzzy set definitions
#[derive(Debug, Clone)]
pub struct FuzzySet<F: Fn(f64) -> f64> {
    pub name: String,
    pub membership_fn: Box<F>,
}

impl<F: Fn(f64) -> f64> FuzzySet<F> {
    pub fn new(name: &str, membership_fn: Box<F>) -> Self {
        Self {
            name: name.to_string(),
            membership_fn,
        }
    }

    /// Evaluate the membership of a value
    pub fn membership(&self, value: f64) -> f64 {
        (self.membership_fn)(value)
    }
}

/// Structure to represent a value that can be inferenced
#[derive(Debug, Clone)]
pub struct FuzzyVariable<F: Fn(f64) -> f64> {
    name: String,
    fuzzy_sets: Vec<FuzzySet<F>>,
    pub defuzz_fn: Box<F>,
}

impl<F: Fn(f64) -> f64> FuzzyVariable<F>
{
    pub fn new(name: &str, fuzzy_sets: Vec<FuzzySet<F>>, defuzz_fn: Box<F>) -> Self {
        Self {
            name: name.to_string(),
            fuzzy_sets,
            defuzz_fn,
        }
    }
}
