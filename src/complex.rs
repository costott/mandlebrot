use std::ops::{Add, Mul, Div, Sub};
use astro_float::BigFloat;
use astro_float::RoundingMode;

fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..n+1 {
        result *= i;
    }
    result
}

fn choose(n: u32, r: u32) -> u32 {
    assert!(n >= r);
    if r == 0 || r == n {return 1}
    if r == 1 || r == n-1 {return n}
    factorial(n) / (factorial(r)*factorial(n-r))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(real: f64, im: f64) -> Complex {
        Complex { real, im }
    }

    pub fn square(&self) -> Self {
        Complex::new(
            self.real*self.real - self.im*self.im, 
            2f64*self.real*self.im
        )
    }

    #[allow(unused)]
    /// raise the complex number to a given power
    pub fn pow(&self, n: u32) -> Self {
        let (mut real , mut im) = (0., 0.);
        for i in 0..=n {
            let b_pow = n-i;
            let coefficient = choose(n, i) as f64 * self.im.powi(b_pow as i32) * self.real.powi(i as i32);
            match b_pow % 4 {
                0 => {real += coefficient},
                1 => {im += coefficient},
                2 => {real -= coefficient},
                3 => {im -= coefficient},
                _ => {}
            }
        }

        Complex::new( real, im )
    }

    pub fn abs_squared(&self) -> f64 {
        self.real.powi(2) + self.im.powi(2)
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            im: -self.im
        }
    }
}
impl Add for Complex { 
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            im: self.im + other.im,
        }
    }
}
impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            im: self.im - rhs.im
        }
    }
}
impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.im * rhs.im,
            im: self.real * rhs.im + self.im * rhs.real
        }
    }
}
impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real * rhs,
            im: self.im * rhs
        }
    }
}
impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self * rhs.conjugate();
        let d = rhs.real * rhs.real + rhs.im * rhs.im;
        Complex {
            real: n.real / d,
            im: n.im / d
        }
    }
}
impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real / rhs,
            im: self.im / rhs
        }
    }
}

const P: usize = 128;
const RM: RoundingMode = RoundingMode::ToEven;
#[derive(Debug, Clone, PartialEq)]
pub struct BigComplex {
    real: BigFloat,
    im: BigFloat
}
impl BigComplex {
    pub fn new(real: BigFloat, im: BigFloat) -> BigComplex {
        BigComplex { real, im }
    }
    
    /// creates a new BigComplex number from a 
    /// given normal complex number
    pub fn from_complex(c: Complex) -> BigComplex {
        BigComplex {
            real: BigFloat::from_f64(c.real, P),
            im: BigFloat::from_f64(c.im, P)
        }
    }

    /// creates a new BigComplex number from
    /// given f64 numbers
    pub fn from_f64s(real: f64, im: f64) -> BigComplex {
        BigComplex {
            real: BigFloat::from_f64(real, P),
            im: BigFloat::from_f64(im, P)
        }
    }

    pub fn square(&self) -> Self {
        BigComplex::new(
            self.real.mul(&self.real, P, RM).sub(
                &self.im.mul(&self.im, P, RM), P, RM
            ),
            BigFloat::from_u8(2, P).mul(&self.real, P, RM).mul(&self.im, P, RM)
        )
    }

    pub fn abs_squared(&self) -> BigFloat {
        self.real.powi(2, P, RM).add(&self.im.powi(2, P, RM), P, RM)
    }

    pub fn conjugate(&self) -> BigComplex {
        BigComplex::new(
            self.real.mul(&BigFloat::from_i8(1, P), P, RM),
            self.im.mul(&BigFloat::from_i8(-1, P), P, RM)
        )
    }
}
impl Add for BigComplex {
    type Output = BigComplex;

    fn add(self, rhs: Self) -> Self::Output {
        BigComplex {
            real: self.real.add(&rhs.real, P, RM),
            im: self.im.add(&rhs.im, P, RM)
        }
    }
}
impl Sub for BigComplex {
    type Output = BigComplex;

