use properties::markid::MarkId;
use properties::size::Size;
use properties::color::Color;

/// Structure representing the properties shared
/// by every type of marks, that is an id,
/// a size, a color and a rotation.
#[derive(Debug, Clone)]
pub(crate) struct MarkProperties {
    pub markid : MarkId,
    pub size : Size,
    pub color: Color,
    pub rotation : f32
}

impl MarkProperties {
    /// Simply returns a new instance of MarkProperties, initializing
    /// all attributes to their default values, except the id.
    pub(crate) fn new(id : usize) -> Self
    {
        MarkProperties {
            markid : MarkId::new(id),
            size : Size::default(),
            color: Color::default(),
            rotation : 0.0,
        }
    }
}