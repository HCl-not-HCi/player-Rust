use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Progress {
    pub distance: u32, // distance between the bases
    pub traveled: u32, // distance already traveled
}
