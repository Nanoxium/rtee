pub mod control_language;
pub mod controller;
pub mod functions;
pub mod inference;

use std::rc::Rc;

pub use control_language::*;
pub use controller::*;

/// Structure to define a rule for membership of a variable.
pub struct Rule {
    antecedents: Vec<FuzzySet>,
    consequent: FuzzySet,
}

impl Rule {
    pub fn new(antecedents: Vec<FuzzySet>, consequent: FuzzySet) -> Self {
        Self {
            antecedents,
            consequent,
        }
    }
}

/// Struct to represent fuzzy set definitions
#[derive(Clone)]
pub struct FuzzySet {
    pub name: String,
    pub membership_fn: Rc<dyn Fn(f64) -> f64>,
}

impl FuzzySet {
    pub fn new(name: &str, membership_fn: Rc<dyn Fn(f64) -> f64>) -> Self {
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
pub struct FuzzyVariable {
    name: String,
    fuzzy_sets: Vec<FuzzySet>,
    pub defuzz_fn: Rc<dyn Fn(f64) -> f64>,
}

impl FuzzyVariable {
    pub fn new(name: &str, fuzzy_sets: Vec<FuzzySet>, defuzz_fn: Rc<dyn Fn(f64) -> f64>) -> Self {
        Self {
            name: name.to_string(),
            fuzzy_sets,
            defuzz_fn,
        }
    }
}
