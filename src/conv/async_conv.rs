use std::ffi::OsStr;
use std::future::Future;
use std::marker::PhantomData;
use std::path::Path;
use std::process::Stdio;

use crate::{InputSource, ToKtxResult};
use crate::conv::OutputExt;

use super::{CommandAsync, PATH_STDOUT, ToKtxConvert};

#[derive(Debug, Copy, Clone)]
pub struct ToKtxConvertAsync<'c, 'i, I: 'i, A> {
    base: ToKtxConvert<'c, 'i, I>,
    _cmd: PhantomData<A>,
}

impl<'c, 'i, I: InputSource + 'i, A: CommandAsync> ToKtxConvertAsync<'c, 'i, I, A> {
    #[inline]
    pub(crate) const fn new(base: ToKtxConvert<'c, 'i, I>) -> Self {
        Self {
            base,
            _cmd: PhantomData,
        }
    }

    /// Get output as a byte vector
    pub fn to_memory(self) -> impl Future<Output = ToKtxResult<Vec<u8>>> {
        process_command(self.cmd(OsStr::new(PATH_STDOUT)), Stdio::piped())
    }

    /// Write output to the given path
    #[inline]
    pub fn to_path(self, path: impl AsRef<Path>) -> impl Future<Output = ToKtxResult<()>> {
        self.to_path_inner(path.as_ref().as_os_str())
    }

    fn to_path_inner(self, path: &OsStr) -> impl Future<Output = ToKtxResult<()>> {
        let command = self.cmd(path);
        async move {
            process_command(command, Stdio::null()).await?;
            Ok(())
        }
    }

    #[inline]
    fn cmd(self, out_path: &OsStr) -> ToKtxResult<A> {
        self.base.cmd(out_path)
    }
}

async fn process_command<A: CommandAsync>(result: ToKtxResult<A>, stdout: Stdio) -> ToKtxResult<Vec<u8>> {
    let mut command = result?;
    command.set_stdout(stdout);
    command.get_output().await.format_output()
}

impl<'c, 'i, I: InputSource + 'i> ToKtxConvert<'c, 'i, I> {
    /// Spawn a non-blocking async [`Command`].
    ///
    /// See [`tokio`](ToKtxConvert::tokio) and [`async_std`](ToKtxConvert::async_std)
    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
    pub const fn future<A: CommandAsync>(self) -> ToKtxConvertAsync<'c, 'i, I, A> {
        ToKtxConvertAsync::new(self)
    }

    /// Spawn a non-blocking async [`Command`](tokio::process::Command) using [`tokio`].
    ///
    /// Equivalent to calling [`future::<tokio::process::Command>()`](ToKtxConvert::future).
    #[inline]
    #[cfg(feature = "tokio")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
    pub const fn tokio(self) -> ToKtxConvertAsync<'c, 'i, I, tokio::process::Command> {
        self.future()
    }

    /// Spawn a non-blocking async [`Command`](async_std::process::Command) using
    /// [`async-std`](async_std).
    ///
    /// Equivalent to calling [`future::<async_std::process::Command>()`](ToKtxConvert::future).
    #[inline]
    #[cfg(feature = "async-std")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
    pub const fn async_std(self) -> ToKtxConvertAsync<'c, 'i, I, async_std::process::Command> {
        self.future()
    }
}

#[cfg(test)]
mod test {
    use crate::temp_path;

    use super::super::CommandAsync;
    use super::super::test::*;

    async fn run<T: CommandAsync>() {
        let temp_path = temp_path();

        CFG.convert(SRC).future::<T>().to_path(&temp_path).await.expect("to_path write");


        let to_path = tokio::fs::read(temp_path).await.expect("to_path read");
        let to_mem = CFG.convert(SRC).future::<T>().to_memory().await.expect("to_mem");

        assert_eq!(to_path, to_mem, "to_path != to_mem");

        drop(to_path);

        if let Ok(sync) = MEMORY_CONV.as_ref() {
            assert_eq!(sync.as_slice(), to_mem.as_slice(), "sync != to_mem");
        }
    }

    #[tokio::test]
    async fn tokio() {
        run::<tokio::process::Command>().await;
    }

    #[tokio::test]
    async fn async_std() {
        run::<async_std::process::Command>().await;
    }
}
