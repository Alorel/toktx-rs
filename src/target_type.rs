use std::fmt::{Debug, Display, Formatter};
use crate::prelude::*;

use strum::IntoStaticStr;

#[derive(Clone, Copy, Hash, Eq, PartialEq, IntoStaticStr, Arg)]
#[arg(to_string)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub enum TargetType {
    R,
    RG,
    RGB,
    RGBA,
}

impl Display for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("@c ")?;
        Debug::fmt(self, f)
    }
}

impl Debug for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&'static str>::from(*self))
    }
}

#[cfg(feature = "serde")]
serde_util!(serialise TargetType as static_str);
