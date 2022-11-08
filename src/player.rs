use minifb::Key;

pub struct Player {
    pub x: isize,
    pub y: isize,
    pub size: isize,
}

impl Player {
    pub fn match_key(&mut self, pressed: &Key) {
        match *pressed {
            Key::D => self.x += 4,
            Key::A => self.x -= 4,
            Key::W => self.y -= 4,
            Key::S => self.y += 4,

            _ => {}
        }
    }
}
