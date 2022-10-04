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

pub fn container() -> Container {
    Container(vec![BAGGRUND; HEIGHT * WIDTH])
}

impl Container {
    pub fn update(&mut self) {
        liquid::liquid(self)
    }
}
