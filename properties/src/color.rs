/// Structure representing a RGBA color
#[derive(Default, Copy, Clone, Debug)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color {
    /// Convert a color structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn as_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From <(f32, f32, f32, f32)> for Color {
    fn from(c : (f32, f32, f32, f32)) -> Color {
       Color {
           r : c.0, g : c.1, b : c.2, a : c.3
       }
    }
}