    fn sub(self, rhs: Self) -> Self::Output {
        BigComplex {
            real: self.real.sub(&rhs.real, P, RM),
            im: self.im.sub(&rhs.im, P, RM)
        }
    }
}
impl Mul for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: Self) -> Self::Output {
        BigComplex {
            real: self.real.mul(&rhs.real, P, RM).sub(
                &self.im.mul(&rhs.im, P, RM), P, RM
            ),
            im: self.real.mul(&rhs.im, P, RM).add(
                &self.im.mul(&rhs.real, P, RM), P, RM
            )
        }
    }
}
impl Mul<f64> for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = BigFloat::from_f64(rhs, P);
        BigComplex {
            real: self.real.mul(&rhs, P, RM),
            im: self.im.mul(&rhs, P, RM)
        }
    }
}
impl Div for BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self * rhs.conjugate();
        let d = rhs.abs_squared();
        BigComplex {
            real: n.real.div(&d, P, RM),
            im: n.im.div(&d, P, RM)
        }
    }
}
impl Div<f64> for BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: f64) -> Self::Output {
        let rhs = BigFloat::from_f64(rhs, P);
        BigComplex {
            real: self.real.div(&rhs, P, RM),
            im: self.im.div(&rhs, P, RM)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Complex::new(1f64, 2f64);
        let b = Complex::new(3f64, 4f64);
        assert_eq!(a + b, Complex::new(4f64, 6f64));
    }

    #[test]
    fn add_bigcomplex() {
        let a = BigComplex::from_f64s(1f64, 2f64);
        let b = BigComplex::from_f64s(3f64, 4f64);
        assert_eq!(a + b, BigComplex::from_f64s(4f64, 6f64));
    }

    #[test]
    fn square() {
        let a = Complex::new(1f64, 2f64);
        assert_eq!(a.square(), Complex::new(-3f64, 4f64));
    }

    #[test]
    fn square_bigcomplex() {
        let a = BigComplex::from_f64s(1., 2.);
        let a2 = BigComplex::from_f64s(-3., 4.);
        assert_eq!(a.square(), a2);
    }

    #[test]
    fn power() {
        let a = Complex::new( 2., -5. );
        let a3 = a.pow(3);

        assert_eq!(Complex::new(-142., 65.), a3);
    }

    #[test]
    fn complex_times() {
        let a = Complex::new(3., 5.);
        let b = Complex::new(2., 7.);

        let answer = Complex::new(-29., 31.);

        assert_eq!(a * b, answer);
    }

    #[test]
    fn bigcomplex_times() {
        let a = BigComplex::from_f64s(3., 5.);
        let b = BigComplex::from_f64s(2., 7.);

        let answer = BigComplex::from_f64s(-29., 31.);

        assert_eq!(a * b, answer);
    }

    #[test]
    fn complex_times_float() {
        let a = Complex::new(3., 6.);
        
        let answer = Complex::new(6., 12.);

        assert_eq!(a * 2., answer);
    }

    #[test]
    fn bigcomplex_times_float() {
        let a = BigComplex::from_f64s(3., 6.);
        
        let answer = BigComplex::from_f64s(6., 12.);

        assert_eq!(a * 2., answer);
    }

    #[test]
    fn complex_divide() {
        let a = Complex::new(3., 5.);
        let b = Complex::new(2., 4.);

        let answer = Complex::new(1.3, -0.1);

        assert_eq!(a / b, answer);
    }
    
    #[test]
    fn complex_divide_float() {
        let a = Complex::new(3., 18.);

        let answer = Complex::new(1., 6.);

        assert_eq!(a / 3., answer);
    }
    
    #[test]
    fn bigcomplex_divide_float() {
        let a = BigComplex::from_f64s(3., 18.);

        let answer = BigComplex::from_f64s(1., 6.);

        assert_eq!(a / 3., answer);
    }
}