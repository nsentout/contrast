use super::size::Size;
use super::color::Color;

/// Structure representing the properties shared
/// by every type of marks, that is an id,
/// a size, a color and a rotation.
#[derive(Debug)]
pub struct MarkProperties {
    pub id : usize,
    pub size : Size,
    pub color: Color,
    pub rotation : f32,
}

impl MarkProperties {
    /// Simply returns a new instance of MarkProperties, initializing
    /// all attributes to their default values, except the id.
    pub fn new(id : usize) -> Self
    {
        MarkProperties {
            id,
            size : Size { width : 0.0, height : 0.0},
            color: Color { r : 0.0, g : 0.0, b : 0.0, a : 0.0 },
            rotation : 0.0,
        }
    }
}