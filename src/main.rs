extern crate minifb;
mod cellular;
mod konstanter;
mod liquid;
mod math;
mod player;
use cellular::container;
use konstanter::{BAGGRUND, ROCK, WATER};
use minifb::{Key, Window, WindowOptions};
use player::Player;
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
        WindowOptions::default(),
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

        buffer.update();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
