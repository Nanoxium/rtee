//! BinomialOpinion implementation
//!
//! All the formulas are from
//! A. Jøsang - Subjective Logic A Formalism for Reasoning Under Uncertainty (2016)

use assert_float_eq::*;

use super::SLOpertors;
const EPSILON: f64 = 1e-10;

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
        dbg!(belief + disbelief + uncertainty, base_rate);
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
    /// Add another opnition using the addition operator
    fn addition(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: dy,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        let a = ax + ay;
        let b = bx + by;
        let d = ((ax * (dx - by)) + (ay * (dy - bx))) / a;
        let u = (ax * ux + ay * uy) / a;

        Self::new(b, d, u, a)
    }

    // Difference (Subtraction)
    fn substract(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: _,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        let b = bx - by;
        let a = ax - ay;
        let d = (ax * (dx + by) - ay * (1. + by - bx - uy)) / a;
        let u = (ax * ux - ay * uy) / a;

        assert!(u >= 0.0);
        assert!(d >= 0.0);

        Self::new(b, d, u, a)
    }

    // Conjunction / AND (Multiplication)
    fn multiply(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: dy,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        let a = ax * ay;
        let d = dx + dy - dx * dy;
        let b = bx * by + ((1. - ax) * ay * bx * uy + (1. - ay) * ax * ux * by) / (1. - a);
        let u = ux * uy + ((1. - ay) * bx * uy + (1. - ax) * ux * by) / (1. - a);

        Self::new(b, d, u, a)
    }

    // Binomial Comultiplication (OR)
    fn comult(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: dy,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        let a = ax + ay - ax * ay;
        let b = bx + by - bx * by;
        let d = dx * dy + ((ax * (1. - ay) * dx * uy + (1. - ax) * ay * ux * dy) / a);
        let u = ux * uy + ((ay * dx * uy + ax * ux * dy) / a);

        Self::new(b, d, u, a)
    }

    // Unconjunction / UN-AND (Division)
    fn divide(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: dy,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        assert!(ax < ay && dx >= dy);

        // Calculate factors that are reused multiple times
        let a = ax / ay;
        let ayx = ay - ax;
        let ay_ayx = ay / ayx;
        let temp_d = (1. - dx) / (1. - dy);
        let bau = (bx + ax * ux) / (by + ay * uy);

        // For assertions
        let comp_a = (1. - ay) / (1. - ax);
        let comp_d = (1. - dx) / (1. - dy);
        assert!(bx >= (a * by * comp_a * comp_d));
        assert!(ux >= (uy * comp_a * comp_d));

        // Calulate the rest of the opinion
        let d = (dx - dy) / (1. - dy);
        let b = (ay_ayx * bau - (ax / ayx) * temp_d) - ((ax / ayx) * temp_d);
        let u = ay_ayx * (temp_d - bau);

        Self::new(b, d, u, a)
    }

    // Undisjunction / UN-OR (Codivision)
    fn codiv(self, other: Self) -> Self {
        // Destructuring to have better readability of the formula
        let BinomialOpinion {
            belief: bx,
            disbelief: dx,
            uncertainty: ux,
            base_rate: ax,
        } = self;

        let BinomialOpinion {
            belief: by,
            disbelief: dy,
            uncertainty: uy,
            base_rate: ay,
        } = other;

        assert!(ax > ay && bx >= by);

        // Calculate factors that are reused many times
        let comp_ax = 1. - ax;
        let comp_ay = 1. - ay;
        let comp_axy = ax - ay;
        let axy = comp_axy / comp_ay;
        let bxy = (1. - bx) / (1. - by);
        let dau = (dx + comp_ax * ux) / (dy + comp_ay * uy);
        let div_a = ay / ax;

        assert!(dx >= (div_a * axy * bxy * dy));
        assert!(ux >= (div_a * bxy * uy));

        let b = (bx - by) / (1. - by);
        let a = comp_axy / comp_ay;
        let d = axy * (dau - bxy);
        let u = axy * (bxy - dau);

        Self::new(b, d, u, a)
    }

    // NOT (Complement)
    fn complement(self) -> Self {
        Self::new(
            self.disbelief,
            self.belief,
            self.uncertainty,
            1. - self.base_rate,
        )
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
        Self::_isclose(self.belief, other.belief, EPSILON)
            && Self::_isclose(self.disbelief, other.disbelief, EPSILON)
            && Self::_isclose(self.uncertainty, other.uncertainty, EPSILON)
            && Self::_isclose(self.base_rate, other.base_rate, EPSILON)
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

impl std::ops::Not for BinomialOpinion {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
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

    #[test]
    fn test_substract() {
        // Test taken from book A. Josang Subjective Logic. chap 6 fig 6.3
        let op1 = BinomialOpinion::new(0.2, 0.6, 0.2, 0.5);
        let op2 = BinomialOpinion::new(0.5, 0.3, 0.2, 0.25);
        let union = BinomialOpinion::new(0.7, 0.1, 0.2, 0.75);

        assert!(BinomialOpinion::_isclose(
            union.prob() - op2.prob(),
            op1.prob(),
            EPSILON
        ));
        dbg!(union.substract(op2), op1);
        assert_eq!(union - op2, op1);
    }

    #[test]
    fn test_complement() {
        // Test taken from book A. Josang Subjective Logic. chap 6 fig 6.5
        let op = BinomialOpinion::new(0.5, 0.1, 0.4, 0.25);
        let expected_result = BinomialOpinion::new(0.1, 0.5, 0.4, 0.75);

        assert!(BinomialOpinion::_isclose(
            op.complement().prob(),
            1.0 - op.prob(),
            EPSILON
        ));
        assert_eq!(op.complement(), expected_result);
        assert_eq!(!op, expected_result);
    }

    #[test]
    fn test_binomial_multiplication() {
        // Test taken from book A. Josang Subjective Logic. chap 7 fig 7.2
        let op1 = BinomialOpinion::new(0.75, 0.15, 0.1, 0.5);
        let op2 = BinomialOpinion::new(0.1, 0.0, 0.9, 0.2);

        let expected_result = BinomialOpinion::new(0.15, 0.15, 0.7, 0.1);

        dbg!(BinomialOpinion::_isclose(
            op1.prob() * op2.prob(),
            expected_result.prob(),
            EPSILON
        ));
        assert_eq!(op1.multiply(op2), expected_result);
        assert_eq!(op1 * op2, expected_result);
    }

    #[test]
    fn test_binomial_comultiplication() {
        // Test taken from book A. Josang Subjective Logic. chap 7 fig 7.3
        let op1 = BinomialOpinion::new(0.75, 0.15, 0.1, 0.5);
        let op2 = BinomialOpinion::new(0.35, 0.0, 0.65, 0.2);

        let expected_result = BinomialOpinion::new(0.84, 0.6, 0.1, 0.6);

        assert_eq!(op1.comult(op2), expected_result);
        let prob = op1.prob() + op2.prob() - (op1.prob() * op2.prob());
        assert!(BinomialOpinion::_isclose(prob, 0.9, EPSILON));
    }


    #[test]
    fn test_binomial_division() {
        // Test taken from book A. Josang Subjective Logic. chap 6 fig 6.2
        let op1 = BinomialOpinion::new(0.05, 0.55, 0.4, 0.75);
        let op2 = BinomialOpinion::new(0.0, 0.8, 0.2, 0.5);

        let expected_result = BinomialOpinion::new(0.05, 0.49, 0.46, 0.5);

        // dbg!(BinomialOpinion::_isclose(
        //     op1.prob() * op2.prob(),
        //     expected_result.prob(),
        //     EPSILON
        // ));
        assert_eq!(op1.divide(op2), expected_result);
        assert_eq!(op1 / op2, expected_result);
    }

    #[test]
    fn test_binomial_codivision() {
        // Test taken from book A. Josang Subjective Logic. chap 7 fig 7.3
        let op1 = BinomialOpinion::new(0.05, 0.55, 0.4, 0.75);
        let op2 = BinomialOpinion::new(0.00, 0.8, 0.2, 0.5);

        let expected_result = BinomialOpinion::new(0.05, 0.49, 0.46, 0.5);

        assert_eq!(op1.codiv(op2), expected_result);
    }

}
