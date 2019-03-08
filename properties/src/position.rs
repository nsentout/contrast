use std::ops::Add;
use std::ops::AddAssign;

/// Structure representing a 3D position
#[derive(PartialEq, Default, Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Position {
    /// Convert a position structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn as_array(&self) -> &[f32; 3] {
        unsafe {
            std::mem::transmute::<&Position, &[f32; 3]>(self)
        }
    }
}

impl From <(f32, f32, f32)> for Position {
    fn from(p : (f32, f32, f32)) -> Position {
       Position {
           x : p.0, y : p.1, z : p.2
       }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, pos: Position) {
        *self = Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
            z: self.z + pos.z
        };
    }
}