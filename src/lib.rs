extern crate arithmetic;

use arithmetic::*;

use std::f32;
use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct DualNum<T>(pub T, pub T);

impl<T> DualNum<T> where T: PseudoField {
  pub fn constant(value: T) -> DualNum<T> {
    DualNum(value, T::zero())
  }

  pub fn param(value: T) -> DualNum<T> {
    DualNum(value, T::one())
  }
}

impl<T> DualNum<T> {
  pub fn real(self) -> T {
    self.0
  }

  pub fn dual(self) -> T {
    self.1
  }
}

impl<T> DualNum<T> where T: Copy + PseudoField {
  pub fn reciprocal(self) -> DualNum<T> {
    DualNum::constant(T::zero()) / self
  }
}

impl<T> Neg for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn neg(self) -> DualNum<T> {
    DualNum(-self.0, -self.1)
  }
}

impl<T> Add<T> for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn add(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 + rhs, self.1)
  }
}

impl<T> Add for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn add(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl<T> Sub<T> for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn sub(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 - rhs, self.1)
  }
}

impl<T> Sub for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn sub(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 - rhs.0, self.1 - rhs.1)
  }
}

impl<T> Mul<T> for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn mul(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 * rhs, self.1 * rhs)
  }
}

impl<T> Mul for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn mul(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 * rhs.0, self.1 * rhs.0 + self.0 * rhs.1)
  }
}

impl<T> Div<T> for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn div(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 / rhs, self.1 / rhs)
  }
}

impl<T> Div for DualNum<T> where T: Copy + PseudoField {
  type Output = DualNum<T>;

  fn div(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 / rhs.0, (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0))
  }
}

pub trait DualNumExt<T> {
  fn sqrt(self) -> DualNum<T>;
  fn exp(self) -> DualNum<T>;
  fn exp2(self) -> DualNum<T>;
  fn ln(self) -> DualNum<T>;
  fn log2(self) -> DualNum<T>;
  fn log10(self) -> DualNum<T>;
  fn sin(self) -> DualNum<T>;
  fn cos(self) -> DualNum<T>;
  fn tan(self) -> DualNum<T>;
  /*fn asin(self) -> DualNum<T>;
  fn acos(self) -> DualNum<T>;
  fn atan(self) -> DualNum<T>;
  fn atan2(self, other: DualNum<T>) -> DualNum<T>;
  fn sinh(self) -> DualNum<T>;
  fn cosh(self) -> DualNum<T>;
  fn tanh(self) -> DualNum<T>;
  fn asinh(self) -> DualNum<T>;
  fn acosh(self) -> DualNum<T>;
  fn atanh(self) -> DualNum<T>;*/
}

impl DualNumExt<f32> for DualNum<f32> {
  fn sqrt(self) -> DualNum<f32> {
    let y = self.0.sqrt();
    let dy = 0.5 / y;
    DualNum(y, dy * self.1)
  }

  fn exp(self) -> DualNum<f32> {
    let y = self.0.exp();
    let dy = y;
    DualNum(y, dy * self.1)
  }

  fn exp2(self) -> DualNum<f32> {
    let y = self.0.exp2();
    let dy = f32::consts::LN_2 * y;
    DualNum(y, dy * self.1)
  }

  fn ln(self) -> DualNum<f32> {
    let y = self.0.ln();
    let dy = 1.0 / self.0;
    DualNum(y, dy * self.1)
  }

  fn log2(self) -> DualNum<f32> {
    let y = self.0.log2();
    let dy = 1.0 / (f32::consts::LN_2 * self.0);
    DualNum(y, dy * self.1)
  }

  fn log10(self) -> DualNum<f32> {
    let y = self.0.log10();
    let dy = 1.0 / (f32::consts::LN_10 * self.0);
    DualNum(y, dy * self.1)
  }

  fn sin(self) -> DualNum<f32> {
    let y = self.0.sin();
    let dy = self.0.cos();
    DualNum(y, dy * self.1)
  }

  fn cos(self) -> DualNum<f32> {
    let y = self.0.cos();
    let dy = -self.0.sin();
    DualNum(y, dy * self.1)
  }

  fn tan(self) -> DualNum<f32> {
    let y = self.0.tan();
    let c = self.0.cos();
    let dy = 1.0 / (c * c);
    DualNum(y, dy * self.1)
  }
}
