/*
Check om pixel er water
Hvis den er water, Så se om den kan rykke sig ned (vand falder i fri luft)
ellers se om det kan flyde ud til siderne

Både andre vand pixels og solide pixels blokkere en given vand pixel i at rykke sig
*/

use crate::{cellular::Container, HEIGHT, WIDTH};

pub const WATER: u32 = 0x005050ff;

pub fn liquid(container: &mut Container) {
    for x in 0..WIDTH - 1 {
        for y in 0..HEIGHT - 1 {
            if container[(x, y)] == WATER {
                if container[(x, y + 1)] == crate::konstanter::BAGGRUND {
                    container[(x, y + 1)] = WATER;
                    container[(x, y)] = crate::konstanter::BAGGRUND;
                }
            }
        }
    }
}
