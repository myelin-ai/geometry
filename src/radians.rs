use serde::{Deserialize, Serialize};
use std::error::Error;
use std::f64::consts::{FRAC_PI_2, PI};
use std::fmt;

/// A radian confined to the range of [0.0; 2π)
#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Radians {
    value: f64,
}

impl Radians {
    /// Radians value representing a half turn (π)
    pub const HALF_TURN: Radians = Radians { value: PI };
    /// Radians value representing a quarter turn (π/2)
    pub const QUARTER_TURN: Radians = Radians { value: FRAC_PI_2 };

    /// Creates a new instance of [`Radians`].
    ///
    /// ### Errors
    /// Returns a [`RadiansError`] if the given value is outside the range [0.0; 2π)
    ///
    /// ### Examples
    /// ```
    /// use myelin_geometry::Radians;
    /// use std::f64::consts::FRAC_PI_3;
    ///
    /// let rotation = Radians::try_new(FRAC_PI_3).expect("Value was outside the range [0.0; 2π)");
    /// ```
    pub fn try_new(value: f64) -> Result<Self, RadiansError> {
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

    /// Convert degrees to radians
    ///
    /// ### Errors
    /// Returns a [`RadiansError`] if the given value is outside the range [0.0°; 360°)
    ///
    /// ### Examples
    /// ```
    /// use myelin_geometry::Radians;
    /// use std::f64::consts::FRAC_PI_2;
    /// use std::f64::consts::PI;
    ///
    /// use nearly_eq::assert_nearly_eq;
    ///
    /// assert_nearly_eq!(FRAC_PI_2, Radians::try_from_degrees(90.0).unwrap().value());
    /// ```
    pub fn try_from_degrees(degrees: f64) -> Result<Self, RadiansError> {
        const MAX_DEGREES: f64 = 360.0;
        const MAX_RADIANS: f64 = 2.0 * PI;

        Radians::try_new(degrees / MAX_DEGREES * MAX_RADIANS)
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
    fn try_from_degrees_works_with_0_as_input() {
        let degrees = 0.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = 0.0;
        assert_nearly_eq!(expected, radians.value())
    }

    #[test]
    fn try_from_degrees_returns_none_with_359_as_input() {
        let degrees = 359.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = (2.0 * PI) - (PI / 180.0);
        assert_nearly_eq!(expected, radians.value())
    }

    #[test]
    fn try_from_degrees_works_with_180_as_input() {
        let degrees = 180.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = PI;
        assert_nearly_eq!(expected, radians.value())
    }

    #[test]
    fn try_from_degrees_works_with_1_as_input() {
        let degrees = 1.0;
        let radians = Radians::try_from_degrees(degrees).unwrap();
        let expected = PI / 180.0;
        assert_nearly_eq!(expected, radians.value())
    }

    #[test]
    fn try_from_degrees_works_with_360_as_input() {
        let degrees = 360.0;
        let radians = Radians::try_from_degrees(degrees);
        assert!(radians.is_err());
    }

    #[test]
    fn try_from_degrees_returns_none_with_negative_1_as_input() {
        let degrees = -1.0;
        let radians = Radians::try_from_degrees(degrees);
        assert!(radians.is_err());
    }

    #[test]
    fn try_from_degrees_returns_none_with_361_as_input() {
        let degrees = 361.0;
        let radians = Radians::try_from_degrees(degrees);
        assert!(radians.is_err());
    }
}
