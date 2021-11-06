use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

trait ApproxEq{
    fn approx_eq(self, other: Self, tolerance: f64) -> bool;
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Complex { real, imaginary }
    }

    pub fn re(self) -> f64{
        self.real
    }

    pub fn im(self) -> f64{
        self.imaginary
    }

    pub fn modulus(&self) -> f64 {
        (self.real.powf(2.0) + self.imaginary.powf(2.0)).sqrt()
    }

    pub fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Self {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Self {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Self {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl ApproxEq for Complex{
    fn approx_eq(self, other: Self, tolerance: f64) -> bool{
        (self.real - other.real).abs() < tolerance && (self.imaginary - other.imaginary).abs() < tolerance
    }
}

impl ApproxEq for f64{
    fn approx_eq(self, other: Self, tolerance: f64) -> bool{
        (self-other).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TOLERANCE: f64 = f64::EPSILON;

    #[test]
    fn test_modulus(){
        let z = Complex::new(0.0, 0.0);
        let w = Complex::new(1.0, 1.0);
        let v = Complex::new(-3.0, -4.0);

        assert!(z.modulus().approx_eq(0.0, TOLERANCE));
        assert!(w.modulus().approx_eq(2.0_f64.sqrt(), TOLERANCE));
        assert!(v.modulus().approx_eq(5.0, TOLERANCE));
    }

    #[test]
    fn test_conjugate(){
        let z = Complex::new(1.0, 1.0);
        let z_conjugate = Complex::new(1.0, -1.0);

        assert!(z.conjugate().approx_eq(z_conjugate, TOLERANCE));
    }
    
    #[test]
    fn test_add() {
        let z = Complex::new(1.0, 2.0);
        let w = Complex::new(2.0, -3.0);
        let z_plus_w = Complex::new(3.0, -1.0);

        assert!((z+w).approx_eq(z_plus_w, TOLERANCE));
    }

    #[test]
    fn test_sub() {
        let z = Complex::new(1.0, 2.0);
        let w = Complex::new(2.0, -3.0);
        let z_minus_w = Complex::new(-1.0, 5.0);
        
        assert!((z-w).approx_eq(z_minus_w, TOLERANCE));
    }

    #[test]
    fn test_mul() {
        let z = Complex::new(1.0, 1.0);
        let w = Complex::new(0.0, 1.0);
        let z_times_w = Complex::new(-1.0, 1.0);

        assert!((z*w).approx_eq(z_times_w, TOLERANCE));
    }
}
