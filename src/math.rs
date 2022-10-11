use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2d {
    pub x: isize,
    pub y: isize,
}

impl Vec2d {
    pub fn dist(&self, other: &Self) -> usize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        ((dx * dx + dy * dy) as f32).sqrt() as usize
    }
}

pub trait ToVec {
    fn vec(self) -> Vec2d;
}

impl ToVec for (isize, isize) {
    fn vec(self) -> Vec2d {
        Vec2d {
            x: self.0,
            y: self.1,
        }
    }
}

impl ToVec for (usize, usize) {
    fn vec(self) -> Vec2d {
        Vec2d {
            x: self.0 as isize,
            y: self.1 as isize,
        }
    }
}

impl Deref for Vec2d {
    type Target = [usize; 2];

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
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

macro_rules! impl_op_assign {
	($trait: ident, $fn: ident, $op: tt) => {
        impl $trait<Vec2d> for Vec2d {
            fn $fn(&mut self, rhs: Vec2d){
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
	};
}

impl_op!(Add, add, +);
impl_op!(Mul, mul, *);
impl_op!(Sub, sub, -);
impl_op!(Div, div, /);
impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(MulAssign, mul_assign, *=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(DivAssign, div_assign, /=);

impl From<(usize, usize)> for Vec2d {
    fn from(from: (usize, usize)) -> Self {
        Self {
            x: from.0 as isize,
            y: from.1 as isize,
        }
    }
}

impl Into<(usize, usize)> for Vec2d {
    fn into(self) -> (usize, usize) {
        if self.x < 0
            || self.y < 0
            || self.x > crate::WIDTH as isize
            || self.y > crate::HEIGHT as isize
        {
            panic!("Cannot convert {self:?} to `(usize, usize)`")
        }
        (self.x as usize, self.y as usize)
    }
}
