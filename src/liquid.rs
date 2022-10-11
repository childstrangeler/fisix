/*
Check om pixel er water
Hvis den er water, Så se om den kan rykke sig ned (vand falder i fri luft)
ellers se om det kan flyde ud til siderne

Både andre vand pixels og solide pixels blokkere en given vand pixel i at rykke sig
*/

//her bliver vand lavet, men idéen er at have flere forskellige væsker på et senere tidspunkt, deraf navnet

use crate::{
    cellular::Container,
    konstanter::{BAGGRUND, WATER},
    HEIGHT, WIDTH,
};

pub fn liquid(container: &mut Container) {
    let mut has_move = std::collections::HashSet::new();
    for x in 1..WIDTH - 1 {
        for y in 1..HEIGHT - 1 {
            if container[(x, y)] == WATER && !has_move.contains(&(x, y)) {
                if container[(x, y + 1)] == BAGGRUND {
                    container[(x, y + 1)] = WATER;
                    container[(x, y)] = BAGGRUND;
                    has_move.insert((x, y + 1));
                } else if container[(x + 1, y)] == BAGGRUND {
                    container[(x + 1, y)] = WATER;
                    container[(x, y)] = BAGGRUND;
                    has_move.insert((x + 1, y));
                } else if container[(x - 1, y)] == BAGGRUND {
                    container[(x - 1, y)] = WATER;
                    container[(x, y)] = BAGGRUND;
                    has_move.insert((x - 1, y));
                }
            }
        }
    }
}
