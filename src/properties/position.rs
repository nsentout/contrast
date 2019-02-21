/// Structure representing a 3D position
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Position { // TODO: éviter de dupliquer la fonction as_array()
    /// Convert a position structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From <(f32, f32, f32)> for Position {
    fn from(p : (f32, f32, f32)) -> Position {
       Position {
           x : p.0, y : p.1, z : p.2
       }
    }
}