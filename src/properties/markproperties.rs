use super::size::Size;
use super::color::Color;

/// Structure representing the properties shared
/// by every type of marks, that is an id,
/// a size, a color and a rotation.
#[derive(Debug)]
pub(crate) struct MarkProperties {
    pub id : usize,
    pub size : Size,
    pub color: Color,
    pub rotation : f32,
}

impl MarkProperties {
    /// Simply returns a new instance of MarkProperties, initializing
    /// all attributes to their default values, except the id.
    pub(crate) fn new(id : usize) -> Self
    {
        MarkProperties {
            id,
            size : Size::default(),
            color: Color::default(),
            rotation : 0.0,
        }
    }
}