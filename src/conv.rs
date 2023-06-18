use std::ffi::OsStr;
use std::io;
use std::marker::PhantomData;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub use command::*;
pub use error::ToKtxError;
pub use input_source::InputSource;

use crate::prelude::*;
use crate::ToKtx;

mod input_source;
mod command;

const PATH_STDOUT: &str = "-";

cfg_if! {
    if #[cfg(feature = "async")] {
        pub use async_conv::ToKtxConvertAsync;
        #[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
        mod async_conv;
    }
}
mod error;

/// A [`Result`] with [`ToKtxError`] as the error type
pub type ToKtxResult<T> = Result<T, ToKtxError>;

impl ToKtx {
    /// Convert the given input to KTX.
    ///
    /// The input can be a path to a file or a `&[u8]`
    #[inline]
    pub fn convert<'c, 'i, I>(&'c self, source: I) -> ToKtxConvert<'c, 'i, I>
    where I: InputSource + 'i {
        ToKtxConvert::new(self, source)
    }
}

trait OutputExt {
    fn format_output(self) -> ToKtxResult<Vec<u8>>;
}

impl OutputExt for io::Result<Output> {
    fn format_output(self) -> ToKtxResult<Vec<u8>> {
        let output = self.map_err(ToKtxError::Spawn)?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(ToKtxError::ExitStatus {
                status: output.status,
                stderr: output.stderr,
            })
        }
    }
}

/// Tmp builder state for [`ToKtx::convert`]
#[derive(Debug, Copy, Clone)]
pub struct ToKtxConvert<'c, 'i, I: 'i> {
    config: &'c ToKtx,
    input: I,
    _marker: PhantomData<&'i ()>,
}

impl<'c, 'i, I: InputSource + 'i> ToKtxConvert<'c, 'i, I> {
    const fn new(config: &'c ToKtx, input: I) -> Self {
        Self {
            config,
            input,
            _marker: PhantomData,
        }
    }

    /// Get output as a byte vector
    pub fn to_memory(self) -> ToKtxResult<Vec<u8>> {
        self.to_memory_with_cmd::<Command>()
    }

    /// Get output as a byte vector using the given blocking variation of [`Command`].
    pub fn to_memory_with_cmd<B: CommandBlocking>(self) -> ToKtxResult<Vec<u8>> {
        self.cmd::<B>(OsStr::new(PATH_STDOUT))?
            .set_stdout(Stdio::piped())
            .get_output()
            .format_output()
    }

    /// Write output to the given path
    #[inline]
    pub fn to_path(self, path: impl AsRef<Path>) -> ToKtxResult<()> {
        self.to_path_with_cmd::<Command>(path)
    }

    /// Write output to the given path using the given blocking variation of [`Command`].
    #[inline]
    pub fn to_path_with_cmd<B: CommandBlocking>(self, path: impl AsRef<Path>) -> ToKtxResult<()> {
        self.to_path_inner::<B>(path.as_ref().as_os_str())
    }

    fn to_path_inner<B: CommandBlocking>(self, path: &OsStr) -> ToKtxResult<()> {
        self.cmd::<B>(path)?
            .set_stdout(Stdio::null())
            .get_output()
            .format_output()?;

        Ok(())
    }

    pub(super) fn cmd<B: CommandLike>(self, out_path: &OsStr) -> ToKtxResult<B> {
        let mut command = {
            let cmd_path = if let Some(ref p) = self.config.path_to_toktx {
                p.as_os_str()
            } else {
                OsStr::new("toktx")
            };
            B::construct(cmd_path)
        };
        command
            .init_stdio(Stdio::null(), Stdio::piped())
            .add_arg_set(self.config)
            .add_arg(out_path);

        self.input.add_args_to(&mut command)?;

        Ok(command)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::fs;
    use once_cell::sync::Lazy;

    use crate::{enc, OutputFormat, temp_path, ToKtx, ToKtxResult, TransferFunction};
    use crate::enc::{UASTCOptions, UASTCQuality};
    use crate::prelude::*;

    pub const SRC: &str = "test/source.png";
    pub static CFG: Lazy<ToKtx> = Lazy::new(move || {
        ToKtx {
            two_dee: true,
            output_format: OutputFormat::KTX2,
            encoding: Some(enc::Encoding::UASTC(UASTCOptions {
                quality: Some(UASTCQuality::Fastest),
                ..UASTCOptions::DEFAULT
            })),
            assign_oetf: Some(TransferFunction::Srgb),
            ..ToKtx::DEFAULT
        }
    });
    pub static MEMORY_CONV: Lazy<ToKtxResult<Vec<u8>>> = Lazy::new(move || {
        CFG.convert(SRC).to_memory()
    });

    #[test]
    fn cmp_path_and_mem_output() {
        let temp_path = temp_path();

        CFG.convert(SRC).to_path(&temp_path).expect("to_path write");

        let to_path = fs::read(temp_path).expect("to_path read");
        let to_mem = MEMORY_CONV.as_ref().expect("to_mem read");

        assert_eq!(to_path.as_slice(), to_mem.as_slice(), "Comparison");
    }
}
