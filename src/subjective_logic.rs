// Define the structure of an Opinion in subjective logic.
pub mod binomial_opinion;


trait SLOpertors {
    // Initialize a new opinion ensuring it's valid.
    fn new(belief: f64, disbelief: f64, uncertainty: f64, base_rate: f64) -> Self;

    /// Calculate the projected probability P(x)
    fn prob(self) -> f64;

    /// Addition operator
    /// ```
    ///
    /// ```
    fn addition(self, other: Self) -> Self;

    // Difference (Subtraction)
    fn substract(self, other: Self) -> Self;

    // Conjunction / AND (Multiplication)
    fn multiply(self, other: Self) -> Self;

    // Unconjunction / UN-AND (Division)
    fn divide(self, other: Self) -> Self;

    // Disjunction / OR (Comultiplication)
    fn comult(self, other: Self) -> Self;

    // Undisjunction / UN-OR (Codivision)
    fn codiv(self, other: Self) -> Self;

    // NOT (Complement)
    fn complement(self) -> Self;

    // Modus ponens (Deduction)
    fn deduct(self, other: Self) -> Self;

    // Contraposition (Subjective Bayes' theorem)
    fn subjective_bayes(self, other: Self) -> Self;

    // Modus tollens (Abduction)
    fn abduct(self, other: Self) -> Self;

    fn cumul_fuse(self, other: Self) -> Self;

    fn constrained_fuse(self, other: Self) -> Self;

    /// Return whether the two numbers `a` and `b` are close.
    /// Closeness is determined by the `epsilon` parameter - 
    /// the numbers are considered close if the difference between them
    /// is no more than epsilon * max(abs(a), abs(b)).
    fn _isclose(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() <= a.abs().max(b.abs()) * epsilon
    }
}
