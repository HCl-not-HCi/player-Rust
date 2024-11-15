use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Progress {
    /// The distance between the source and destination base
    pub distance: u32,
    /// The progress of the Bits and how far they already travelled
    pub traveled: u32,
}
