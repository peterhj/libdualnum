extern crate arithmetic;

use arithmetic::*;

use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct DualNum<T>(pub T, pub T);

impl<T> DualNum<T> where T: Field {
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

impl<T> DualNum<T> where T: Copy + Field {
  pub fn reciprocal(self) -> DualNum<T> {
    DualNum::constant(T::zero()) / self
  }
}

impl<T> Neg for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn neg(self) -> DualNum<T> {
    DualNum(-self.0, -self.1)
  }
}

impl<T> Add<T> for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn add(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 + rhs, self.1)
  }
}

impl<T> Add for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn add(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl<T> Sub<T> for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn sub(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 - rhs, self.1)
  }
}

impl<T> Sub for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn sub(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 - rhs.0, self.1 - rhs.1)
  }
}

impl<T> Mul<T> for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn mul(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 * rhs, self.1 * rhs)
  }
}

impl<T> Mul for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn mul(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 * rhs.0, self.1 * rhs.0 + self.0 * rhs.1)
  }
}

impl<T> Div<T> for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn div(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 / rhs, self.1 / rhs)
  }
}

impl<T> Div for DualNum<T> where T: Copy + Field {
  type Output = DualNum<T>;

  fn div(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 / rhs.0, (self.1 * rhs.0 - self.0 * rhs.1) / (rhs.0 * rhs.0))
  }
}
