use std::fmt::{Display, Write};
use std::str::{FromStr};

use crate::prelude::*;
use delegate_display::{DelegateDebug, DelegateDisplay};
use derive_more::{Deref, DerefMut, From, Into};

const ELEM_COUNT: usize = 4;

/// An alhpanumeric sequence matching the regular expression `^[rgba01]{4}$`
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Into, From, Deref, DerefMut, Arg)]
#[arg(to_string)]
pub struct Swizzle(pub [SwizzleChar; ELEM_COUNT]);

impl Display for Swizzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c, d] = **self;

        f.write_char(*a)?;
        f.write_char(*b)?;
        f.write_char(*c)?;
        f.write_char(*d)
    }
}

impl FromStr for Swizzle {
    type Err = Option<usize>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ELEM_COUNT {
            return Err(None);
        }

        let mut inner = [SwizzleChar::ZERO; ELEM_COUNT];

        for (idx, char) in s.char_indices() {
            inner[idx] = match SwizzleChar::try_from(char) {
                Ok(ch) => ch,
                Err(_) => return Err(Some(idx)),
            };
        }

        Ok(Self(inner))
    }
}

/// A character that can appear in [`Swizzle`].
#[derive(Clone, Copy, Hash, Eq, PartialEq, DelegateDisplay, DelegateDebug, Into, Deref)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(try_from = "char", into = "char"))]
pub struct SwizzleChar(char);

#[allow(missing_docs)]
impl SwizzleChar {
    pub const R: Self = Self('r');
    pub const G: Self = Self('g');
    pub const B: Self = Self('b');
    pub const A: Self = Self('a');
    pub const ZERO: Self = Self('0');
    pub const ONE: Self = Self('1');
}

impl TryFrom<char> for SwizzleChar {
    type Error = char;

fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'r' | 'g' | 'b' | 'a' | '0' | '1' => Ok(Self(value)),
            _ => Err(value),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "serde")] {
        serde_util!(serialise Swizzle as to_string);
        serde_util!(deserialise Swizzle as parse with "Swizzle");
    }
}
