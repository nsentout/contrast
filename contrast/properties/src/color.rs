/// Structure representing a color understandable by the shaders.
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

    /// Simply convert an rgba color to a color understandable by the shaders.
    pub fn from_rgba(r : f32, g : f32, b : f32, a : f32) -> Color {
        Color { r : r / 255.0, g : g / 255.0, b : b / 255.0, a : a / 255.0 }
    }

    pub fn black() -> Color {
        Color { r : 0.0, g : 0.0, b : 0.0, a : 1.0 }
    }

    pub fn white() -> Color {
        Color { r : 1.0, g : 1.0, b : 1.0, a : 1.0 }
    }

    pub fn yellow() -> Color {
        Color { r : 1.0, g : 1.0, b : 0.0, a : 1.0 }
    }

    pub fn pink() -> Color {
        Color { r : 1.0, g : 0.0, b : 1.0, a : 1.0 }
    }

    pub fn cyan() -> Color {
        Color { r : 0.0, g : 1.0, b : 1.0, a : 1.0 }
    }
    
    pub fn grey() -> Color {
        Color { r : 0.5, g : 0.5, b : 0.5, a : 1.0 }
    }

    pub fn red() -> Color {
        Color { r : 1.0, g : 0.0, b : 0.0, a : 1.0 }
    }

    pub fn green() -> Color {
        Color { r : 0.0, g : 1.0, b : 0.0, a : 1.0 }
    }

    pub fn blue() -> Color {
        Color { r : 0.0, g : 0.0, b : 1.0, a : 1.0 }
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

    // Macro copied from the crate 'assert_approx_eq'
    // (https://github.com/ashleygwilliams/assert_approx_eq)
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            let eps = 1.0e-5;
            let (a, b) = (&$a, &$b);
            assert!(
                (*a - *b).abs() < eps,
                "assertion failed: `(left !== right)` \
                (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                *a,
                *b,
                eps,
                (*a - *b).abs()
            );
        }
    }

    #[test]
    fn to_array()
    {
        let c1 = Color { r : 1.0, g : 0.5, b : 0.7, a : 0.0 };
        let c2 = Color { r : -10.0, g : -15.5, b : -10.0, a : -7.5};

        assert_eq!(&[1.0, 0.5, 0.7, 0.0], c1.to_array());
        assert_eq!(&[-10.0, -15.5, -10.0, -7.5], c2.to_array());
    }

    #[test]
    fn from_rgba()
    {
        let c1 = Color::from_rgba(255.0, 0.0, 0.0, 255.0);
        let c2 = Color::from_rgba(127.5, 127.5, 127.5, 127.5);
        let c3 = Color::from_rgba(40.157, 100.578, 200.546, 0.246);

        assert_eq!(c1, Color { r : 1.0, g : 0.0, b : 0.0, a : 1.0});
        assert_eq!(c2, Color { r : 0.5, g : 0.5, b : 0.5, a : 0.5});
        assert_approx_eq!(c3.r, 0.157478);
        assert_approx_eq!(c3.g, 0.394423);
        assert_approx_eq!(c3.b, 0.786455);
        assert_approx_eq!(c3.a, 0.000965);
    }
    
}