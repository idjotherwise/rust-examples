pub struct Polynomial<const N: usize> {
    // Coefs of a polynomial
    // The ith element is the coefficient of x**i
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    pub fn new(coefficients: [f64; N]) -> Self {
        Self { coefficients }
    }

    // Evaluate the polynomial at x
    pub fn eval(&self, x: f64) -> f64 {
        // Horners method apparently
        let mut sum = 0.0;
        for i in 0..N {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}
