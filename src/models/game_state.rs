use crate::models::base::Base;
use crate::models::board_action::BoardAction;
use crate::models::game::Game;
use crate::models::game_config::GameConfig;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct GameState {
    /// List of all actions in progress
    pub actions: Vec<BoardAction>,
    /// list of all bases
    pub bases: Vec<Base>,
    /// Current game settings
    pub config: GameConfig,
    /// Current state of this game
    pub game: Game,
}
