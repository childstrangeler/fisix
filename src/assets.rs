use lazy_static::lazy_static;
use image::{self, imageops};
use crate::{WIDTH, HEIGHT};

lazy_static! {
    pub static ref WATER_IMAGE: Vec<u32> = {
        let mut img = image::open("tests/images/jpg/progressive/cat.jpg").unwrap();
        let subimg = imageops::crop(&mut img, 0, 0, WIDTH as u32, HEIGHT as u32).to_image();
        let mut buffer = vec![0; WIDTH*HEIGHT];

        for (x, y, pixel) in subimg.enumerate_pixels(){
            let v = pixel.0;
            buffer[x as usize + y as usize * WIDTH] = ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32
        }

        buffer
    };
}