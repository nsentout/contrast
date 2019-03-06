/// Structure representing a size
#[derive(PartialEq, Default, Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

impl Size {
    /// Convert a size structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn as_array(&self) -> &[f32; 2] {
        unsafe {
            std::mem::transmute::<&Size, &[f32; 2]>(self)
        }
    }
}

impl From <(f32, f32)> for Size {
    fn from(s : (f32, f32)) -> Size {
       Size {
           width : s.0, height : s.1
       }
    }
}
