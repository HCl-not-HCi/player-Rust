use crate::models::progress::Progress;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct BoardAction {
    /// Unique ID of the source base
    pub src: u32,
    /// Unique ID of destination base
    pub dest: u32,
    /// Number of Bits to move
    pub amount: u32,
    /// UUID of this action
    pub uuid: Uuid,
    /// Unique ID of the player that this action belongs to
    pub player: u32,
    /// Current progress of this action
    pub progress: Progress,
}
