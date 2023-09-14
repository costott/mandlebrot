// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use std::ops::{Add, Mul, Div, Sub, Neg};
use dashu_float::{FBig, round::mode};

use crate::{lerpf64_pow, lerp_fbig_pow};

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

pub trait ComplexNumber {
    fn square(&self) -> Self;
    /// the absolute value of the complex number, squared
    fn abs_squared(&self) -> f64;
    fn conjugate(&self) -> Self;
    fn arg(&self) -> f64;
    /// the squared distance between the complex number and another
    fn distance2_to(&self, other: ComplexType) -> f64;
    fn update_real_from_string(&mut self, new: String);
    fn update_im_from_string(&mut self, new: String);
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexType {
    Double(Complex),
    Big(BigComplex)
}
impl ComplexType {
    /// gives a complex number with the given values of the same type as other
    pub fn same_type(real: f64, im: f64, other: ComplexType) -> ComplexType {
        match other {
            ComplexType::Double(_) => ComplexType::Double(Complex::new(real, im)),
            ComplexType::Big(_) => ComplexType::Big(BigComplex::from_f64s(real, im))
        }
    }

    /// returns the real part of the number as an f64, regarless of type
    pub fn real_f64(&self) -> f64 {
        match self {
            ComplexType::Double(c) => c.real,
            ComplexType::Big(c) => c.real.to_f64().value()
        }
    }

    /// returns the real part of the number as an FBig, regarless of type
    pub fn real_fbig(&self) -> FBig {
        match self {
            ComplexType::Double(c) => FBig::try_from(c.real).unwrap(),
            ComplexType::Big(c) => c.real.clone()
        }
    }   

   
    pub fn real_string(&self) -> String {
        match self {
            ComplexType::Double(c) => c.real.to_string(),
            ComplexType::Big(c) => c.real.clone().with_base_and_precision::<10>(c.real.precision()).value().to_string()
        }
    }

    /// returns the imaginary part of the number as an f64, regarless of type
    pub fn im_f64(&self) -> f64 {
        match self {
            ComplexType::Double(c) => c.im,
            ComplexType::Big(c) => c.im.to_f64().value()
        }
    }

    /// returns the imaginary part of the number as an FBig, regarless of type
    pub fn im_fbig(&self) -> FBig {
        match self {
            ComplexType::Double(c) => FBig::try_from(c.im).unwrap(),
            ComplexType::Big(c) => c.im.clone()
        }
    } 

    pub fn im_string(&self) -> String {
        match self {
            ComplexType::Double(c) => c.im.to_string(),
            ComplexType::Big(c) => c.im.clone().with_base_and_precision::<10>(c.im.precision()).value().to_string()
        }
    }

    /// returns the complex number of other if it's a double
    /// 
    /// # Panics
    /// The other is a `ComplexType::Big`
    #[allow(unused)]
    fn unpack_double(&self, other: ComplexType) -> Complex {
        match other {
            ComplexType::Double(c) => c,
            ComplexType::Big(_) => panic!("Can't perform operations on different complex types - attempted to unpack double")
        }
    }

    /// returns the complex number of other if it's big
    /// 
    /// # Panics
    /// The other is a `ComplexType::Double`
    #[allow(unused)]
    fn unpack_big(&self, other: ComplexType) -> BigComplex {
        match other {
            ComplexType::Double(_) => panic!("Can't perform operations on different complex types - attempted to unpack big"),
            ComplexType::Big(c) => c,
        }
    }

    /// converts the complex type to be big
    pub fn make_big(&self) -> ComplexType {
        match &self {
            ComplexType::Double(c) => ComplexType::Big(BigComplex::from_complex(*c)),
            ComplexType::Big(_) => self.clone()
        }
    }

    /// converts the complex type to be double
    pub fn make_double(&self) -> ComplexType {
        match &self {
            ComplexType::Big(c) => ComplexType::Double(c.to_complex()),
            ComplexType::Double(_) => self.clone(),
        }
    }

