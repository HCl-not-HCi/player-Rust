use std::cmp;

use crate::models::{
    game_state::GameState,
    player_action::PlayerAction,
    base::Base,
    base_level::BaseLevel,
    game_config::GameConfig
};

const MIN_DEFENDERS: u32 = 5;

fn euclid(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> u32 {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    let dz = (z1 - z2).abs();
    ((dx * dx + dy * dy + dz * dz) as f32).sqrt() as u32
}

fn get_base_distance(base_1: &Base, base_2: &Base) -> u32 {
    euclid(
        base_1.position.x,
        base_1.position.y,
        base_1.position.z,
        base_2.position.x,
        base_2.position.y,
        base_2.position.z,
    )
}

fn get_base_level<'a>(base: &Base, base_levels: &'a [BaseLevel]) -> &'a BaseLevel {
    &base_levels[base.level as usize]
}

fn filter_bases(
    bases: &[Base],
    our_player: u32,
) -> (Vec<&Base>, Vec<&Base>, Vec<&Base>) {
    let mut our_bases = Vec::new();
    let mut other_bases = Vec::new();
    let mut empty_bases = Vec::new();

    for base in bases {
        if base.player == our_player {
            our_bases.push(base);
        } else if base.player == 0 {
            empty_bases.push(base);
        } else {
            other_bases.push(base);
        }
    }

    (our_bases, other_bases, empty_bases)
}

fn survivors(src_base: &Base, dest_base: &Base, config: &GameConfig) -> u32 {
    let grace_period = config.paths.grace_period;
    let death_rate = config.paths.death_rate;
    let distance = get_base_distance(src_base, dest_base);
    let deaths = cmp::max((distance - grace_period) as u32 * death_rate, 0) as u32;
    cmp::max(src_base.population - MIN_DEFENDERS, 0) - deaths
}

fn defenders_at_time(dest_time: u32, dest_base: &Base, config: &GameConfig) -> u32 {
    let spawn_rate = get_base_level(dest_base, &config.base_levels).spawn_rate;
    dest_base.population + dest_time * spawn_rate
}

fn attack_decision(
    attackers: u32,
    dest_base: &Base,
    config: &GameConfig,
    dest_time: u32,
) -> i32 {
    (attackers as i32) - (defenders_at_time(dest_time, dest_base, config) as i32)
}

fn population_average(bases: &[&Base]) -> u32 {
    let total_population: u32 = bases.iter().map(|base| base.population).sum();
    total_population / bases.len() as u32
}

fn iterate_bases<'a>(
    other_bases: &'a [&'a Base],
    our_bases: &'a [&'a Base],
    config: &'a GameConfig,
) -> Option<&'a Base> {
    let mut best_target_base = None;
    let mut most_survivors = 0;

    for target_base in other_bases {
        let mut sum_survivors = 0;
        for selected_base in our_bases {
            sum_survivors += survivors(
                selected_base,
                target_base,
                config,
            );
        }
        if sum_survivors > most_survivors {
            most_survivors = sum_survivors;
            best_target_base = Some(*target_base);
        }
    }

    best_target_base
}

fn attack(
    other_bases: &[&Base],
    our_bases: &[&Base],
    config: &GameConfig,
) -> Vec<PlayerAction> {
    let mut attack_list = Vec::new();
    if let Some(target_base) = iterate_bases(other_bases, our_bases, config) {
        for src_base in our_bases {
            let survivors = survivors(src_base, target_base, config);
            if survivors > 0 {
                attack_list.push(
                    PlayerAction {
                        src: src_base.uid,
                        dest: target_base.uid,
                        amount: survivors
                    }
                )
            }
            }
        }
    
    attack_list
}


pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    // TODO: place your player logic here.

    let our_player = game_state.game.player;
    let bases = game_state.bases;
    let config = game_state.config;
    let (our_bases, other_bases, empty_bases) = filter_bases(&bases, our_player);
    // our_bases.extend(empty_bases.iter());

    // attack(&other_bases, &our_bases, &config)
    attack(&[other_bases, empty_bases].concat(), &our_bases, &config)

    // vec![PlayerAction {
    //     src: 0,
    //     dest: 0,
    //     amount: 0,
    // }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }
}
