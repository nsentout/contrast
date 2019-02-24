/// Structure representing a size
#[derive(Default, Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

impl Size {
    /// Convert a size structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn as_array(self) -> [f32; 2] {
        [self.width, self.height]
    }
}
