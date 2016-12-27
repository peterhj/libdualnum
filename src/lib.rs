#![feature(zero_one)]

use std::num::{Zero, One};
use std::ops::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct DualNum<T>(pub T, pub T);

impl<T> DualNum<T> where T: Zero + One {
  pub fn constant(value: T) -> DualNum<T> {
    DualNum(value, T::zero())
  }

  pub fn param(value: T) -> DualNum<T> {
    DualNum(value, T::one())
  }
}

impl<T> DualNum<T> where T: Copy {
  pub fn primal(&self) -> T {
    self.0
  }

  pub fn dual(&self) -> T {
    self.1
  }
}

impl<T> Neg for DualNum<T> where T: Copy + Neg<Output=T> {
  type Output = DualNum<T>;

  fn neg(self) -> DualNum<T> {
    //let DualNum(x, y): DualNum<T> = self;
    //DualNum(-x, -y)
    DualNum(-self.0, -self.1)
  }
}

impl<T> Add<T> for DualNum<T> where T: Copy + Add<Output=T> {
  type Output = DualNum<T>;

  fn add(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 + rhs, self.1)
  }
}

impl<T> Add for DualNum<T> where T: Copy + Add<Output=T> {
  type Output = DualNum<T>;

  fn add(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl<T> Sub<T> for DualNum<T> where T: Copy + Sub<Output=T> {
  type Output = DualNum<T>;

  fn sub(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 - rhs, self.1)
  }
}

impl<T> Sub for DualNum<T> where T: Copy + Sub<Output=T> {
  type Output = DualNum<T>;

  fn sub(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 - rhs.0, self.1 - rhs.1)
  }
}

impl<T> Mul<T> for DualNum<T> where T: Copy + Mul<Output=T> {
  type Output = DualNum<T>;

  fn mul(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 * rhs, self.1 * rhs)
  }
}

impl<T> Mul for DualNum<T> where T: Copy + Add<Output=T> + Mul<Output=T> {
  type Output = DualNum<T>;

  fn mul(self, rhs: DualNum<T>) -> DualNum<T> {
    DualNum(self.0 * rhs.0, self.0 * rhs.1 + self.1 * rhs.0)
  }
}

impl<T> Div<T> for DualNum<T> where T: Copy + Div<Output=T> {
  type Output = DualNum<T>;

  fn div(self, rhs: T) -> DualNum<T> {
    DualNum(self.0 / rhs, self.1 / rhs)
  }
}

impl<T> Div for DualNum<T> where T: Copy + Neg<Output=T> + Add<Output=T> + Mul<Output=T> + Div<Output=T> {
  type Output = DualNum<T>;

  fn div(self, rhs: DualNum<T>) -> DualNum<T> {
    self * DualNum(rhs.0, -rhs.1) / (rhs.0 * rhs.0)
  }
}
