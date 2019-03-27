use contrast_properties::markid::MarkId;
use contrast_properties::size::Size;
use contrast_properties::color::Color;

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
    /// all attributes to their default values.
    pub(crate) fn new() -> Self
    {
        MarkProperties {
            markid : MarkId::new(),
            size : Size::default(),
            color: Color::default(),
            rotation : 0.0,
        }
    }
}