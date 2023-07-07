//! Encoders

use strum::IntoStaticStr;

pub use astc::*;
pub use etc1s::*;
pub use uastc::*;

use crate::prelude::*;

mod astc;
mod etc1s;
mod uastc;

/// Texture encoding
#[derive(Debug, Clone, PartialEq, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(tag = "encoding"))]
pub enum Encoding {
    /// High-quality ASTC format
    ASTC(ASTCOptions),

    /// Supercompress the image data with ETC1S / BasisLZ.
    /// RED images will become RGB with RED in each component. RG images
    /// will have R in the RGB part and G in the alpha part of the
    /// compressed texture.
    ETC1S(ETC1SOptions),

    /// High-quality transcodable UASTC format.
    UASTC(UASTCOptions),
}

impl Encoding {
    /// Construct [`ASTC`](Encoding::ASTC) with default [options](ASTCOptions)
    pub const fn astc() -> Self {
        Self::ASTC(ASTCOptions::DEFAULT)
    }

    /// Construct [`ETC1S`](Encoding::ETC1S) with default [options](ETC1SOptions)
    pub const fn etc1s() -> Self {
        Self::ETC1S(ETC1SOptions::DEFAULT)
    }

    /// Construct [`UASTC`](Encoding::UASTC) with default [options](UASTCOptions)
    pub const fn uastc() -> Self {
        Self::UASTC(UASTCOptions::DEFAULT)
    }
}

impl Arg for Encoding {
    fn add_to(&self, name: &str, consumer: &mut impl ArgConsumer) -> bool {
        consumer.add_arg(name);
        consumer.add_arg(<&'static str>::from(self));
        self.add_unnamed_to(consumer)
    }

    fn add_unnamed_to(&self, consumer: &mut impl ArgConsumer) -> bool {
        match self {
            Encoding::ASTC(options) => {
                options.add_unnamed_to(consumer);
            }
            Encoding::ETC1S(options) => {
                options.add_unnamed_to(consumer);
            }
            Encoding::UASTC(options) => {
                options.add_unnamed_to(consumer);
            }
        };
        true
    }
}
