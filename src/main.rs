extern crate minifb;
mod cellular;
mod konstanter;
mod liquid;
use cellular::container;
use konstanter::ROCK;
use liquid::WATER;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer = cellular::container();

    buffer.rect(WATER, (0, 0), (WIDTH, 75));

    buffer.rect(
        ROCK,
        (WIDTH / 2, HEIGHT / 2),
        (WIDTH / 2 + 100, HEIGHT / 2 + 100),
    );

    buffer.rect(
        ROCK,
        (WIDTH / 2 - 20, HEIGHT / 2 + 120),
        (WIDTH / 2 + 80, HEIGHT),
    );

    let mut window = Window::new(
        "Fisix - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        buffer.update();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
