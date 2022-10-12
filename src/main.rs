extern crate minifb;
mod cellular;
mod konstanter;
mod liquid;
mod math;
mod player;
mod assets;
use std::clone;

use cellular::container;
use konstanter::{ROCK, WATER};
use minifb::{Key, Scale, Window, WindowOptions};
use player::Player;
use rand::random;
//use player::match_key;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

/*
Forsøger at lave en simpel pixelbaseret physics engine, måske med en idé om at tage det videre
til en form for Noita inspireret spil.
*/

fn main() {
    let mut buffer = container();

    //her fyldes de øverste 75 pixels over hele skærmen med vand
    buffer.rect(WATER, (0, 0).into(), (WIDTH, 75).into());
    //her sættes en flyvende sten på skærmen
    buffer.rect(
        ROCK,
        (WIDTH / 2, HEIGHT / 2).into(),
        (WIDTH / 2 + 100, HEIGHT / 2 + 100).into(),
    );
    //her sættes en sten under den flyvende sten
    buffer.rect(
        ROCK,
        (WIDTH / 2 - 20, HEIGHT / 2 + 120).into(),
        (WIDTH / 2 + 80, HEIGHT).into(),
    );
    //her fyldes de nederste ti pixels hen ad hele skærmen med sten
    buffer.rect(ROCK, (0, HEIGHT - 10).into(), (WIDTH, HEIGHT).into());

    //vores stenkant (Aksels idé)
    buffer.rect(ROCK, (0, 0).into(), (2, HEIGHT).into());
    buffer.rect(ROCK, (0, 0).into(), (WIDTH, 2).into());
    buffer.rect(ROCK, (WIDTH - 2, 0).into(), (WIDTH, HEIGHT).into());

    //tegner en cirkel udfra
    buffer.circle(ROCK, (60, 280).into(), 50);
    let mut player = Player {
        x: 70,
        y: 80,
        size: 16,
    };

    let mut window = Window::new(
        "Fisix - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::FitScreen,
            ..Default::default()
        },
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        buffer.rect(
            crate::konstanter::PLAYER,
            (player.x, player.y).into(),
            (player.x + player.size, player.y + player.size + 10).into(),
        );
        for pressed in window.get_keys().iter() {
            player.match_key(pressed);
        }
        let vandfarve = [
            0x003facff, 0x0037a9ff, 0x002ea5ff, 0x0026a2ff, 0x001e9eff, 0x00169bff, 0x000e97ff,
            0x000694ff, 0x000090fc, 0x00008bf4, 0x000087ec,
        ];
        buffer.texture(&|pixel: u32| {
            if pixel == WATER {
                vandfarve[rand::random::<usize>() % vandfarve.len()]
            } else {
                pixel
            }
        });

        buffer.update();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
