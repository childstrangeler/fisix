use crate::{HEIGHT, WIDTH};

pub struct Container([[u32;HEIGHT];WIDTH]);

impl std::ops::Deref for Container{
    type Target = [u32];
    fn deref(&self) -> &Self::Target{
        unsafe {std::slice::from_raw_parts(std::mem::transmute(self), HEIGHT*WIDTH)}
    }
}

pub fn container() -> Container{
   Container([[0; HEIGHT];WIDTH])
}