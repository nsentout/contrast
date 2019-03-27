/// Structure representing a size
#[derive(PartialEq, Default, Copy, Clone, Debug)]
pub struct Size {
    pub width : f32,
    pub height : f32
}

impl Size {
    /// Convert a size structure to an array.
    /// Useful when converting our marks to vertices.
    pub fn to_array(&self) -> &[f32; 2] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_array()
    {
        let s1 = Size { width : 10.0, height : 15.5 };
        let s2 = Size { width : -10.0, height : -15.5 };

        assert_eq!(&[10.0, 15.5], s1.to_array());
        assert_eq!(&[-10.0, -15.5], s2.to_array());
    }
}
