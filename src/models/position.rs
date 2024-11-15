use serde::Deserialize;

/// The position of a base in the 3-dimensional space
#[derive(Deserialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_position() {
        let default = Position::default();

        let position_null = Position { x: 0, y: 0, z: 0 };

        assert_eq!(default, position_null);
    }
}
