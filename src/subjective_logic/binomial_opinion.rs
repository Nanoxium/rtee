//! BinomialOpinion implementation
//!
use assert_float_eq::*;

use super::SLOpertors;

#[derive(Debug, Clone, Copy)]
struct BinomialOpinion {
    belief: f64,
    disbelief: f64,
    uncertainty: f64,
    base_rate: f64,
}


impl SLOpertors for BinomialOpinion {
    // Initialize a new opinion ensuring it's valid.
    fn new(belief: f64, disbelief: f64, uncertainty: f64, base_rate: f64) -> Self {
        dbg!(belief + disbelief + uncertainty);
        assert_float_relative_eq!(belief + disbelief + uncertainty, 1.0, f64::EPSILON);
        assert!(belief >= 0.0 && disbelief >= 0.0 && uncertainty >= 0.0 && base_rate >= 0.0);
        assert!(belief <= 1.0 && disbelief <= 1.0 && uncertainty <= 1.0 && base_rate <= 1.0);
        Self {
            belief,
            disbelief,
            uncertainty,
            base_rate,
        }
    }

    /// Calculate the projected probability P(x)
    fn prob(self) -> f64 {
        self.belief + self.base_rate * self.uncertainty
    }

    /// Addition operator
    /// ```
    ///
    /// ```
    fn addition(self, other: Self) -> Self {
        let a = self.base_rate + other.base_rate;
        let b = self.belief + other.belief;
        let d = ((self.base_rate * (self.disbelief - other.belief))
            + (other.base_rate * (other.disbelief - self.belief)))
            / a;
        let u = (self.base_rate * self.uncertainty + other.base_rate * other.uncertainty) / a;

        Self::new(b, d, u, a)
    }

    // Difference (Subtraction)
    fn substract(self, other: Self) -> Self {
        let union = self.addition(other);
        
        todo!("Understand the operations and implement it")
    }

    // Conjunction / AND (Multiplication)
    fn multiply(self, other: Self) -> Self {
        todo!("Understand the conjunction operator and implement it")
    }

    // Unconjunction / UN-AND (Division)
    fn divide(self, other: Self) -> Self {
        todo!("Understand the unconjunction operator and implement it")
    }

    // Disjunction / OR (Comultiplication)
    fn comult(self, other: Self) -> Self {
        todo!("Understand the Comultiplication operator and implement it")
    }

    // Undisjunction / UN-OR (Codivision)
    fn codiv(self, other: Self) -> Self {
        todo!("Understand the Codivision operator and implement it")
    }

    // NOT (Complement)
    fn complement(self) -> Self {
        todo!("Understand the complement operator and implement it")
    }

    // Modus ponens (Deduction)
    fn deduct(self, other: Self) -> Self {
        todo!("Understand the deduction operator and implement it")
    }

    // Contraposition (Subjective Bayes' theorem)
    fn subjective_bayes(self, other: Self) -> Self {
        todo!("Understand the disconjunction operator and implement it")
    }

    // Modus tollens (Abduction)
    fn abduct(self, other: Self) -> Self {
        todo!("Understand the cumulative operator and implement it")
    }

    fn cumul_fuse(self, other: Self) -> Self {
        todo!("Understand the constraint fuse operator and implement it")
    }

    fn constrained_fuse(self, other: Self) -> Self {
        todo!("Understand the constraint fuse operator and implement it")
    }
}

impl PartialEq for BinomialOpinion {
    fn eq(&self, other: &Self) -> bool {
        Self::_isclose(self.belief, other.belief, f64::EPSILON)
        && Self::_isclose(self.disbelief, other.disbelief, f64::EPSILON)
        && Self::_isclose(self.uncertainty, other.uncertainty, f64::EPSILON)
        && Self::_isclose(self.base_rate, other.base_rate, f64::EPSILON)
    }
}

impl std::ops::Add for BinomialOpinion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.addition(other)
    }
}

impl std::ops::Sub for BinomialOpinion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.substract(other)
    }
}

impl std::ops::Mul for BinomialOpinion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.multiply(other)
    }
}

impl std::ops::Div for BinomialOpinion {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.divide(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // Test taken from book A. Josang Subjective Logic. chap 6 fig 6.2
        let op1 = BinomialOpinion::new(0.2, 0.4, 0.4, 0.25);
        let op2 = BinomialOpinion::new(0.1, 0.5, 0.4, 0.5);

        let expected_result = BinomialOpinion::new(0.3, 0.3, 0.4, 0.75);
        assert_eq!(op1.prob() + op2.prob(), expected_result.prob());
        assert_eq!(op1.addition(op2), expected_result);
        assert_eq!(op1 + op2, expected_result);
    }

    // #[test]
    // fn test_substrac() {
    //     // Test taken from book A. Josang Subjective Logic. chap 6 fig 6.3
    //     let op1 = Opinion::new(0.7, 0.1, 0.2, 0.75);
    //     let op2 = Opinion::new(0.5, 0.3, 0.2, 0.25);
    //
    //     let expected_result = Opinion::new(0.2, 0.6, 0.2, 0.5);
    //     assert_eq!(op1.sub(op2), expected_result);
    //     assert_eq!(op1 - op2, expected_result);
    // }
}
