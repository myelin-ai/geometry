use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::f64::consts::PI;
use std::fmt;

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Creates a new instance of [`Radians`].
    ///
    /// ### Errors
    /// Returns a [`RadiansError`] if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_geometry::Radians;
    /// use std::f64::consts::PI;
    ///
    /// let rotation = Radians::try_new(PI).expect("Value was outside the range [0.0; 2π)");
    /// ```
    pub fn try_new(value: f64) -> Result<Radians, RadiansError> {
        if value >= 0.0 && value < 2.0 * PI {
            Ok(Radians { value })
        } else {
            Err(RadiansError::OutOfRange)
        }
    }

    /// Returns the underlying value
    pub fn value(self) -> f64 {
        self.value
    }

    /// Try getting the radians value of the degrees
    pub fn try_from_degrees(degrees: f64) -> Result<Radians, RadiansError> {
        return Radians::try_new(degrees / 360.0 * 2.0 * PI);
    }
}

/// The reason why a [`Radians`] instance could not be created
#[derive(Debug)]
pub enum RadiansError {
    /// The given value was not in the range [0.0; 2π)
    OutOfRange,
}

impl fmt::Display for RadiansError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Given value is not in range [0.0; 2π)")
    }
}

impl Error for RadiansError {}

#[cfg(test)]
mod tests {
    use super::*;
    use nearly_eq::assert_nearly_eq;
    use std::f64::consts::PI;

    #[test]
    fn radians_new_with_negative_0_point_1_is_none() {
        let radians = Radians::try_new(-0.1);
        assert!(radians.is_err())
    }

    #[test]
    fn radians_new_with_0_is_some() {
        let radians = Radians::try_new(0.0);
        assert!(radians.is_ok())
    }

    #[test]
    fn radians_new_with_1_point_9_pi_is_some() {
        let radians = Radians::try_new(1.9 * PI);
        assert!(radians.is_ok())
    }

    #[test]
    fn radians_new_with_2_pi_is_none() {
        let radians = Radians::try_new(2.0 * PI);
        assert!(radians.is_err())
    }

    #[test]
    fn radians_value_returns_1_when_given_1() {
        let value = 1.0;
        let radians = Radians::try_new(value).unwrap();
        assert_nearly_eq!(value, radians.value())
    }

    #[test]
    fn try_get_radians_works_with_0_as_input() {
        let degrees = 0.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = 0.0;
        assert_nearly_eq!(expected, radians.value())
    }
    #[test]
    fn try_get_radians_works_with_360_as_input() {
        let degrees = 360.0;
        let radians = Radians::try_from_degrees(degrees);
        match radians {
            Ok(_) => assert!(false, "360 degrees is too big. 2 PI as radiant is disallowed"),
            Err(_) => assert!(true),
        }
    }
    #[test]
    fn try_get_radians_returns_none_with_359_as_input() {
        let degrees = 359.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = 2.0 * PI - PI / 180.0;
        assert_nearly_eq!(expected, radians.value())
    }
    #[test]
    fn try_get_radians_returns_none_with_negative_1_as_input() {
        let degrees = -1.0;
        let radians = Radians::try_from_degrees(degrees);
        match radians {
            Ok(_) => assert!(false, "Negative one isn not an allowed degrees value"),
            Err(_) => assert!(true),
        }
    }
    #[test]
    fn try_get_radians_returns_none_with_361_as_input() {
        let degrees = 361.0;
        let radians = Radians::try_from_degrees(degrees);
        match radians {
            Ok(_) => assert!(
                false,
                "The maximum allowed degrees should be 360, which equals to 2 PI. 361 is too big."
            ),
            Err(_) => assert!(true),
        }
    }
    #[test]
    fn try_get_radians_works_with_180_as_input() {
        let degrees = 180.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = PI;
        assert_nearly_eq!(expected, radians.value())
    }
    #[test]
    fn try_get_radians_works_with_1_as_input() {
        let degrees = 1.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = PI / 180.0;
        assert_nearly_eq!(expected, radians.value())
    }
}
