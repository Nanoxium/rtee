/// Trapezoidal distribution function
pub fn triangular(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| {
        if x <= a || x >=c {
            0.0
        } else if x <= b {
            (x-a) / (b-a)
        } else {
            (c - x) / (c -b)
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
pub fn sigmoid(a: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| 1.0 / (1.0 + (-a * (x - c)).exp()))
}

/// Bell distribution 
pub fn bell(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| 1.0 / (1.0 + ((x - c) / a).abs().powf(2.0 * b)))
}
