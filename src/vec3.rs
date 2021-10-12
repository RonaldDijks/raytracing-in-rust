#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn normalized(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Vec3(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().normalized()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }
}

macro_rules! impl_binary_operation {
    ($operation:ident $op_fun: ident $op_symbol:tt) => {
        // &Vec `op` &Vec
        impl<'a, 'b> $operation<&'a Vec3> for &'b Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: &'a Vec3) -> Vec3 {
                Vec3(
                    self.0 $op_symbol other.0,
                    self.1 $op_symbol other.1,
                    self.2 $op_symbol other.2
                )
            }
        }

        // Vec `op` &Vec
        impl<'a> $operation<&'a Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: &'a Vec3) -> Vec3 {
                Vec3(
                    self.0 $op_symbol other.0,
                    self.1 $op_symbol other.1,
                    self.2 $op_symbol other.2
                )
            }
        }
        // &Vec `op` Vec
        impl<'a> $operation<Vec3> for &'a Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: Vec3) -> Vec3 {
                Vec3(
                    self.0 $op_symbol other.0,
                    self.1 $op_symbol other.1,
                    self.2 $op_symbol other.2
                )
            }
        }

        // Vec `op` Vec
        impl $operation<Vec3> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: Vec3) -> Vec3 {
                Vec3(
                    self.0 $op_symbol other.0,
                    self.1 $op_symbol other.1,
                    self.2 $op_symbol other.2
                )
            }
        }

        // &Vec3 `op` f64
        impl <'a> $operation<f64> for &'a Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: f64) -> Vec3 {
                Vec3(
                    self.0 $op_symbol other,
                    self.1 $op_symbol other,
                    self.2 $op_symbol other,
                )
            }
        }

        // Vec3 `op` f64
        impl <'a> $operation<f64> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: f64) -> Vec3 {
                &self $op_symbol other
            }
        }

        // f64 `op` Vec3
        impl <'a> $operation<Vec3> for f64 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: Vec3) -> Vec3 {
                &other $op_symbol self
            }
        }

        // f64 `op` Vec3
        impl <'a> $operation<&'a Vec3> for f64 {
            type Output = Vec3;

            #[inline]
            fn $op_fun(self, other: &'a Vec3) -> Vec3 {
                other $op_symbol self
            }
        }
    };
}

macro_rules! impl_unary_operation {
    ($operation:ident $op_fun:ident $op_symbol:tt) => {
      impl<'a> $operation for &'a Vec3 {
        type Output = Vec3;

        #[inline]
        fn $op_fun(self) -> Vec3 {
          Vec3 (
            $op_symbol self.0,
            $op_symbol self.1,
            $op_symbol self.2,
          )
        }
      }

      impl $operation for Vec3 {
        type Output = Vec3;

        #[inline]
        fn $op_fun(self) -> Vec3 {
          $op_symbol &self
        }
      }
    };
}

macro_rules! impl_op_assign {
    ($op_type:ident $op_fun:ident $op_symbol:tt) => {
      impl<'a> $op_type<&'a Vec3> for Vec3 {
        fn $op_fun(&mut self, other: &'a Vec3) {
          *self = Vec3 (
            self.0 $op_symbol other.0,
            self.0 $op_symbol other.0,
            self.0 $op_symbol other.0,
          );
        }
      }

      impl $op_type for Vec3 {
        #[inline]
        fn $op_fun(&mut self, other: Vec3) {
          *self = *self $op_symbol &other
        }
      }
    };
  }

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use rand::Rng;

impl_binary_operation!(Add add +);
impl_binary_operation!(Sub sub -);
impl_binary_operation!(Mul mul *);
impl_binary_operation!(Div div /);

impl_op_assign!(AddAssign add_assign +);
impl_op_assign!(SubAssign sub_assign -);
impl_op_assign!(MulAssign mul_assign *);
impl_op_assign!(DivAssign div_assign /);

impl_unary_operation!(Neg neg -);
