use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// A point on a 2-D plane.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl crate::TwoDim for Point {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl Point {
    /// Provide two coordinates on the Cartesian coordinate system to form a
    /// point. Inputs can be integers or floating point numbers.
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Eq for Point {}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .trim_start_matches(|c| c == '(')
            .trim_end_matches(|c| c == ')')
            .split(',')
            .collect();
        if coords.len() != 2 {
            return Err(ParsePointError::CommaError);
        }
        let x = coords[0].trim().parse()?;
        let y = coords[1].trim().parse()?;
        Ok(Point { x, y })
    }
}

impl<T, U> From<(T, U)> for Point
where
    T: Into<f64>,
    U: Into<f64>,
{
    fn from((x, y): (T, U)) -> Self {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.x.fmt(f)?;
        write!(f, ", ")?;
        self.y.fmt(f)?;
        write!(f, ")")
    }
}

#[derive(Debug)]
pub enum ParsePointError {
    FloatError(ParseFloatError),
    CommaError,
}

impl Error for ParsePointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FloatError(e) => Some(e),
            Self::CommaError => None,
        }
    }
}

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FloatError(e) => e.fmt(f),
            Self::CommaError => write!(f, "CommaError"),
        }
    }
}

impl From<ParseFloatError> for ParsePointError {
    fn from(error: ParseFloatError) -> Self {
        ParsePointError::FloatError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() -> Result<(), ParsePointError> {
        macro_rules! assert_point {
            ($str:expr, $rslt:expr) => {
                let p: Point = $str.parse()?;
                assert_eq!(p, $rslt.into());
            };
        }

        assert_point!("(1.1,2.2)", (1.1, 2.2));
        assert_point!("1 , 3 ", (1, 3));
        assert_point!("1 ,2.)", (1, 2));
        assert_point!("( 1., 2.3", (1, 2.3));
        assert_point!("234, 567", (234, 567));

        Ok(())
    }

    #[test]
    fn from_str_error() {
        macro_rules! assert_err_point {
            ($str:expr) => {
                assert!($str.parse::<Point>().is_err());
            };
        }

        assert_err_point!("12");
        assert_err_point!("1.2, 2.3, 3.4, 5.6");
        assert_err_point!("(1.2, 2.3, 3.4)");
        assert_err_point!(")2, 4");
        assert_err_point!("blah, blah");
    }
}
