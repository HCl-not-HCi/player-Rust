use crate::models::base_level::BaseLevel;
use crate::models::path_config::PathConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct GameConfig {
    /// Available base level in this game
    pub base_levels: Vec<BaseLevel>,
    /// Contains the travel difficulty (path) between bases
    pub paths: PathConfig,
}
