use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BaseLevel {
    /// Number of maximum inhabitants (Bits) in the base
    pub max_population: u32,
    /// Bits required to upgrade to the next level
    pub upgrade_cost: u32,
    /// How many Bits spawn every tick
    pub spawn_rate: u32,
}
