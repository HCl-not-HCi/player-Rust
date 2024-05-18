use std::cmp;

use crate::models::{
    base::Base, base_level::BaseLevel, board_action::BoardAction, game_config::GameConfig, game_state::GameState, player_action::PlayerAction
};

const MIN_DEFENDERS: i32 = 5;

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
) -> (Vec<Base>, Vec<Base>, Vec<Base>) {
    let mut our_bases = Vec::new();
    let mut other_bases = Vec::new();
    let mut empty_bases = Vec::new();

    for base in bases {
        if base.player == our_player {
            our_bases.push(*base);
        } else if base.player == 0 {
            empty_bases.push(*base);
        } else {
            other_bases.push(*base);
        }
    }

    (our_bases, other_bases, empty_bases)
}

fn get_min_defenders(our_base: Base, enemy_bases: &[Base], remaining_players: u32) -> u32 {
    // let mut total_enemy_bits = 0;

    // for enemy_base in enemy_bases {
    //     total_enemy_bits += enemy_base.population;
    // }

    // return cmp::max((our_base.population as f64 * 0.3).floor() as u32, ((total_enemy_bits as f64 / remaining_players as f64) * 0.3).floor() as u32);
    5
}

fn survivors(src_base: &Base, dest_base: &Base, enemy_bases: &[Base], actions: &[BoardAction], remaining_players: u32, config: &GameConfig) -> i32 {
    let grace_period = config.paths.grace_period as i32;
    let death_rate = config.paths.death_rate as i32;

    let distance = get_base_distance(src_base, dest_base) as i32;
    let deaths = cmp::max((distance - grace_period) * death_rate, 0);

    let total_attacking_bits = get_total_attacking_bits(*src_base, &actions);

    let spawn_rate = get_base_level(&src_base, &config.base_levels).spawn_rate as i32;
    // if total_attacking_bits > src_base.population + spawn_rate * 5 {
    //     // Our src_base is probably lost, go all in
    //     return src_base.population - deaths;
    // } else {
    //     // We can still defend our src_base, so keep some defenders at src_base
    //     let defenders = cmp::max(get_min_defenders(*src_base, enemy_bases, remaining_players), total_attacking_bits);
    //     return cmp::max(src_base.population - defenders, 0) - deaths;
    // }
    return cmp::max(src_base.population as i32 - MIN_DEFENDERS, 0) - deaths;

    
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

fn best_target_base<'a>(
    other_bases: &'a [Base],
    our_bases: &'a [Base],
    actions: &[BoardAction],
    remaining_players: u32,
    config: &'a GameConfig,
) -> Option<Base> {
    let mut best_target_base = None;
    let mut most_survivors = 0;

    for target_base in other_bases {
        let mut sum_survivors = 0;

        for selected_base in our_bases {
            sum_survivors += survivors(
                selected_base,
                target_base,
                other_bases,
                actions,
                remaining_players,
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

/// Returns total bits currently on the way to attack the given base
fn get_total_attacking_bits(base: Base, actions: &[BoardAction]) -> i32 {
    let mut total_attack_bits = 0;

    for action in actions {
        if action.dest == base.uid {
            total_attack_bits += action.amount as i32;
        }
    }

    return total_attack_bits;
}

fn attack(
    other_bases: &[Base],
    our_bases: &[Base],
    actions: &[BoardAction],
    remaining_players: u32,
    config: &GameConfig,
) -> Vec<PlayerAction> {
    let mut attack_list = Vec::new();

    if let Some(target_base) = best_target_base(other_bases, our_bases, &actions, remaining_players, config) {
        for src_base in our_bases {
            let survivors = survivors(src_base, &target_base, other_bases, actions, remaining_players, config);
            if survivors > 0 {
                attack_list.push(
                    PlayerAction {
                        src: src_base.uid,
                        dest: target_base.uid,
                        amount: survivors as u32
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
    attack(&[other_bases, empty_bases].concat(), &our_bases, &game_state.actions, game_state.game.remaining_players, &config)

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