    pub fn update_real_from_string(&mut self, new: String) {
        match self {
            ComplexType::Double(ref mut c) => c.update_real_from_string(new.clone()),
            ComplexType::Big(ref mut c) => c.update_real_from_string(new.clone())
        }
        match self {
            ComplexType::Big(_) => {},
            ComplexType::Double(c) => {
                if c.real.to_string() != new {
                    *self = self.make_big();
                    self.update_real_from_string(new);
                }
            }
        }
    }
    pub fn update_im_from_string(&mut self, new: String) {
        match self {
            ComplexType::Double(ref mut c) => c.update_im_from_string(new.clone()),
            ComplexType::Big(ref mut c) => c.update_im_from_string(new.clone())
        }
        match self {
            ComplexType::Big(_) => {},
            ComplexType::Double(c) => {
                if c.im.to_string() != new {
                    *self = self.make_big();
                    self.update_im_from_string(new);
                }
            }
        }
    }

    pub fn lerp_complex(complex1: &ComplexType, complex2: &ComplexType, percent: f64, arb_precision: bool, p: f64) -> ComplexType {
        match arb_precision {
            true => {
                let t = FBig::try_from(percent).unwrap();
                let p = FBig::try_from(p).unwrap();
                ComplexType::Big(BigComplex::new(
                    lerp_fbig_pow(complex1.real_fbig(), complex2.real_fbig(), &t, &p),
                    lerp_fbig_pow(complex1.im_fbig(), complex2.im_fbig(), &t, &p)
                ))
            },
            false => {
                ComplexType::Double(Complex::new(
                    lerpf64_pow(complex1.real_f64(), complex2.real_f64(), percent, p),
                    lerpf64_pow(complex1.im_f64(), complex2.im_f64(), percent, p),
                ))
            }
        }
    }
}
// impl ComplexNumber for ComplexType {
//     fn square(&self) -> Self {
//         match self {
//             Self::Double(c) => Self::Double(c.square()),
//             Self::Big(c) => Self::Big(c.square())
//         }
//     }

//     fn abs_squared(&self) -> f64 {
//         match self {
//             Self::Double(c) => c.abs_squared(),
//             Self::Big(c) => c.abs_squared()
//         }
//     }

//     fn conjugate(&self) -> Self {
//         match self {
//             Self::Double(c) => Self::Double(c.conjugate()),
//             Self::Big(c) => Self::Big(c.conjugate())
//         }
//     }

//     fn arg(&self) -> f64 {
//         match self {
//             Self::Double(c) => c.arg(),
//             Self::Big(c) => c.arg()
//         }
//     }

//     fn distance2_to(&self, other: ComplexType) -> f64 {
//         match self {
//             Self::Double(c) => c.distance2_to(other),
//             Self::Big(c) => c.distance2_to(other)
//         }
//     }
// }
// impl Add for ComplexType {
//     type Output = ComplexType;

//     fn add(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c + self.unpack_double(rhs)),
//             Self::Big(ref c) => Self::Big(c.clone() + self.unpack_big(rhs))
//         }
//     }
// }
// impl Sub for ComplexType {
//     type Output = ComplexType;

//     fn sub(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c - self.unpack_double(rhs)),
//             Self::Big(ref c) => Self::Big(c.clone() - self.unpack_big(rhs))
//         }
//     }
// }
// impl Mul for ComplexType {
//     type Output = ComplexType;

//     fn mul(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c * self.unpack_double(rhs)),
//             Self::Big(ref c) => Self::Big(c.clone() * self.unpack_big(rhs))
//         }
//     }
// }
// impl Mul<f64> for ComplexType {
//     type Output = ComplexType;

//     fn mul(self, rhs: f64) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c * rhs),
//             Self::Big(c) => Self::Big(c * rhs)
//         }
//     }
// }
// impl Div for ComplexType {
//     type Output = ComplexType;

//     fn div(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c / self.unpack_double(rhs)),
//             Self::Big(ref c) => Self::Big(c.clone() / self.unpack_big(rhs))
//         }
//     }
// }
// impl Div<f64> for ComplexType {
//     type Output = ComplexType;

//     fn div(self, rhs: f64) -> Self::Output {
//         match self {
//             Self::Double(c) => Self::Double(c / rhs),
//             Self::Big(c) => Self::Big(c / rhs)
//         }
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub im: f64,
}
impl Complex {
    pub fn new(real: f64, im: f64) -> Complex {
        Complex { real, im }
    }

    #[allow(unused)]
    /// raise the complex number to a given power
    /// TODO: use demoivre's instead
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

    /// returns the real part of the number as an f64
    pub fn real_f64(&self) -> f64 {
        self.real
    }

