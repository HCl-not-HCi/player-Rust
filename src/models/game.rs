use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Game {
    /// UID of this game
    pub uid: u32,
    /// Current tick in this game
    pub tick: u32,
    /// Number of total players in this game
    pub player_count: u32,
    /// Number of remaining players in this game.
    /// This number will decrease as players lose all their bases.
    pub remaining_players: u32,
    /// UID of your player
    pub player: u32,
}
