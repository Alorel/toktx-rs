use std::fmt::{Display, Formatter, Write};

use crate::prelude::*;

/// 2D Vector delimited by `'x'`
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Arg)]
#[arg(to_string)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XY<T>(pub T, pub T);

/// 3D Vector delimited by `'x'`
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Arg)]
#[arg(to_string)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct XYZ<T>(pub T, pub T, pub T);

impl<T: Display> Display for XY<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)?;
        f.write_char('x')?;
        Display::fmt(&self.1, f)
    }
}

impl<T: Display> Display for XYZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)?;
        f.write_char('x')?;
        Display::fmt(&self.1, f)?;
        f.write_char('x')?;
        Display::fmt(&self.2, f)
    }
}

#[allow(missing_docs)]
impl<T: Copy> XY<T> {
    #[inline]
    pub fn x(self) -> T {
        self.0
    }

    #[inline]
    pub fn y(self) -> T {
        self.1
    }
}

#[allow(missing_docs)]
impl<T: Copy> XYZ<T> {
    #[inline]
    pub fn x(self) -> T {
        self.0
    }

    #[inline]
    pub fn y(self) -> T {
        self.1
    }

    #[inline]
    pub fn z(self) -> T {
        self.2
    }
}

impl<T> From<(T, T)> for XY<T> {
    #[inline]
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T> From<(T, T, T)> for XYZ<T> {
    #[inline]
    fn from((x, y, z): (T, T, T)) -> Self {
        Self(x, y, z)
    }
}

impl<T> From<[T; 2]> for XY<T> {
    #[inline]
    fn from([x, y]: [T; 2]) -> Self {
        Self(x, y)
    }
}

impl<T> From<[T; 3]> for XYZ<T> {
    #[inline]
    fn from([x, y, z]: [T; 3]) -> Self {
        Self(x, y, z)
    }
}
