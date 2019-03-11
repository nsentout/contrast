/// Structure representing a RGBA color
#[derive(PartialEq, Default, Copy, Clone, Debug)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Color {
    /// Convert a color structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn to_array(&self) -> &[f32; 4] {
        unsafe {
            std::mem::transmute::<&Color, &[f32; 4]>(self)
        }
    }
}

impl From <(f32, f32, f32, f32)> for Color {
    fn from(c : (f32, f32, f32, f32)) -> Color {
       Color {
           r : c.0, g : c.1, b : c.2, a : c.3
       }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_array()
    {
        let c1 = Color { r : 1.0, g : 0.5, b : 0.7, a : 0.0 };
        let c2 = Color { r : -10.0, g : -15.5, b : -10.0, a : -7.5};

        assert_eq!(&[1.0, 0.5, 0.7, 0.0], c1.to_array());
        assert_eq!(&[-10.0, -15.5, -10.0, -7.5], c2.to_array());
    }
}