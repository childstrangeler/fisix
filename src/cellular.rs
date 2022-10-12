use std::vec;

use crate::{
    konstanter::BAGGRUND,
    liquid,
    math::{ToVec, Vec2d},
    HEIGHT, WIDTH,
};

pub struct Container {
    cell_layer: Vec<u32>,
    draw_layer: Vec<u32>,
}

impl std::ops::Deref for Container {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.draw_layer[0..]
    }
}

impl<T: Into<(usize, usize)>> std::ops::Index<T> for Container {
    type Output = u32;
    fn index(&self, i: T) -> &u32 {
        let i = i.into();
        &self.cell_layer[i.0 + i.1 * WIDTH]
    }
}

impl<T: Into<(usize, usize)>> std::ops::IndexMut<T> for Container {
    fn index_mut(&mut self, i: T) -> &mut Self::Output {
        let i = i.into();
        &mut self.cell_layer[i.0 + i.1 * WIDTH]
    }
}

//her laves ren funktion der tegner et rektangel af en farve som f.eks. ROCK eller WATER, og af to hjørner (a, b)
impl Container {
    pub fn rect(&mut self, color: u32, a: Vec2d, b: Vec2d) {
        for x in a.x..b.x {
            for y in a.y..b.y {
                self[(x, y).vec()] = color;
            }
        }
    }

    pub fn circle(&mut self, color: u32, center: Vec2d, radius: usize) {
        for x in center.x as usize - radius..center.x as usize + radius {
            for y in center.y as usize - radius..center.y as usize + radius {
                if center.dist(&(x, y).into()) <= radius {
                    self[(x, y)] = color;
                }
            }
        }
    }

    pub fn texture(&mut self, texture: &dyn Fn(u32) -> u32) {
        for n in 0..WIDTH * HEIGHT {
            self.draw_layer[n] = texture(self.cell_layer[n])
        }
    }
}

pub fn container() -> Container {
    Container {
        cell_layer: vec![BAGGRUND; HEIGHT * WIDTH],
        draw_layer: vec![BAGGRUND; HEIGHT * WIDTH],
    }
}

impl Container {
    pub fn update(&mut self) {
        liquid::liquid(self)
    }
}
