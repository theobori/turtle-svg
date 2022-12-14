use std::ops::{
    Add,
    Sub,
    Mul,
    Div
};

use crate::angle::Angle;

/// Distances type
pub type Distance = f64;

/// Location on the plan (x, y)
#[derive(Clone, Copy)]
pub struct Pos<T: Sized + Add + Mul + Div + Sub>(pub T, pub T);

/// Macro to implement a type for the Pos struct
macro_rules! pos_impl {
    ($t: ty) => {
        impl From<($t, $t)> for Pos<$t> {
            fn from(tuple: ($t, $t)) -> Self {
                Self(
                    tuple.0 as $t,
                    tuple.1 as $t
                )
            }
        }

        impl Default for Pos<$t> {
            fn default() -> Self {
                Self(0 as $t, 0 as $t)
            }
        }

        impl Into<($t, $t)> for Pos<$t> {
            fn into(self) -> ($t, $t) {
                (
                    self.0 as $t,
                    self.1 as $t
                )
            }
        }

        impl Pos<$t> {
            /// Return another Pos depending of an angle and a Pos
            ///
            /// `t` is the heading angle
            ///
            /// `a` is the turn angle
            ///
            /// `d` is the distance
            pub fn next_pos_turn(&self, t: Angle, a: Angle, d: Distance) -> Pos<$t> {
                let (x0, y0) = (self.0, self.1);
                let direction = (a.radian() + t.radian());

                let x1 = x0 + (d * direction.cos()) as $t;
                let y1 = y0 + (d * direction.sin()) as $t;
        
                Self(x1, y1)
            }
        }
    };
}

// Implementing number types for Pos
pos_impl!(i16);
pos_impl!(i32);
pos_impl!(i64);
pos_impl!(i128);
pos_impl!(f64);
