use std::fmt::{Display, Formatter};

use strum::{EnumString, IntoStaticStr};

use crate::prelude::*;

/// Filter to use when generating mipmaps.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
pub enum Filter {
    Box,
    Tent,
    Bell,

    #[strum(serialize = "b-spline")]
    BSpline,
    Mitchell,
    Lanczos3,
    Lanczos4,
    Lanczos6,
    Lanczos12,
    Blackman,
    Kaiser,
    Gaussian,
    Catmullrom,
    QuadraticInterp,
    QuadraticApprox,
    QuadraticMix,
}

arg_static_str!(Filter);

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(<&'static str>::from(*self))
    }
}

cfg_if! {
    if #[cfg(feature = "serde")] {
        serde_util!(serialise Filter as static_str);
        serde_util!(deserialise Filter as parse with "Filter");
    }
}