    /// returns the imaginary part of the number as an f64
    pub fn im_f64(&self) -> f64 {
        self.im
    }
}
impl ComplexNumber for Complex {
    fn square(&self) -> Self {
        Complex::new(
            self.real*self.real - self.im*self.im, 
            2f64*self.real*self.im
        )
    }

    fn abs_squared(&self) -> f64 {
        self.real.powi(2) + self.im.powi(2)
    }

    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            im: -self.im
        }
    }

    /// the argument of the complex number between [-pi, pi]
    fn arg(&self) -> f64 {
        f64::atan2(self.im, self.real)
    }

    fn distance2_to(&self, other: ComplexType) -> f64 {
        match other {
            ComplexType::Double(c) => (*self-c).abs_squared(),
            ComplexType::Big(c) => (BigComplex::from_complex(*self)-c).abs_squared()
        }
    }

    fn update_real_from_string(&mut self, new: String) {
        if let Ok(new) = new.parse() {
            self.real = new;
        }
    }
    fn update_im_from_string(&mut self, new: String) {
        if let Ok(new) = new.parse() {
            self.im = new;
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
impl<'l> Div<f64> for &'l Complex {
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            real: self.real / rhs,
            im: self.im / rhs
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BigComplex {
    pub real: FBig,
    pub im: FBig
}
impl BigComplex {
    pub fn new(real: FBig, im: FBig) -> BigComplex {
        BigComplex { real, im }
    }

    /// creates a new BigComplex number from a normal 
    /// double precision Complex number
    pub fn from_complex(c: Complex) -> BigComplex {
        BigComplex {
            real: FBig::try_from(c.real).unwrap().with_precision(64).value(),
            im: FBig::try_from(c.im).unwrap().with_precision(64).value()
        }
    }

    /// creates a new BigComplex number from
    /// given f64 numbers
    pub fn from_f64s(real: f64, im: f64) -> BigComplex {
        BigComplex {
            real: FBig::try_from(real).unwrap(),
            im: FBig::try_from(im).unwrap()
        }
    }

    pub fn from_string_base10(real: &str, im: &str) -> BigComplex {
        // BigComplex { 
        //     real: FBig::from_str_native(real).unwrap().with_precision(100).value(), 
        //     im: FBig::from_str_native(im).unwrap().with_precision(100).value()
        // }
        BigComplex {
            real: FBig::<mode::Zero, 10>::from_str_native(real).unwrap().with_base::<2>().value(),
            im: FBig::<mode::Zero, 10>::from_str_native(im).unwrap().with_base::<2>().value()
        }
    }

    /// creates a new double precision Complex number from
    /// the BigComplex number
    pub fn to_complex(&self) -> Complex {
        Complex {
            real: self.real.to_f64().value(),
            im: self.im.to_f64().value()
        }
    }

    /// returns the real part of the number as an f64
    pub fn real_f64(&self) -> f64 {
        self.real.to_f64().value()
    }

    /// returns the imaginary part of the number as an f64
    pub fn im_f64(&self) -> f64 {
        self.im.to_f64().value()
    }
}
impl ComplexNumber for BigComplex {
    fn square(&self) -> Self {
        BigComplex { 
            real: self.real.square() - self.im.square(), 
            im: (FBig::ONE + FBig::ONE) * &self.real * &self.im
        }
    }

    fn abs_squared(&self) -> f64 {
        let abs = self.real.square() + self.im.square();
        abs.to_f64().value()
    }

    fn conjugate(&self) -> Self {
        BigComplex {
            real: self.real.clone(),
            im: self.im.clone().neg()
        }
    }

    fn arg(&self) -> f64 {
        f64::atan2(self.im.to_f64().value(), self.real.to_f64().value())
    }

    fn distance2_to(&self, other: ComplexType) -> f64 {
        match other {
            ComplexType::Double(c) => (self.clone()-BigComplex::from_complex(c)).abs_squared(),
            ComplexType::Big(c) => (self.clone()-c).abs_squared()
        }
    }

    fn update_real_from_string(&mut self, new: String) {
        if let Ok(new) = FBig::<mode::Zero, 10>::from_str_native(&new) {
            self.real = new.with_base::<2>().value();
        }
    }
    fn update_im_from_string(&mut self, new: String) {
        if let Ok(new) = FBig::<mode::Zero, 10>::from_str_native(&new) {
            self.im = new.with_base::<2>().value();
        }
    }
}
impl Add for BigComplex { 
    type Output = BigComplex;

    fn add(self, other: BigComplex) -> Self::Output {
        BigComplex {
            real: self.real + other.real,
            im: self.im + other.im,
        }
    }
}
impl<'l, 'r> Add<&'r BigComplex> for &'l BigComplex {
    type Output = BigComplex;

    fn add(self, rhs: &'r BigComplex) -> Self::Output {
        BigComplex {
            real: &self.real + &rhs.real,
            im: &self.im + &rhs.im
        }
    }
}
impl<'r> Add<&'r BigComplex> for BigComplex {
    type Output = BigComplex;

    fn add(self, rhs: &'r BigComplex) -> Self::Output {
        BigComplex {
            real: &self.real + &rhs.real,
            im: &self.im + &rhs.im
        }
    }
}
impl Sub for BigComplex {
    type Output = BigComplex;

    fn sub(self, rhs: Self) -> Self::Output {
        BigComplex {
            real: self.real - rhs.real,
            im: self.im - rhs.im
        }
    }
}
impl<'l, 'r> Sub<&'r BigComplex> for &'l BigComplex {
    type Output = BigComplex;

    fn sub(self, rhs: &'r BigComplex) -> Self::Output {
        BigComplex {
            real: &self.real - &rhs.real,
            im: &self.im - &rhs.im
        }
    }
}
impl Mul for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: Self) -> Self::Output {
        BigComplex {
            real: &self.real * &rhs.real - &self.im * &rhs.im,
            im: self.real * rhs.im + self.im * rhs.real
        }
    }
}
impl<'l, 'r> Mul<&'r BigComplex> for &'l BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: &'r BigComplex) -> Self::Output {
        BigComplex {
            real: &self.real * &rhs.real - &self.im * &rhs.im,
            im: &self.real * &rhs.im + &self.im * &rhs.real
        }
    }
}
impl<'l> Mul<BigComplex> for &'l BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: BigComplex) -> Self::Output {
        BigComplex {
            real: &self.real * &rhs.real - &self.im * &rhs.im,
            im: &self.real * &rhs.im + &self.im * &rhs.real
        }
    }
}
impl Mul<f64> for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = FBig::try_from(rhs).unwrap();
        BigComplex {
            real: self.real * &rhs,
            im: self.im * rhs
        }
    }
}
impl<'l> Mul<f64> for &'l BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = FBig::try_from(rhs).unwrap();
        BigComplex {
            real: &self.real * &rhs,
            im: &self.im * rhs
        }
    }
}
impl Mul<dashu_float::FBig> for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: dashu_float::FBig) -> Self::Output {
        BigComplex {
            real: self.real * &rhs,
            im: self.im * rhs
        }
    }
}
impl<'r> Mul<&'r dashu_float::FBig> for BigComplex {
    type Output = BigComplex;

