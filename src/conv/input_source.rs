use std::fs;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::temp_path;
use crate::conv::error::ToKtxError;
use crate::prelude::*;

use super::ToKtxResult;

/// KTX conversion input
pub trait InputSource {
    /// Mirrors [`Arg::add_unnamed_to`]
    fn add_args_to(&self, consumer: &mut impl ArgConsumer) -> ToKtxResult<()>;
}

macro_rules! impl_input_source {
    (slice) => {
        #[inline]
        fn add_args_to(&self, consumer: &mut impl ArgConsumer) -> ToKtxResult<()> {
            <[u8]>::add_args_to(self, consumer)
        }
    };
    (slice > $($src: ty),+) => {
        $(
            impl InputSource for $src {
                impl_input_source!(slice);
            }
        )+
    };
    (borrowed_body => $borrowed: ty) => {
        #[inline]
        fn add_args_to(&self, consumer: &mut impl ArgConsumer) -> ToKtxResult<()> {
            <$borrowed>::add_args_to(self, consumer)
        }
    };
    (borrow > $([$owned: ty => $borrowed: ty]),+) => {
        $(
            impl InputSource for $borrowed {
                #[inline]
                fn add_args_to(&self, consumer: &mut impl ArgConsumer) -> ToKtxResult<()> {
                    consumer.add_arg(self);
                    Ok(())
                }
            }
            impl InputSource for $owned {
                impl_input_source!(borrowed_body => $borrowed);
            }

            impl InputSource for &$owned {
                impl_input_source!(borrowed_body => $borrowed);
            }

            impl InputSource for &$borrowed {
                impl_input_source!(borrowed_body => $borrowed);
            }
        )+
    };
}

impl_input_source!(borrow > [PathBuf => Path], [String => str], [OsString => OsStr]);
impl_input_source!(slice > Vec<u8>);

impl InputSource for [u8] {
    fn add_args_to(&self, consumer: &mut impl ArgConsumer) -> ToKtxResult<()> {
        let path = temp_path();

        if let Err(e) = fs::write(path.as_path(), self) {
            return Err(ToKtxError::SourcePath(e));
        }

        consumer.add_arg(path);

        Ok(())
    }
}

impl<const N: usize> InputSource for [u8; N] {
    impl_input_source!(slice);
}
