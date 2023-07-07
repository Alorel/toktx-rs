//! Basic programmatic interface for the `toktx` utility

#![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

use std::env;
use std::path::PathBuf;
use std::sync::atomic;
use std::sync::atomic::AtomicU32;
pub use const_default::ConstDefault;
use strum::{EnumString, IntoStaticStr};
use uuid::Uuid;

pub use conv::{InputSource, ToKtxResult};
pub use conv::ToKtxError;
pub use filter::Filter;
use prelude::*;
pub use swizzle::{Swizzle, SwizzleChar};
pub use target_type::TargetType;
pub use toktx::ToKtx;
pub use xy::{XY, XYZ};

macro_rules! arg_static_str {
    ($($for: ty)+) => {
        $(
            impl Arg for $for {
                fn add_unnamed_to(&self, consumer: &mut impl ArgConsumer) -> bool {
                    consumer.add_arg(<&'static str>::from(*self));
                    true
                }
            }
        )+
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "serde")] {
        #[inline]
        const fn is_false(value: &bool) -> bool {
            !*value
        }

        fn is_default(value: &(impl Default + PartialEq)) -> bool {
            *value == Default::default()
        }

        macro_rules! serde_util {
            (serialise $ty: ty as static_str) => {
                impl Serialize for $ty  {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                        <&'static str>::from(*self).serialize(serializer)
                    }
                }
            };
            (serialise $ty: ty as to_string) => {
                impl Serialize for $ty  {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                        self.to_string().serialize(serializer)
                    }
                }
            };
            (deserialise $ty: ty as parse with $msg: literal) => {
                impl<'de> Deserialize<'de> for $ty {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                      where D: Deserializer<'de>
                    {
                        struct DerivedVisitor;
                        impl<'de> Visitor<'de> for DerivedVisitor {
                            type Value = $ty;

                            #[inline]
                            fn expecting(&self, formatter: &mut Formatter) -> ::std::fmt::Result {
                                formatter.write_str($msg)
                            }

                            #[inline]
                            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
                                v.parse().map_err(move |_| E::invalid_value(::serde::de::Unexpected::Str(v), &self))
                            }
                        }

                        deserializer.deserialize_str(DerivedVisitor)
                    }
                }
            };
        }
    }
}

mod conv;
pub mod enc;
mod filter;
mod swizzle;
mod target_type;
mod toktx;
mod xy;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
#[allow(missing_docs)]
pub enum Primaries {
    Bt709,
    Srgb,
    None,
}

/// How to sample pixels near the image boundaries
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumString, IntoStaticStr)]
#[allow(missing_docs)]
#[strum(serialize_all = "lowercase")]
pub enum WMode {
    Wrap,
    Reflect,
    Clamp,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumString, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
#[allow(missing_docs)]
pub enum TransferFunction {
    Linear,
    Srgb,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, EnumString, IntoStaticStr, Arg)]
#[strum(serialize_all = "lowercase")]
#[arg(drop_name)]
#[allow(missing_docs)]
pub enum OutputFormat {
    KTX,

    #[arg(value = "--t2")]
    KTX2,
}

impl ConstDefault for OutputFormat {
    const DEFAULT: Self = Self::KTX2;
}
impl Default for OutputFormat {
    fn default() -> Self {
        Self::DEFAULT
    }
}

arg_static_str!(TransferFunction WMode Primaries);

fn temp_path() -> PathBuf {
    let mut path = env::temp_dir();

    const COUNTER: AtomicU32 = AtomicU32::new(0);

    path.push(format!("toktx-rs-{}-{}.ktx2", COUNTER.fetch_add(1, atomic::Ordering::SeqCst), Uuid::new_v4()));
    path
}

cfg_if! {
    if #[cfg(feature = "serde")] {
        serde_util!(serialise OutputFormat as static_str);
        serde_util!(deserialise OutputFormat as parse with "OutputFormat");

        serde_util!(serialise TransferFunction as static_str);
        serde_util!(deserialise TransferFunction as parse with "TransferFunction");

        serde_util!(serialise WMode as static_str);
        serde_util!(deserialise WMode as parse with "WMode");

        serde_util!(serialise Primaries as static_str);
        serde_util!(deserialise Primaries as parse with "Primaries");
    }
}

mod prelude {
    pub(crate) use argley::prelude::*;
    pub(crate) use cfg_if::cfg_if;
    pub(crate) use crate::ConstDefault;

    #[cfg(feature = "serde")]
    pub(crate) use {
        serde::{
            de::{Error, Visitor},
            Deserialize, Deserializer, Serialize, Serializer,
        },
        std::fmt::Formatter,
    };
}
