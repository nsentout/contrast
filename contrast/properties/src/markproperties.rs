use crate::markid::MarkId;
use crate::size::Size;
use crate::color::Color;

/// Structure representing the properties shared
/// by every type of marks, that is an id,
/// a size, a color and a rotation.
#[derive(Debug, Clone)]
pub struct MarkProperties {
    pub markid : MarkId,
    pub size : Size,
    pub color: Color,
    pub rotation : f32
}

impl MarkProperties {
    /// Simply returns a new instance of MarkProperties, initializing
    /// all attributes to their default values.
    pub fn new() -> Self
    {
        MarkProperties {
            markid : MarkId::new(),
            size : Size::default(),
            color: Color::default(),
            rotation : 0.0,
        }
    }
}