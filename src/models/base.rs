use crate::models::position::Position;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Base {
    /// Position of the base
    pub position: Position,
    /// Unique ID of the base
    pub uid: u32,
    /// Name of the base
    pub name: String,
    /// Owner of the base
    pub player: u32,
    /// Current population of the base
    pub population: u32,
    /// Level of the base
    pub level: u32,
    /// Number of units required to upgrade
    pub units_until_upgrade: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::position::Position;

    #[test]
    fn default_base() {
        let default = Base::default();

        let base_null = Base {
            position: Position::default(),
            uid: 0,
            name: String::new(),
            player: 0,
            population: 0,
            level: 0,
            units_until_upgrade: 0,
        };

        assert_eq!(default, base_null);
    }
}
