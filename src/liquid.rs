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
    math::ToVec,
    HEIGHT, WIDTH,
};

pub fn liquid(container: &mut Container) {
    let mut has_move = std::collections::HashSet::new();
    const FLOW_DIRECTIONS: &[(isize, isize)] = &[(0, 1), (1, 0), (1, 1)];
    for y in 1..HEIGHT - 1 {
        for mut x in 1..WIDTH - 1 {
            if rand::random() {
                x = WIDTH - 1 - x;
            }
            if container[(x, y)] == WATER && !has_move.contains(&(x, y).into()) {
                for mut flow in FLOW_DIRECTIONS.iter().copied().map(|n| n.vec()) {
                    if rand::random() {
                        flow *= (-1isize, 1).vec()
                    }
                    let mut dest = container[(x, y).vec() + flow];
                    if dest != BAGGRUND && dest != crate::konstanter::PLAYER {
                        flow *= (-1isize, 1).vec();
                        dest = container[(x, y).vec() + flow];
                        if dest != BAGGRUND && dest != crate::konstanter::PLAYER {
                            continue;
                        }
                    }

                    container[(x, y).vec() + flow] = WATER;
                    container[(x, y)] = BAGGRUND;
                    has_move.insert((x, y).vec() + flow);
                    break;
                }
            }
        }
    }
}
