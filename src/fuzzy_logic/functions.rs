/// Trapezoidal distribution function
pub fn triangular(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        if x <= a || x >= c {
            0.0
        } else if x <= b {
            (x - a) / (b - a)
        } else {
            (c - x) / (c - b)
        }
    })
}

/// Trapezoidal distribution function
pub fn trapezoidal(a: f64, b: f64, c: f64, d: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        if x <= a || x >= d {
            0.0
        } else if x <= b {
            (x - a) / (b - a)
        } else if x <= c {
            1.0
        } else {
            (d - x) / (d - c)
        }
    })
}

/// Gaussian distribution
pub fn gaussian(c: f64, sigma: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| (-((x - c).powi(2)) / (2.0 * sigma.powi(2))).exp())
}

/// Sigmoid function to evaluate an x
/// `a` controls the curve steepness and `c` shifts horizontaly the sigmoid.
/// For a standard sigmoid curve call like this :
/// ```
/// # use rtee::prelude::*;
/// # fn main() {
/// let sigmoid_func = sigmoid(1.0, 0.0);
/// assert_eq!(sigmoid_func(0.0), 0.5)
/// # }
/// ```
pub fn sigmoid(a: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| 1.0 / (1.0 + (-a * (x - c)).exp()))
}

/// Bell distribution
pub fn bell(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| 1.0 / (1.0 + ((x - c) / a).abs().powf(2.0 * b)))
}

/// Center of gravity
fn cog(x: f64, y: f64) -> f64 {
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        assert_eq!(sigmoid(1.0, 0.0)(0.0), 0.5);
    }

    #[test]
    fn test_gaussian() {
        assert_eq!(gaussian(0.0, 1.0)(0.0), 1.0);
    }

    #[test]
    fn test_bell() {
        assert_eq!(bell(1.0, 1.0, 0.0)(0.0), 1.0);
    }

    #[test]
    fn test_cog() {
        assert_eq!(cog(1.0, 1.0), 1.0);
    }
}
