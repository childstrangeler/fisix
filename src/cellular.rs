use crate::{konstanter::BAGGRUND, liquid, HEIGHT, WIDTH};

pub struct Container(Vec<u32>);

impl std::ops::Deref for Container {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        &self.0[0..]
    }
}

impl std::ops::Index<(usize, usize)> for Container {
    type Output = u32;
    fn index(&self, i: (usize, usize)) -> &u32 {
        &self.0[i.0 + i.1 * WIDTH]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Container {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.0[i.0 + i.1 * WIDTH]
    }
}

//her laves ren funktion der tegner et rektangel af en farve som f.eks. ROCK eller WATER, og af to hjørner (a, b)
impl Container {
    pub fn rect(&mut self, color: u32, a: (usize, usize), b: (usize, usize)) {
        for x in a.0..b.0 {
            for y in a.1..b.1 {
                self[(x, y)] = color;
            }
        }
    }
}

//forsøgte at lave en trekant... virker ikke
/*
impl Container {
    pub fn tri(&mut self, color: u32, a: (usize, usize), b: (usize, usize), c: (usize, usize)) {
        for _x in a.0..b.0 {
            for x in b.0..c.0 {
                for _y in a.1..b.1 {
                    for y in b.1..c.1 {
                        self[(x, y)] = color;
                    }
                }
            }
        }
    }
}
*/
pub fn container() -> Container {
    Container(vec![BAGGRUND; HEIGHT * WIDTH])
}

impl Container {
    pub fn update(&mut self) {
        liquid::liquid(self)
    }
}
