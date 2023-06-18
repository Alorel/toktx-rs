use std::ffi::OsStr;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::process::{Command, Output, Stdio};

use crate::prelude::*;

pub trait CommandLike: ArgConsumer {
    fn construct(program: &OsStr) -> Self;

    fn init_stdio(&mut self, stdin: Stdio, stderr: Stdio) -> &mut Self;

    fn set_stdout(&mut self, stdout: Stdio) -> &mut Self;
}

pub trait CommandBlocking: CommandLike {
    fn get_output(&mut self) -> Result<Output, io::Error>;
}

/// A non-blocking async [`Command`].
#[cfg(feature = "async")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
pub trait CommandAsync: CommandLike {
    fn get_output(&mut self) -> Pin<Box<dyn Future<Output = Result<Output, io::Error>>>>;
}

impl CommandBlocking for Command {
    #[inline]
    fn get_output(&mut self) -> Result<Output, io::Error> {
        self.output()
    }
}

macro_rules! impl_cmd {
    ($namespace: ident) => {
        impl CommandLike for ::$namespace::process::Command {
            #[inline]
            fn construct(program: &OsStr) -> Self {
                Self::new(program)
            }

            fn init_stdio(&mut self, stdin: Stdio, stderr: Stdio) -> &mut Self {
                self.stdin(stdin).stderr(stderr)
            }

            #[inline]
            fn set_stdout(&mut self, stdout: Stdio) -> &mut Self {
                self.stdout(stdout)
            }
        }
    };
    (async $namespace: ident) => {
        impl_cmd!($namespace);

        impl CommandAsync for ::$namespace::process::Command {
            #[inline]
            fn get_output(&mut self) -> Pin<Box<dyn Future<Output = Result<Output, io::Error>>>> {
                Box::pin(self.output())
            }
        }
    };
}

impl_cmd!(std);

#[cfg(feature = "tokio")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "tokio")))]
impl_cmd!(async tokio);

#[cfg(feature = "async-std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async-std")))]
impl_cmd!(async async_std);
