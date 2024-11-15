use serde::Serialize;

#[derive(Serialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PlayerAction {
    /// Uid of source base
    pub src: u32,
    /// Uid of destination base
    pub dest: u32,
    /// Number of bits moved
    pub amount: u32,
}
