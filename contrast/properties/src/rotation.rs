/// Empty structure whose only purpose is to provide a conversion
/// function from degrees to radian.
pub struct Rotation;

impl Rotation {
    /// Simply convert degrees to radian.
    pub fn from_degrees(degrees : f32) -> f32 {
        0.0174533 * degrees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PI : f32 = 3.141592653589793238462643383279;

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
    fn from_degrees()
    {
        assert_approx_eq!(Rotation::from_degrees(57.2958), 1.0);
        assert_approx_eq!(Rotation::from_degrees(90.0), PI / 2.0);
        assert_approx_eq!(Rotation::from_degrees(180.0), PI);
        assert_approx_eq!(Rotation::from_degrees(225.0), 3.92699);
        assert_approx_eq!(Rotation::from_degrees(360.0), PI * 2.0);
    }
}