    fn mul(self, rhs: &dashu_float::FBig) -> Self::Output {
        BigComplex {
            real: &self.real * rhs,
            im: &self.im * rhs
        }
    }
}
impl Div for BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self * rhs.conjugate();
        let d = rhs.real.square() + rhs.im.square();
        BigComplex {
            real: n.real / &d,
            im: n.im / d
        }
    }
}
impl<'l, 'r> Div<&'r BigComplex> for &'l BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: &'r BigComplex) -> Self::Output {
        let n = self * &rhs.conjugate();
        let d = rhs.real.square() + rhs.im.square();
        BigComplex {
            real: n.real / &d,
            im: n.im / d
        }
    }
}
impl Div<f64> for BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: f64) -> Self::Output {
        let rhs = FBig::try_from(rhs).unwrap();
        BigComplex {
            real: self.real / &rhs,
            im: self.im / rhs
        }
    }
}
impl<'l> Div<f64> for &'l BigComplex {
    type Output = BigComplex;

    fn div(self, rhs: f64) -> Self::Output {
        let rhs = FBig::try_from(rhs).unwrap();
        BigComplex {
            real: &self.real / &rhs,
            im: &self.im / rhs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn create_big_complex() {
        let a = Complex::new(0.01, 1.23);
        let c = BigComplex::from_complex(a);
        
        assert_eq!(c.to_complex(), a);
    }

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
    fn arg() {
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(-1.0, 0.0);
        assert_eq!(a.arg(), PI/4.);
        assert_eq!(b.arg(), PI);
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