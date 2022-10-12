use minifb::Key;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub size: usize,
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

        /*
            if self.x + 1 != BAGGRUND as usize {
                self.x += 0
            } else if self.y + 1 != BAGGRUND as usize {
                self.y += 0
            }
        */
    }
}
