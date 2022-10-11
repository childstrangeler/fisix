use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2d {
    x: usize,
    y: usize,
}

macro_rules! impl_op {
	($trait: ident, $fn: ident, $op: tt) => {
impl $trait<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn $fn(self, rhs: Vec2d) -> Self::Output {
        Self {
            x: self.x $op rhs.x,
            y: self.y $op rhs.y,
        }
    }
}

	};
}

impl_op!(Add, add, +);
impl_op!(Mul, mul, *);
impl_op!(Sub, sub, -);
impl_op!(Div, div, /);
