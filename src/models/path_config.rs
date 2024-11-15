use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PathConfig {
    /// During the grace period your Bits travel without taking damage, meaning no bits die.
    pub grace_period: u32,
    /// After the grace period, every tick `death_rate` Bits die.
    /// You could say that they starve :)
    pub death_rate: u32,
}
